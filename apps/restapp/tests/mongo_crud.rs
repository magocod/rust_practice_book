use actix_web::{http, test, web, App};

use common::mongo::{Book, COLL_BOOKS, DB_NAME};
use restapp::mongo_crud::{create_book, get_book};
use restapp::state::AppState;

#[actix_web::test]
async fn create_book_successfully() {
    let app_state = web::Data::new(AppState::new().await);

    let scope = web::scope("/books").route("/create_book", web::post().to(create_book));

    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;
    let req = test::TestRequest::post()
        .uri("/books/create_book")
        .set_json(Book::factory())
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn get_one_book() {
    let app_state = web::Data::new(AppState::new().await);

    let book_data = Book::factory();

    let _ = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

    let scope = web::scope("/books").route("/get_book", web::get().to(get_book));

    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;
    let req = test::TestRequest::get()
        .uri("/books/get_book")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}
