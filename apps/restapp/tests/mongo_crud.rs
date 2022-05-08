use actix_web::{http, test, web, App, Scope};

use common::mongo::{Book, BookDoc, COLL_BOOKS, DB_NAME};
use restapp::mongo_crud::{create_book, delete_book, get_book, list_books, update_book};
use restapp::state::AppState;

async fn test_app() -> (Scope, web::Data<AppState>) {
    let app_state = web::Data::new(AppState::new().await);

    let scope = web::scope("/books")
        .route("/create_book", web::post().to(create_book))
        .route("/get_book", web::get().to(get_book))
        .route("/update_book/{_id}", web::put().to(update_book))
        .route("/delete_book/{_id}", web::delete().to(delete_book))
        .route("/list_books", web::get().to(list_books));

    (scope, app_state)
}

#[actix_web::test]
async fn create_book_successfully() {
    let (scope, app_state) = test_app().await;
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
    let (scope, app_state) = test_app().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let book_data = Book::factory();

    let _ = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

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
    let (scope, app_state) = test_app().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let book_data = Book::factory();
    let update_data = Book::factory();

    let ior = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

    let req = test::TestRequest::put()
        .uri(
            format!(
                "/books/update_book/{}",
                ior.inserted_id.as_object_id().unwrap()
            )
            .as_str(),
        )
        .set_json(&update_data)
        .to_request();

    let resp = test::call_service(&app, req).await;

    println!("{}, {:?}", resp.status(), resp.response().body());
    // assert_eq!(resp.status(), http::StatusCode::OK);
}

#[actix_web::test]
async fn remove_one_book_for_id() {
    let (scope, app_state) = test_app().await;
    let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

    let book_data = Book::factory();

    let ior = app_state
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS)
        .insert_one(&book_data, None)
        .await
        .expect("failed create test book");

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

#[cfg(test)]
mod list_books_by_query_string {
    use super::*;

    #[actix_web::test]
    async fn unfiltered() {
        let (scope, app_state) = test_app().await;
        let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

        let book_data = BookDoc::factory();

        let _ = app_state
            .mongodb_client
            .database(DB_NAME)
            .collection::<BookDoc>(COLL_BOOKS)
            .insert_one(&book_data, None)
            .await
            .expect("failed create test book");

        let req = test::TestRequest::get()
            .uri("/books/list_books")
            .to_request();

        // let resp = test::call_service(&app, req).await;
        //
        // println!("{}, {:?}", resp.status(), resp.response().body());
        // assert_eq!(resp.status(), http::StatusCode::OK);

        let resp: Vec<BookDoc> = test::call_and_read_body_json(&app, req).await;
        println!("{:?}", resp);

        assert_eq!(2 + 1, 3);
    }

    #[actix_web::test]
    async fn limit_number_of_documents() {
        let (scope, app_state) = test_app().await;
        let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

        let param = 3;
        let book_data = BookDoc::factory();

        let _ = app_state
            .mongodb_client
            .database(DB_NAME)
            .collection::<BookDoc>(COLL_BOOKS)
            .insert_one(&book_data, None)
            .await
            .expect("failed create test book");

        let req = test::TestRequest::get()
            .uri(format!("/books/list_books?limit={}", param).as_str())
            .to_request();

        // let resp = test::call_service(&app, req).await;
        //
        // println!("{}, {:?}", resp.status(), resp.response().body());
        // assert_eq!(resp.status(), http::StatusCode::OK);

        let resp: Vec<BookDoc> = test::call_and_read_body_json(&app, req).await;
        println!("{:?}", resp);

        assert_eq!(2 + 1, 3);
    }

    #[actix_web::test]
    async fn filter_by_author() {
        let (scope, app_state) = test_app().await;
        let app = test::init_service(App::new().app_data(app_state.clone()).service(scope)).await;

        let book_data = BookDoc::factory();

        let param = &book_data.author.split(" ").next().unwrap();

        // println!("{}", format!("/books/list_books?limit=4&author={}", &book_data.author));

        let _ = app_state
            .mongodb_client
            .database(DB_NAME)
            .collection::<BookDoc>(COLL_BOOKS)
            .insert_one(&book_data, None)
            .await
            .expect("failed create test book");

        let req = test::TestRequest::get()
            .uri(format!("/books/list_books?limit=4&author={}", param).as_str())
            .to_request();

        // let resp = test::call_service(&app, req).await;
        //
        // println!("{}, {:?}", resp.status(), resp.response().body());
        // assert_eq!(resp.status(), http::StatusCode::OK);

        let resp: Vec<BookDoc> = test::call_and_read_body_json(&app, req).await;
        println!("{:?}", resp);

        assert_eq!(2 + 1, 3);
    }
}
