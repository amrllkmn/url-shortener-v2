use actix_web::{App, HttpServer};

mod api_v1;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().configure(api_v1::api_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}