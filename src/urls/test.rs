#[cfg(test)]
mod test {
    use actix_web::{test, App}; 
    use super::super::service;// Import modules from urls/mod.rs
    #[actix_web::test]
    async fn test_your_service() {
        let mut app = test::init_service(App::new().configure(service)).await;

        // Make a test request and assert the response
        let request = test::TestRequest::get().uri("/urls/").to_request();
        let response = test::call_service(&mut app, request).await;

        assert!(response.status().is_success());  // Adjust your assertion as needed
    }
}
