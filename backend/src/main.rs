mod converter;

use rusty_api;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use serde::Deserialize;
use bytes::BytesMut;

// Import the types from the converter module
use converter::{Converter, ConverterConfig};

/// Parses the multipart payload, extracting the image and config JSON (if present).
async fn parse_multipart(mut payload: Multipart) -> Result<(BytesMut, Option<BytesMut>), rusty_api::HttpResponse> {
    let mut image_bytes = BytesMut::new();
    let mut config_json = None;

    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => return Err(rusty_api::HttpResponse::BadRequest().body(format!("Multipart error: {e}"))),
        };

        match field.name() {
            "image" => {
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(d) => d,
                        Err(e) => return Err(rusty_api::HttpResponse::InternalServerError().body(format!("Read error: {e}"))),
                    };
                    image_bytes.extend_from_slice(&data);
                }
            }
            "config" => {
                let mut config_bytes = BytesMut::new();
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(d) => d,
                        Err(e) => return Err(rusty_api::HttpResponse::InternalServerError().body(format!("Config read error: {e}"))),
                    };
                    config_bytes.extend_from_slice(&data);
                }
                config_json = Some(config_bytes);
            }
            _ => {
                return Err(rusty_api::HttpResponse::BadRequest()
                    .body(format!("Unexpected field: {}", field.name())));
            }
        }
    }

    Ok((image_bytes, config_json))
}

/// Main route handler for image-to-ASCII conversion.
/// Accepts multipart form-data with "image" and optional "config" fields.
async fn convert_image_route(mut payload: Multipart) -> rusty_api::HttpResponse {
    // Parse multipart payload
    let (image_bytes, config_json) = match parse_multipart(payload).await {
        Ok(res) => res,
        Err(resp) => return resp,
    };

    // Parse config JSON or use defaults
    let config: converter::ConverterConfig = match config_json {
        Some(json_bytes) => match serde_json::from_slice(&json_bytes) {
            Ok(cfg) => cfg,
            Err(e) => return rusty_api::HttpResponse::BadRequest().body(format!("Invalid config JSON: {}", e)),
        },
        None => return rusty_api::HttpResponse::BadRequest().body("Missing config JSON"),
    };

    // Convert image and return ASCII grid as JSON
    match converter::Converter::convert_from_bytes(&image_bytes, config) {
        Ok(ascii_grid) => {
            match serde_json::to_string(&ascii_grid) {
                Ok(json) => rusty_api::HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json),
                Err(e) => rusty_api::HttpResponse::InternalServerError()
                    .body(format!("Serialization error: {}", e)),
            }
        }
        Err(e) => rusty_api::HttpResponse::InternalServerError()
            .body(format!("Conversion error: {}", e)),
    }
}

/// Entrypoint: sets up API routes, TLS, CORS, and starts the server.
fn main() {
    let routes = rusty_api::Routes::new()
        .add_route(rusty_api::Method::POST, "/convert-image", convert_image_route);

    rusty_api::Api::new()
        .certs("certs/cert.pem", "certs/key.pem")
        .rate_limit(3, 20)
        .bind("0.0.0.0", 8444)
        .configure_routes(routes)
        .configure_cors(|| {
            rusty_api::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allowed_header("ngrok-skip-browser-warning")
        })
        .start();
}