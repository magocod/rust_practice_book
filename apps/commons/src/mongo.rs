use mongodb::results::InsertManyResult;
use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::error::Error;

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

use fake::faker::internet::raw::SafeEmail;
use fake::faker::lorem::raw::Sentences;
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use fake::Fake;
use mongodb::bson::oid::ObjectId;

use crate::jwt::generate_token;
use jsonwebtoken::errors;

pub const DB_NAME: &str = "actix";

pub const COLL_USERS: &str = "users";

pub const COLL_BOOKS: &str = "books";

pub const COLL_TOKENS: &str = "tokens";

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub token: String,
    pub user: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

impl User {
    /// generate random data
    pub fn factory() -> Self {
        Self {
            id: None,
            name: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
            email: SafeEmail(EN).fake(),
        }
    }
}

impl User {
    /// generate tk
    pub fn generate_token(&self) -> errors::Result<Option<Token>> {
        match self.id {
            None => Ok(None),
            Some(v) => Ok(Some(Token {
                id: None,
                token: generate_token(self)?,
                user: v,
            })),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub title: String,
    pub author: String,
}

impl Book {
    /// generate random data
    pub fn factory() -> Self {
        Self {
            title: Sentences(EN, 1..3).fake::<Vec<String>>().join(" "),
            author: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BookDoc {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub author: String,
}

impl BookDoc {
    /// generate random book
    pub fn factory() -> Self {
        Self {
            id: None,
            title: Sentences(EN, 1..3).fake::<Vec<String>>().join(" "),
            author: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
        }
    }
}

pub async fn connect() -> Result<Client, Box<dyn Error>> {
    // Parse a connection string into an options struct.
    let mut client_options = ClientOptions::parse("mongodb://localhost:27017/?readPreference=primary&appname=MongoDB%20Compass&directConnection=true&ssl=false").await?;

    // Manually set an option.
    client_options.app_name = Some(DB_NAME.to_string());

    // Get a handle to the deployment.
    let client = Client::with_options(client_options)?;
    // println!("{:?}", client.clone());

    // List the names of the databases in that deployment.
    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }

    // Get a handle to a database.
    // let db = client.database(DB_NAME);

    // // List the names of the collections in that database.
    // for collection_name in db.list_collection_names(None).await? {
    //     println!("{}", collection_name);
    // }

    Ok(client)
}

pub async fn seed(
    db: &Database,
    quantity: Option<usize>,
) -> Result<InsertManyResult, Box<dyn Error>> {
    let q: usize = match quantity {
        Some(v) => v,
        None => 3,
    };

    // Get a handle to a collection of `Book`.
    let typed_collection = db.collection::<Book>("books");
    let mut books: Vec<Book> = Vec::with_capacity(q);

    for _ in 0..q {
        books.push(Book {
            title: Sentences(EN, 1..3).fake::<Vec<String>>().join(" "),
            author: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
        })
    }

    // let books = vec![
    //     Book {
    //         // title: "The Grapes of Wrath".to_string(),
    //         title: Sentences(EN, 1..3).fake::<Vec<String>>().join(" "),
    //         author: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
    //     },
    //     Book {
    //         title: Sentences(EN, 2..4).fake::<Vec<String>>().join(" "),
    //         author: FirstName(EN).fake::<String>() + " " + LastName(EN).fake(),
    //     },
    // ];

    // Insert the books into "mydb.books" collection, no manual conversion to BSON necessary.
    let books = typed_collection.insert_many(books, None).await?;

    let filter = doc! {};
    // let mut find_options = FindOptions::default();
    // find_options.limit = Some(5);
    let find_options: FindOptions = FindOptions::builder()
        .limit(Some(4))
        .sort(doc! { "title": 1 })
        .build();

    let mut cursor = typed_collection.find(filter, find_options).await?;

    println!(
        "total: {:?}",
        typed_collection.count_documents(None, None).await?
    );

    // Iterate over the results of the cursor.
    while let Some(book) = cursor.try_next().await? {
        println!("title: {:?}", book);
    }

    Ok(books)
}
