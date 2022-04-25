use actix_web::{http, test, web, App};

use common::mongo::{Book, BookDoc, COLL_BOOKS, DB_NAME};
use restapp::mongo_crud::{create_book, delete_book, get_book, list_books, update_book};
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

#[actix_web::test]
async fn update_one_book() {
    let app_state = web::Data::new(AppState::new().await);

    let book_data = Book::factory();

    let _ = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

    let scope = web::scope("/books").route("/get_book", web::put().to(update_book));

    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;
    let req = test::TestRequest::put()
        .uri("/books/get_book")
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    // assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn remove_one_book_for_full_match() {
    let app_state = web::Data::new(AppState::new().await);

    let book_data = Book::factory();

    let ior = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

    let scope = web::scope("/books").route("/delete_book/{_id}", web::delete().to(delete_book));

    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;
    let req = test::TestRequest::delete()
        .uri(
            format!(
                "/books/delete_book/{}",
                ior.inserted_id.as_object_id().unwrap()
            )
            .as_str(),
        )
        .set_json(&book_data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn list_unfiltered_books() {
    let app_state = web::Data::new(AppState::new().await);

    let book_data = BookDoc::factory();

    let _ = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<BookDoc>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

    let scope = web::scope("/books").route("/list_books", web::get().to(list_books));

    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;
    let req = test::TestRequest::get()
        .uri("/books/list_books")
        .to_request();

    // let resp = test::call_service(&app, req).await;
    //
    // println!("{}, {:?}", resp.status(), resp.response().body());
    // assert_eq!(resp.status(), http::StatusCode::OK);

    let resp: Vec<Book> = test::call_and_read_body_json(&app, req).await;
    println!("{:?}", resp);
}















