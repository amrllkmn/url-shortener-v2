use actix_web::{App, HttpServer, get, Responder, HttpResponse};

mod urls;

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("OK")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .configure(urls::service)
        .service(healthcheck)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}