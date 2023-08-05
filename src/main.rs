use actix_web::{web, App, HttpResponse, HttpServer, Responder};

/// ### GET /
/// Returns a "Hello world!" response.
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

/// ### POST /echo
/// Returns the request body as a response.
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}


/// ### GET /hey
/// Returns a "Hey there!" response.
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(
            // prefixes all resources and routes attached to it...
            web::scope("/api/v1")
                // ...so this handles requests for `GET /app/index.html`
                .route("/", web::get().to(hello))
                .route("/hey", web::get().to(manual_hello))
                .route("/echo", web::post().to(echo)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}