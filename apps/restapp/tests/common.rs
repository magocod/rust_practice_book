use actix_web::{web, Scope};

use restapp::state::{AppState, RabbitState};

use restapp::{github, jwt_handler};

#[allow(dead_code)]
pub fn setup() -> (Scope, u32) {
    let http_client = web::Data::new(reqwest::Client::new());

    let scope = web::scope("/api")
        .app_data(http_client.clone())
        .service(
            web::resource("/jwt")
                // .route(web::post().to(jwt_handler::login))
                // .route(web::delete().to(jwt_handler::logout))
                .route(web::get().to(jwt_handler::get_me)),
        )
        .route("http_request", web::get().to(github::http_request))
        .route(
            "reuse_http_request",
            web::get().to(github::reuse_http_request),
        );

    (scope, 1)
}

#[allow(dead_code)]
pub async fn db_setup() -> (Scope, web::Data<AppState>) {
    let http_client = web::Data::new(reqwest::Client::new());
    let app_state = web::Data::new(AppState::new().await);
    let rabbit_state = web::Data::new(RabbitState::new().await);

    let scope = web::scope("/api")
        .app_data(http_client.clone())
        .app_data(rabbit_state.clone())
        .service(
            web::resource("/jwt")
                .route(web::post().to(jwt_handler::login))
                .route(web::delete().to(jwt_handler::logout))
                .route(web::get().to(jwt_handler::get_me)),
        );

    (scope, app_state)
}
