use actix_web::{web, HttpResponse, Responder};
use common::mongo::{Book, BookDoc, COLL_BOOKS, DB_NAME};
use mongodb::bson::doc;
use futures::TryStreamExt;
// use mongodb::bson::oid::ObjectId;

use serde::{Deserialize};

use crate::err::ErrorData;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct BookListQuery {
    limit: usize,
}

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
        .collection::<BookDoc>(COLL_BOOKS);
    let d = doc! {
        "author": &book_form.author
    };
    let rs = collection.find_one(d, None).await;

    match rs {
        Ok(b) => match b {
            None => HttpResponse::NotFound().json(ErrorData::new(
                "not found".to_string(),
                "not_found".to_string(),
            )),
            Some(v) => HttpResponse::Ok().json(v),
        },
        Err(e) => HttpResponse::BadRequest().json(ErrorData::new("...".to_string(), e.to_string())),
    }
}

pub async fn update_book(data: web::Data<AppState>, book_form: web::Json<Book>) -> impl Responder {
    let collection = data
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS);

    let query = doc! {
        "title": book_form.title.to_string(),
        "author": book_form.author.to_string(),
    };

    let update = doc! {
        "$set": {
            "title": "....",
            "author": "updated",
        }
    };

    let rs = collection.update_one(query, update, None).await;

    match rs {
        Ok(b) => HttpResponse::Ok().json(b),
        Err(e) => HttpResponse::BadRequest().json(ErrorData::new("...".to_string(), e.to_string())),
    }
}

pub async fn delete_book(data: web::Data<AppState>, _id: web::Path<String>) -> impl Responder {
    let collection = data
        .mongodb_client
        .database(DB_NAME)
        .collection::<Book>(COLL_BOOKS);

    // let t = to_bson(&_id.into_inner()).unwrap();
    let t = _id;
    println!("{:?}", t);

    let d = doc! {
        "_id": "6266b86a456ad7764b8d18bd"
    };
    let rs = collection.delete_one(d, None).await;

    match rs {
        Ok(dr) => HttpResponse::Ok().json(dr),
        Err(e) => HttpResponse::BadRequest().json(ErrorData::new("...".to_string(), e.to_string())),
    }
}

pub async fn list_books(data: web::Data<AppState>, qs: web::Query<BookListQuery>) -> impl Responder {
    println!("{:?}", qs);

    let collection = data
        .mongodb_client
        .database(DB_NAME)
        .collection::<BookDoc>(COLL_BOOKS);
    // let d = doc! {
    //     "author": &book_form.author
    // };
    let rs = collection.find(None, None).await;

    match rs {
        Ok(mut cursor) => {
            let mut v: Vec<BookDoc> = vec![];
            // Iterate over the results of the cursor.
            while let Some(book) = cursor.try_next().await.unwrap() {
                // println!("book: {:?}", book);
                v.push(book);
            }
            HttpResponse::Ok().json(v)
        },
        Err(e) => HttpResponse::BadRequest().json(ErrorData::new("...".to_string(), e.to_string())),
    }
}
