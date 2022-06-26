use actix_web::{http, test, App};
mod common;

#[actix_web::test]
async fn task_sent_to_the_rabbit() {
    let (scope, _) = common::db_setup().await;
    let app = test::init_service(App::new().service(scope)).await;

    let req = test::TestRequest::get()
        .uri("/api/rabbit/send_to_queue")
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}