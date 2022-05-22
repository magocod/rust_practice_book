use actix_web::{http, test, App};
use commons::jwt::generate_token;
use commons::mongo::User;

mod common;

#[actix_web::test]
async fn token_exists_in_db() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let mut user = User::factory();
    let rs = user
        .save(&app_state.mongodb_client)
        .await
        .expect("failed save user");

    user.id = rs.inserted_id.as_object_id();

    let tk = user
        .generate_token()
        .expect("failed generate_token")
        .expect("failed tk option");
    tk.save(&app_state.mongodb_client)
        .await
        .expect("failed save token");

    let req = test::TestRequest::delete()
        .uri("/api/jwt")
        .insert_header((http::header::AUTHORIZATION, format!("bearer {}", tk.token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn token_does_not_exists_in_db() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let tk = generate_token(&User::factory()).expect("error generate token");

    let req = test::TestRequest::delete()
        .uri("/api/jwt")
        .insert_header((http::header::AUTHORIZATION, format!("bearer {}", tk)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}

#[actix_web::test]
async fn no_authentication_header() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let req = test::TestRequest::delete().uri("/api/jwt").to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn incomplete_authentication_header() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let req = test::TestRequest::delete()
        .uri("/api/jwt")
        .insert_header((http::header::AUTHORIZATION, "bearer"))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn invalid_token() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let mut user = User::factory();
    let rs = user
        .save(&app_state.mongodb_client)
        .await
        .expect("failed save user");

    user.id = rs.inserted_id.as_object_id();

    let mut tk = user
        .generate_token()
        .expect("failed generate_token")
        .expect("failed tk option");
    tk.token = "invalid".to_string();

    tk.save(&app_state.mongodb_client)
        .await
        .expect("failed save token");

    let req = test::TestRequest::delete()
        .uri("/api/jwt")
        .insert_header((http::header::AUTHORIZATION, format!("bearer {}", tk.token)))
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}
