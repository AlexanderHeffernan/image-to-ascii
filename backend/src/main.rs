mod converter;

use rusty_api;

async fn convert_image_route(_req: rusty_api::HttpRequest) -> rusty_api::HttpResponse {
    let img_path = "images/image.png";
    let output_width: u32 = 200;
    let brightness_factor = 1.5;
    let use_background = false;

    match converter::Converter::convert(img_path, output_width, brightness_factor, use_background) {
        Ok(ascii_grid) => {
            let json = serde_json::to_string(&ascii_grid).unwrap();
            rusty_api::HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
        Err(e) => rusty_api::HttpResponse::InternalServerError()
            .body(format!("Error: {}", e)),
    }
}

fn main() {
    let routes = rusty_api::Routes::new()
        .add_route("/convert-image", convert_image_route);

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