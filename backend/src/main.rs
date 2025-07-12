mod converter;
mod request_logger;
mod compressor;

use rusty_api;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;
use bytes::BytesMut;

// Import the types from the converter module
use converter::{Converter, ConverterConfig};
use request_logger::RequestLogger;

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
async fn convert_image_route(payload: Multipart) -> rusty_api::HttpResponse {
    // Generate a request ID for logging
    let request_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    
    let logger = RequestLogger::new(request_id);
    logger.info("Processing image conversion request");

    // Parse multipart payload
    let (image_bytes, config_json) = match parse_multipart(payload).await {
        Ok((img, cfg)) => (img, cfg),
        Err(response) => return response,
    };

    if image_bytes.is_empty() {
        logger.error("No image data provided");
        return rusty_api::HttpResponse::BadRequest().body("No image data provided");
    }

    // Parse config if provided, otherwise use default
    let config = if let Some(config_bytes) = config_json {
        match serde_json::from_slice::<ConverterConfig>(&config_bytes) {
            Ok(cfg) => cfg,
            Err(e) => {
                logger.error(format!("Invalid config JSON: {}", e));
                return rusty_api::HttpResponse::BadRequest().body(format!("Invalid config JSON: {}", e));
            }
        }
    } else {
        ConverterConfig::default()
    };

    // Convert image and optionally compress
    match Converter::convert_from_bytes(&image_bytes, config) {
        Ok(ascii_grid) => {
            logger.info("Image converted successfully");
            
            // Always use compression for now
            let use_compression = true;
            
            if use_compression {
                // Send compressed data
                match compressor::compress_ascii_grid(&ascii_grid) {
                    Ok(compressed) => {
                        let original_size = serde_json::to_string(&ascii_grid).unwrap_or_default().len();
                        let compressed_size = compressed.len();
                        let ratio = compressed_size as f64 / original_size as f64;
                        logger.info(format!("Compression ratio: {:.2}% ({} -> {} bytes)", 
                            ratio * 100.0, original_size, compressed_size));
                        
                        // Create a custom header string to include compression info
                        let compression_header = format!("rle-gzip;original={};compressed={};ratio={:.2}", 
                            original_size, compressed_size, ratio);
                        
                        rusty_api::HttpResponse::Ok()
                            .content_type("application/octet-stream")
                            .insert_header(("X-Compression", compression_header.as_str()))
                            .insert_header(("X-Original-Size", original_size.to_string().as_str()))
                            .insert_header(("X-Compressed-Size", compressed_size.to_string().as_str()))
                            .body(compressed)
                    },
                    Err(e) => {
                        logger.error(format!("Compression failed: {}", e));
                        // Fall back to uncompressed
                        match serde_json::to_string(&ascii_grid) {
                            Ok(json) => rusty_api::HttpResponse::Ok()
                                .content_type("application/json")
                                .body(json),
                            Err(e) => rusty_api::HttpResponse::InternalServerError()
                                .body(format!("Serialization failed: {}", e)),
                        }
                    }
                }
            } else {
                // Send uncompressed JSON
                match serde_json::to_string(&ascii_grid) {
                    Ok(json) => rusty_api::HttpResponse::Ok()
                        .content_type("application/json")
                        .body(json),
                    Err(e) => rusty_api::HttpResponse::InternalServerError()
                        .body(format!("Serialization failed: {}", e)),
                }
            }
        },
        Err(e) => {
            logger.error(format!("Image conversion failed: {}", e));
            rusty_api::HttpResponse::InternalServerError()
                .body(format!("Image conversion failed: {}", e))
        },
    }
}

/// Entrypoint: sets up API routes, TLS, CORS, and starts the server.
fn main() {
    let routes = rusty_api::Routes::new()
        .add_route(rusty_api::Method::POST, "/convert-image", convert_image_route);

    rusty_api::Api::new()
        .certs("certs/cert.pem", "certs/key.pem")
        .rate_limit(3, 20)
        .bind("0.0.0.0", 49162)
        .configure_routes(routes)
        .configure_cors(|| {
            rusty_api::Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allowed_header("ngrok-skip-browser-warning")
                .allowed_header("X-Compression")
                .allowed_header("X-Original-Size")
                .allowed_header("X-Compressed-Size")
                .expose_headers([
                    "X-Compression",
                    "X-Original-Size", 
                    "X-Compressed-Size"
                ])
        })
        .start();
}