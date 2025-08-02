use actix_web::{App, HttpServer, web};

mod handlers;
mod token_generator;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route(
                "/vulnerable/forgot-password",
                web::post().to(handlers::vulnerable::forgot_password),
            )
            .route(
                "/secure/forgot-password",
                web::post().to(handlers::secure::forgot_password),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
