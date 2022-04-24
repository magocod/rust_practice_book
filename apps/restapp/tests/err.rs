use actix_web::{http, test, App};

#[actix_web::test]
async fn returns_the_error_in_plain_text() {
    let app = test::init_service(App::new().service(restapp::err::custom_error)).await;
    let req = test::TestRequest::get().uri("/custom_error").to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn return_the_error_in_json_text() {
    let app = test::init_service(App::new().service(restapp::err::custom_error_json)).await;
    let req = test::TestRequest::get()
        .uri("/custom_error_json")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::INTERNAL_SERVER_ERROR);
}

#[actix_web::test]
async fn return_error_with_helper() {
    let app = test::init_service(App::new().service(restapp::err::error_with_helper)).await;
    let req = test::TestRequest::get()
        .uri("/error_with_helper")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}
