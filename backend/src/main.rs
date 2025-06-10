mod converter;

use rusty_api;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use serde::Deserialize;
use bytes::BytesMut;
use simplelog::*;
use std::fs::{File, OpenOptions};
use log::{info, error};
use chrono::Utc;

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
    struct RequestLogger {
        request_id: i64,
    }

    impl Drop for RequestLogger {
        fn drop(&mut self) {
            info!("------------------- [Request {}] End -------------------", self.request_id);
        }
    }

    let request_id = Utc::now().timestamp_millis();
    let _guard = RequestLogger { request_id };
    info!("------------------- [Request {}] -------------------", request_id);
    info!("[{}] Received request to /convert-image", request_id);

    // Parse multipart payload
    let (image_bytes, config_json) = match parse_multipart(payload).await {
        Ok(res) => res,
        Err(resp) => {
            error!("[{}] Failed to parse multipart payload", request_id);
            return resp;
        },
    };

    // Parse config JSON or use defaults
    let config: converter::ConverterConfig = match config_json {
        Some(json_bytes) => match serde_json::from_slice(&json_bytes) {
            Ok(cfg) => {
                // Log the config details as JSON
                match serde_json::to_string(&cfg) {
                    Ok(cfg_str) => info!("[{}] Config received: {}", request_id, cfg_str),
                    Err(e) => error!("[{}] Failed to serialize config for logging: {}", request_id, e),
                }
                cfg
            },
            Err(e) => {
                error!("[{}] Invalid config JSON: {}", request_id, e);
                return rusty_api::HttpResponse::BadRequest().body(format!("Invalid config JSON: {}", e));
            },
        },
        None => {
            error!("[{}] Missing config JSON", request_id);
            return rusty_api::HttpResponse::BadRequest().body("Missing config JSON");
        },
    };

    // Convert image and return ASCII grid as JSON
    match converter::Converter::convert_from_bytes(&image_bytes, config) {
        Ok(ascii_grid) => {
            info!("[{}] Image converted successfully", request_id);
            match serde_json::to_string(&ascii_grid) {
                Ok(json) => rusty_api::HttpResponse::Ok()
                    .content_type("application/json")
                    .body(json),
                Err(e) => {
                    error!("[{}] Serialization error: {}", request_id, e);
                    return rusty_api::HttpResponse::InternalServerError()
                        .body(format!("Serialization error: {}", e));
                },
            }
        }
        Err(e) => {
            error!("[{}] Conversion error: {}", request_id, e);
            return rusty_api::HttpResponse::InternalServerError()
                .body(format!("Conversion error: {}", e));
        },
    }
}

/// Entrypoint: sets up API routes, TLS, CORS, and starts the server.
fn main() {
    // Initialize logging
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            OpenOptions::new()
                .create(true)
                .append(true)
                .open("backend.log")
                .unwrap(),
        ),
    ]).unwrap();

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