use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct Url {
    id: i32,
    url: String,
    slug: String,
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
    Ok(HttpResponse::Ok().json(list))
}

pub fn service(cfg: &mut web::ServiceConfig) {
    cfg.service(
            // prefixes all resources and routes attached to it...
            web::scope("/urls")
                // ...so this handles requests for `GET /app/index.html`
                .route("/", web::get().to(list_urls))
    );
}