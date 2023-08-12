#[cfg(test)]
mod test {
    use actix_web::{test, App}; 
    use super::super::*; // Import modules from urls/mod.rs
    #[actix_web::test]
    async fn list_urls_should_return_ok() {
        let mut app = test::init_service(App::new().configure(service)).await;

        // Make a test request and assert the response
        let request = test::TestRequest::get().uri("/urls/").to_request();
        let response = test::call_service(&mut app, request).await;
        assert!(response.status().is_success());
        let body: [Url; 3] = test::read_body_json(response).await; 
        assert_eq!(body[0].url, "https://www.google.com");
    }
}
