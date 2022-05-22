use actix_web::{web, Scope};

use restapp::state::AppState;

use restapp::jwt_handler;

#[allow(dead_code)]
pub fn setup() -> (Scope, u32) {
    let scope = web::scope("/api").service(
        web::resource("/jwt")
            // .route(web::post().to(jwt_handler::login))
            // .route(web::delete().to(jwt_handler::logout))
            .route(web::get().to(jwt_handler::get_me)),
    );

    (scope, 1)
}

pub async fn db_setup() -> (Scope, web::Data<AppState>) {
    let app_state = web::Data::new(AppState::new().await);

    let scope = web::scope("/api").service(
        web::resource("/jwt")
            .route(web::post().to(jwt_handler::login))
            // .route(web::delete().to(jwt_handler::logout))
            .route(web::get().to(jwt_handler::get_me)),
    );

    (scope, app_state)
}
