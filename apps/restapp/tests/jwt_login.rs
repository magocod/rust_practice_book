use actix_web::{http, test, App};
use commons::mongo::{User, COLL_USERS, DB_NAME};

use restapp::jwt_handler::{hash_password, AuthData};

mod common;

#[actix_web::test]
async fn valid_credentials() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let mut user = User::factory();

    let auth = AuthData {
        email: user.email.clone(),
        password: user.password.clone(),
    };

    user.password = hash_password(&user.password).expect("failed hash");

    let _ = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<User>(COLL_USERS)
        .insert_one(&user, None)
        .await
        .expect("failed create test user");

    let req = test::TestRequest::post()
        .uri("/api/jwt")
        .set_json(auth)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn invalid_password_hash() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let user = User::factory();

    let auth = AuthData {
        email: user.email.clone(),
        password: user.password.clone(),
    };

    user.save(&app_state.mongodb_client)
        .await
        .expect("failed create test user");

    let req = test::TestRequest::post()
        .uri("/api/jwt")
        .set_json(auth)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn invalid_password() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let mut user = User::factory();

    let auth = AuthData {
        email: user.email.clone(),
        password: "not_valid".to_string(),
    };

    user.password = hash_password(&user.password).expect("failed hash");

    user.save(&app_state.mongodb_client)
        .await
        .expect("failed create test user");

    let req = test::TestRequest::post()
        .uri("/api/jwt")
        .set_json(auth)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::BAD_REQUEST);
}

#[actix_web::test]
async fn email_does_not_exist() {
    let (scope, app_state) = common::db_setup().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let mut user = User::factory();

    let auth = AuthData {
        email: "notexist@fail.com".to_string(),
        password: user.password.clone(),
    };

    user.password = hash_password(&user.password).expect("failed hash");

    user.save(&app_state.mongodb_client)
        .await
        .expect("failed create test user");

    let req = test::TestRequest::post()
        .uri("/api/jwt")
        .set_json(auth)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::NOT_FOUND);
}
