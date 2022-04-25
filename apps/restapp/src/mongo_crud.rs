use actix_web::{web, HttpResponse, Responder};
use common::mongo::{Book, COLL_BOOKS, DB_NAME};
use mongodb::bson::doc;

use crate::err::ErrorData;
use crate::state::AppState;

pub async fn create_book(data: web::Data<AppState>, book_form: web::Json<Book>) -> impl Responder {
    let collection = data
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS);

    let rs = collection
        .insert_one(
            Book {
                title: book_form.title.to_string(),
                author: book_form.author.to_string(),
            },
            None,
        )
        .await;

    match rs {
        Ok(b) => HttpResponse::Ok().json(b),
        Err(e) => HttpResponse::BadRequest().json(ErrorData::new("...".to_string(), e.to_string())),
    }
}

pub async fn get_book(data: web::Data<AppState>, book_form: web::Json<Book>) -> impl Responder {
    let collection = data
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS);
    let d = doc! {
        "author": &book_form.author
    };
    let rs = collection.find_one(d, None).await;

    match rs {
        Ok(b) => {
            match b {
                None => HttpResponse::NotFound().json(ErrorData::new(
                    "not found".to_string(),
                    "not_found".to_string(),
                )),
                Some(v) => HttpResponse::Ok().json(v),
            }
        }
        Err(e) => HttpResponse::BadRequest().json(ErrorData::new("...".to_string(), e.to_string())),
    }
}
