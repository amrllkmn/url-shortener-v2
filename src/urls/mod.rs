use super::AppState;
use actix_web::{web, HttpResponse, Result};
use serde::Serialize;

use super::entities::{url, Mutation, Query};

#[derive(Serialize)]
struct CreatedResponse<'a> {
    message: &'a str,
    data: url::Model,
}

#[derive(Serialize)]
struct InternalErrorResponse<'a> {
    message: &'a str,
}
#[derive(Serialize)]
struct OkResponse {
    data: Vec<url::Model>,
}
/// ### GET urls/
/// Returns a list of URLS shortened.
async fn list_urls(data: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    println!("GET /urls/");
    let conn = &data.conn;
    match Query::find_all(conn).await {
        Ok(urls) => Ok(HttpResponse::Ok().json(OkResponse { data: urls })),
        Err(err) => {
            // Log the error or handle it as needed
            println!("Database error: {:?}", err);

            // You can customize the error response based on the error type
            let message = "Internal Server Error";

            Ok(HttpResponse::InternalServerError().json(InternalErrorResponse { message }))
        }
    }
}

async fn create_url(
    data: web::Data<AppState>,
    url_form: web::Json<url::Model>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("POST /urls/");
    let conn = &data.conn;
    let form = url_form.into_inner();

    match Mutation::create(conn, form).await {
        Ok(url) => Ok(HttpResponse::Created().json(CreatedResponse {
            message: "URL Created.",
            data: url,
        })),
        Err(err) => {
            // Log the error or handle it as needed
            println!("Database error: {:?}", err);

            // You can customize the error response based on the error type
            let message = "Internal Server Error";

            Ok(HttpResponse::InternalServerError().json(InternalErrorResponse { message }))
        }
    }
}

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        // prefixes all resources and routes attached to it...
        web::scope("/urls")
            // ...so this handles requests for `GET /urls/`
            .route("/", web::get().to(list_urls))
            .route("/", web::post().to(create_url)),
    );
}
