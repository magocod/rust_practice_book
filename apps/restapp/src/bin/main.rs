use actix_web::{web, App, HttpServer};
use std::sync::Mutex;

use commons::mongo::connect;

use restapp::{jwt_handler, mongo_crud, rabbit_handler, state};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let mongodb_client = connect().await.expect("failed to connect mongodb");

    // Note: web::Data created _outside_ HttpServer::new closure
    let app_state = web::Data::new(state::AppState {
        counter: Mutex::new(0),
        mongodb_client,
    });

    let rabbit_state = web::Data::new(state::RabbitState::new().await);

    HttpServer::new(move || {
        let scope = web::scope("/books")
            .route("/create_book", web::post().to(mongo_crud::create_book))
            .route("/get_book", web::get().to(mongo_crud::get_book));

        App::new()
            .app_data(app_state.clone()) // <- register the created data
            .app_data(rabbit_state.clone())
            .service(scope)
            .service(restapp::hello)
            .service(restapp::hello_json)
            .service(restapp::echo)
            .service(restapp::err::custom_error)
            .service(restapp::err::custom_error_json)
            .service(restapp::err::error_with_helper)
            .service(
                web::scope("/api").service(
                    web::resource("/jwt")
                        .route(web::post().to(jwt_handler::login))
                        // .route(web::delete().to(jwt_handler::logout))
                        .route(web::get().to(jwt_handler::get_me)),
                ),
            )
            .route("/hey/{name}", web::get().to(restapp::manual_path))
            .route("/update_counter", web::get().to(state::update_counter))
            .route(
                "/json_update_counter",
                web::get().to(state::json_update_counter),
            )
            .service(web::scope("/rabbit").route(
                "/send_to_queue",
                web::post().to(rabbit_handler::send_to_queue),
            ))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
