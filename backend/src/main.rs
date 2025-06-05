mod converter;

use rusty_api;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use serde::Deserialize;

#[derive(Deserialize)]
struct ConverterConfigInput {
    character_set: Option<Vec<char>>,
    output_width: Option<u32>,
    output_height: Option<u32>,
    brightness_factor: Option<f32>,
    contrast_factor: Option<f32>,
    is_color: Option<bool>,
    aspect_ratio_correction: Option<f32>,
}

async fn convert_image_route(mut payload: Multipart) -> rusty_api::HttpResponse {
    use bytes::BytesMut;

    let mut image_bytes = BytesMut::new();
    let mut config_json = None;
    
    // Iterate over multipart fields
    while let Some(item) = payload.next().await {
        let mut field = match item {
            Ok(f) => f,
            Err(e) => return rusty_api::HttpResponse::BadRequest().body(format!("Multipart error: {e}")),
        };

        match field.name() {
            "image" => {
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(d) => d,
                        Err(e) => return rusty_api::HttpResponse::InternalServerError().body(format!("Read error: {e}")),
                    };
                    image_bytes.extend_from_slice(&data);
                }
            }
            "config" => {
                let mut config_bytes = BytesMut::new();
                while let Some(chunk) = field.next().await {
                    let data = match chunk {
                        Ok(d) => d,
                        Err(e) => return rusty_api::HttpResponse::InternalServerError().body(format!("Config read error: {e}")),
                    };
                    config_bytes.extend_from_slice(&data);
                }
                config_json = Some(config_bytes);
            }
            _ => {
                return rusty_api::HttpResponse::BadRequest()
                    .body(format!("Unexpected field: {}", field.name()));
            }
        }
    }

    // Parse config JSON or use defaults
    let config: converter::ConverterConfig = match config_json {
        Some(json_bytes) => {
            let input: ConverterConfigInput = match serde_json::from_slice(&json_bytes) {
                Ok(input) => input,
                Err(e) => return rusty_api::HttpResponse::BadRequest().body(format!("Invalid config JSON: {}", e)),
            };
            converter::ConverterConfig {
                character_set: input.character_set.unwrap_or_else(|| converter::Converter::DEFAULT_CHARS.to_vec()),
                output_width: input.output_width.unwrap_or(200),
                output_height: input.output_height,
                brightness_factor: input.brightness_factor.unwrap_or(1.0),
                contrast_factor: input.contrast_factor.unwrap_or(1.0),
                is_color: input.is_color.unwrap_or(false),
                aspect_ratio_correction: input.aspect_ratio_correction.unwrap_or(0.55),
            }
        }
        None => converter::ConverterConfig {
            character_set: converter::Converter::DEFAULT_CHARS.to_vec(),
            output_width: 200,
            output_height: None,
            brightness_factor: 1.0,
            contrast_factor: 1.0,
            is_color: false,
            aspect_ratio_correction: 0.55,
        }
    };

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