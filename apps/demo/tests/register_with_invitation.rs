use actix_web::{http, test, web, App};
use demo::invitation_handler::InvitationData;
use demo::register_handler::UserData;

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

    let inv = common::generate_invitation(data.email, &pool).expect("invitation save");
    // println!("{:?}", inv);

    let req_data = UserData::factory();

    let req = test::TestRequest::post()
        .uri(format!("/api/register/{}", inv.id).as_str())
        .set_json(req_data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}
