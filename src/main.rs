use actix_web::{App, HttpServer, get, Responder, HttpResponse};

mod api_v1;

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("OK")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(api_v1::api_service)
        .service(healthcheck)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}