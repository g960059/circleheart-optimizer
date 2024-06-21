mod model;
mod api;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    
    println!("Starting server on port {}", port);

    HttpServer::new(|| {
        App::new()
            .configure(api::routes::config)
    })
    .bind(format!("0.0.0.0:{}", port))?
    .run()
    .await
}