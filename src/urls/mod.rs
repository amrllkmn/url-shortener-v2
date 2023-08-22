use actix_web::{web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use sea_orm::DatabaseConnection;

use super::entities::{url, Mutation};
#[derive(Serialize, Deserialize, Debug)]
struct Url {
    id: i32,
    url: String,
    slug: String,
}
struct AppState {
    conn: DatabaseConnection,
}
/// ### GET urls/
/// Returns a list of URLS shortened.
async fn list_urls() -> Result<impl Responder, actix_web::Error> {
    println!("GET /urls");
    let list: [Url; 3] = [
        Url {
            id: 1,
            url: "https://www.google.com".to_string(),
            slug: "google".to_string(),
        },
        Url {
            id: 2,
            url: "https://www.youtube.com".to_string(),
            slug: "youtube".to_string(),
        },
        Url {
            id: 3,
            url: "https://www.facebook.com".to_string(),
            slug: "facebook".to_string(),
        }
    ];

    Ok(web::Json(list)) // We don't have proper error handling as we are not using a database yet.
}

async fn create_url(data: web::Data<AppState>, url_form: web::Form<url::Model>) -> Result<HttpResponse, actix_web::Error> {
    let conn = &data.conn;
    let form = url_form.into_inner();

    Mutation::create(conn, form)
    .await
    .expect("Could not create a URL");


    Ok(HttpResponse::Created().body("URL Created."))
}

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
            // prefixes all resources and routes attached to it...
            web::scope("/urls")
                // ...so this handles requests for `GET /urls/`
                .route("/", web::get().to(list_urls))
                .route("/", web::post().to(create_url))
    );
}

#[cfg(test)]
mod test;