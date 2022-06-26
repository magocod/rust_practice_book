use actix_web::{http, test, App};
mod common;

#[actix_web::test]
async fn successful_http_request() {
    let (scope, _) = common::setup();
    let app = test::init_service(App::new().service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/http_request")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn successful_reuse_http_request() {
    let (scope, _) = common::setup();
    let app = test::init_service(App::new().service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/reuse_http_request")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}