use actix_web::{http, test, App};
use commons::jwt::generate_token;
use commons::mongo::User;
mod common;

#[actix_web::test]
async fn get_current_user() {
    let (scope, _) = common::setup();
    let app = test::init_service(App::new().service(scope)).await;

    let tk = generate_token(&User::factory()).expect("error generate token");

    let req = test::TestRequest::get()
        .uri("/api/jwt")
        .insert_header((http::header::AUTHORIZATION, format!("bearer {}", tk)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn no_authentication_header() {
    let (scope, _) = common::setup();
    let app = test::init_service(App::new().service(scope)).await;

    let req = test::TestRequest::get().uri("/api/jwt").to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn incomplete_authentication_header() {
    let (scope, _) = common::setup();
    let app = test::init_service(App::new().service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/jwt")
        .insert_header((http::header::AUTHORIZATION, "bearer"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn invalid_token() {
    let (scope, _) = common::setup();
    let app = test::init_service(App::new().service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/jwt")
        .insert_header((
            http::header::AUTHORIZATION,
            "bearer invalidadtk",
        ))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}
