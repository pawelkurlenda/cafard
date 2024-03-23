/*#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use super::*;

    #[actix_web::test]
    async fn test_index_get() {
        let app = test::init_service(App::new().service(index)).await;
        let req = test::TestRequest::default()
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}*/

use actix_web::{http::StatusCode, App, test};
use api_lib::health::{service, API_VERSION};

#[actix_rt::test]
async fn health_check_works() {
    let app = App::new().configure(service);
    let mut app = actix_web::test::init_service(app).await;
    let req = actix_web::test::TestRequest::get()
        .uri("/health")
        .to_request();

    let res = actix_web::test::call_service(&mut app, req).await;

    assert!(res.status().is_success());
    assert_eq!(res.status(), StatusCode::OK);
    let data = res.headers().get("version").and_then(|h| h.to_str().ok());
    assert_eq!(data, Some(API_VERSION));
}