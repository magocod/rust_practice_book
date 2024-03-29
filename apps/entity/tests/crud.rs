#[cfg(test)]
mod post_crud {
    use entity::post;
    use entity::post::Entity as Post;
    // use sea_orm::ActiveValue::NotSet;
    use sea_orm::entity::*;
    use sea_orm::error::DbErr;
    use sea_orm::{DatabaseConnection, DeleteResult};

    async fn create_post() -> Result<(post::Model, DatabaseConnection), DbErr> {
        let conn = sea_orm::Database::connect("postgres://postgres:123@localhost/actix").await?;

        let f = post::Factory::build();

        let p = post::ActiveModel {
            id: NotSet,
            title: Set(f.title),
            text: Set(f.text),
        }
        .insert(&conn)
        .await?;

        Ok((p, conn))
    }

    #[tokio::test]
    async fn create_with_factory_build() {
        let conn = sea_orm::Database::connect("postgres://postgres:123@localhost/actix")
            .await
            .unwrap();

        let f = post::Factory::build();
        // println!("{:?}", f);

        let p: post::Model = post::ActiveModel {
            id: NotSet,
            title: Set(f.title),
            text: Set(f.text),
        }
        .insert(&conn)
        .await
        .expect("could not insert post");

        // println!("{:?}", p);
        assert!(p.id > 0);
    }

    #[tokio::test]
    async fn find_by_id() {
        let (m, conn) = create_post().await.unwrap();
        // Find by primary key
        let found: Option<post::Model> = Post::find_by_id(m.id).one(&conn).await.unwrap();
        println!("{:?}", found);
        assert_eq!(found.is_some(), true);
    }

    #[tokio::test]
    async fn update_by_id() {
        let (m, conn) = create_post().await.unwrap();
        // Find by primary key
        let found: Option<post::Model> = Post::find_by_id(m.id).one(&conn).await.unwrap();
        // println!("{:?}", found);

        // Into ActiveModel
        let mut found: post::ActiveModel = found.unwrap().into();

        // Update name attribute
        found.title = Set("Sweet pear".to_owned());

        // Update corresponding row in database using primary key value
        let _found: post::Model = found.update(&conn).await.unwrap();

        // println!("{:?}", found);

        assert_eq!(true, true);
    }

    #[tokio::test]
    async fn delete_by_id() {
        let (m, conn) = create_post().await.unwrap();
        // Find by primary key
        let found: Option<post::Model> = Post::find_by_id(m.id).one(&conn).await.unwrap();
        let found: post::Model = found.unwrap();
        let res: DeleteResult = found.delete(&conn).await.unwrap();
        println!("{:?}", res);
        assert_eq!(res.rows_affected, 1);
        // assert_eq!(found.is_some(), true);
    }
}
