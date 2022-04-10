use mongodb::results::InsertManyResult;
use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::error::Error;

use futures::stream::TryStreamExt;
use mongodb::{bson::doc, options::FindOptions};

use fake::faker::lorem::raw::Sentences;
use fake::faker::name::raw::{FirstName, LastName};
use fake::locales::EN;
use fake::Fake;

pub const DB_NAME: &str = "actix";

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    title: String,
    author: String,
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
