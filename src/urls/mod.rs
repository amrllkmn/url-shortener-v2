use super::AppState;
use actix_web::{
    http::header,
    web::{self},
    HttpResponse, Result,
};
use serde::Serialize;

use entities::url;

use service::{Mutation, Query};

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
struct NotFoundErrorResponse<'a> {
    message: &'a str,
}
#[derive(Serialize)]
struct OkResponse<T> {
    data: T,
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

async fn redirect_url(
    data: web::Data<AppState>,
    slug: web::Path<String>,
) -> Result<HttpResponse, actix_web::Error> {
    println!("GET /urls/:slug");
    let conn = &data.conn;
    let slug = slug.into_inner();
    match Query::find_by_slug(conn, slug).await {
        Ok(db_transaction) => {
            if let Some(url) = db_transaction {
                let redirected_url = url.url;
                let header_value = header::HeaderValue::from_str(redirected_url.as_str())
                    .expect("The url isn't a string then.");
                Ok(HttpResponse::TemporaryRedirect()
                    .append_header((header::LOCATION, header_value))
                    .finish())
            } else {
                Ok(HttpResponse::NotFound().json(NotFoundErrorResponse {
                    message: "URL not found.",
                }))
            }
        }
        Err(err) => {
            // Log the error or handle it as needed
            println!("Database error: {:?}", err);

            // You can customize the error response based on the error type
            let message = "Internal Server Error";

            Ok(HttpResponse::InternalServerError().json(InternalErrorResponse { message }))
        }
    }
}

async fn delete_url(
    data: web::Data<AppState>,
    id: web::Path<i32>,
) -> Result<HttpResponse, actix_web::Error> {
    let conn = &data.conn;
    let id = id.into_inner();

    match Mutation::delete(conn, id).await {
        Ok(_) => {
            println!("URL Deleted.");
            Ok(HttpResponse::Ok().json(OkResponse {
                data: "URL Deleted.",
            }))
        }
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
            .route("/", web::post().to(create_url))
            .route("/{slug}", web::get().to(redirect_url))
            .route("/{id}", web::delete().to(delete_url)),
    );
}
