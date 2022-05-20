use actix_web::{http, test, web, App};
use demo::invitation_handler::InvitationData;

mod common;

#[actix_web::test]
async fn request_invitation() {
    let (scope, pool, _) = common::setup();
    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(scope),
    )
    .await;

    let data = InvitationData::factory();
    // println!("{:?}", data);

    let req = test::TestRequest::post()
        .uri("/api/invitation")
        .set_json(data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}
