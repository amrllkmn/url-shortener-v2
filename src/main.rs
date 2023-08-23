use actix_web::{App, HttpServer, get, Responder, HttpResponse, web};
use sea_orm::{DatabaseConnection, Database};

mod urls;
mod entities;
#[derive(Debug, Clone)]
struct AppState {
    conn: DatabaseConnection,
}

#[get("/healthcheck")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("OK")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_url= String::from("postgres://postgres:postgres@localhost:5434/url_shortener_dev"); // Hardcoding it cos for now it's a docker db server
    let db_conn = Database::connect(&db_url).await.unwrap();
    let state = AppState {
        conn: db_conn
    };
    
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(state.clone()))
        .configure(urls::service)
        .service(healthcheck)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}