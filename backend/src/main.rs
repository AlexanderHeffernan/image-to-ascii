mod converter;

use rusty_api;
use actix_multipart::Multipart;
use futures_util::StreamExt as _;

async fn convert_image_route(mut payload: Multipart) -> rusty_api::HttpResponse {
    use bytes::BytesMut;

    let mut image_bytes = BytesMut::new();
    
    // Iterate over multipart fields
    while let Some(item) = payload.next().await {
        println!("Processing multipart item...");
        let mut field = match item {
            Ok(f) => f,
            Err(e) => return rusty_api::HttpResponse::BadRequest().body(format!("Multipart error: {e}")),
        };

        // Only process the "image" field
        if field.name() == "image" {
            while let Some(chunk) = field.next().await {
                let data = match chunk {
                    Ok(d) => d,
                    Err(e) => return rusty_api::HttpResponse::InternalServerError().body(format!("Read error: {e}")),
                };
                image_bytes.extend_from_slice(&data);
            }
        }
    }

    let output_width: u32 = 200;
    let brightness_factor = 1.5;
    let use_background = false;

    match converter::Converter::convert_from_bytes(&image_bytes, output_width, brightness_factor, use_background) {
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