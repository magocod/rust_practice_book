use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use common::mongo::connect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_client = connect().await.expect("failed to connect mongodb");

    // Note: web::Data created _outside_ HttpServer::new closure
    let app_state = web::Data::new(restapp::state::AppState {
        counter: Mutex::new(0),
        mongodb_client,
    });

    HttpServer::new(move || {
        let scope = web::scope("/books").route(
            "/create_book",
            web::post().to(restapp::mongo_crud::create_book),
        ).route(
            "/get_book",
            web::get().to(restapp::mongo_crud::get_book),
        );

        App::new()
            .app_data(app_state.clone()) // <- register the created data
            .service(scope)
            .service(restapp::hello)
            .service(restapp::hello_json)
            .service(restapp::echo)
            .service(restapp::err::custom_error)
            .service(restapp::err::custom_error_json)
            .service(restapp::err::error_with_helper)
            .route("/hey/{name}", web::get().to(restapp::manual_path))
            .route(
                "/update_counter",
                web::get().to(restapp::state::update_counter),
            )
            .route(
                "/json_update_counter",
                web::get().to(restapp::state::json_update_counter),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
