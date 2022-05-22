use actix_web::{web, Scope};

use restapp::jwt_handler;

pub fn setup() -> (Scope, u32) {
    let scope = web::scope("/api").service(
        web::resource("/jwt")
            // .route(web::post().to(jwt_handler::login))
            // .route(web::delete().to(jwt_handler::logout))
            .route(web::get().to(jwt_handler::get_me)),
    );

    (scope, 1)
}
