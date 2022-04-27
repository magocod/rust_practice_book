use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use fake::faker::lorem::raw::{Sentences, Word};
use fake::locales::EN;
use fake::Fake;

// use sea_orm::{entity::ActiveValue::NotSet, ActiveModelTrait, Set};
// use sea_orm::DatabaseConnection;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub title: String,
    #[sea_orm(column_type = "Text")]
    pub text: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Factory;

impl Factory {
    pub fn build() -> Model {
        Model {
            id: 0,
            title: Word(EN).fake(),
            text: Sentences(EN, 1..3).fake::<Vec<String>>().join(" "),
        }
    }
}

// #[derive(Debug)]
// pub struct ModelFactory;
//
// impl ModelFactory {
//     pub async fn create(conn: &DatabaseConnection) {
//         let f = Factory::build();
//         let m = ActiveModel {
//             id: NotSet,
//             title: Set(f.title),
//             text: Set(f.text),
//         }.insert(conn).await;
//     }
// }
