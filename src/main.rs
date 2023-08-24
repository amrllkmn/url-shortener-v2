use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

mod entities;
mod urls;
mod utils;
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
    let db_url = String::from("postgres://postgres:postgres@localhost:5434/url_shortener_dev"); // Hardcoding it cos for now it's a docker db server
    let db_conn = Database::connect(&db_url).await.unwrap();

    // Run migrations before connecting to database
    Migrator::up(&db_conn, None).await.unwrap();
    let state = AppState { conn: db_conn };

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
