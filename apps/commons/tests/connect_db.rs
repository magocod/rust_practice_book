#[tokio::test]
async fn my_test() {
    assert!(true);
}

#[cfg(test)]
mod mongodb_tests {
    #[tokio::test]
    async fn test_connect_mongodb() {
        let rs = commons::mongo::connect().await;
        assert!(rs.ok().is_some(), "failed connection")
    }

    #[tokio::test]
    async fn test_default_seed_mongodb() {
        let client = commons::mongo::connect().await.unwrap();
        let db = client.database(commons::mongo::DB_NAME);

        let rs = commons::mongo::seed(&db, None).await;
        assert!(rs.ok().is_some(), "seed failed")
    }

    #[tokio::test]
    async fn test_seed_mongodb() {
        let client = commons::mongo::connect().await.unwrap();
        let db = client.database(commons::mongo::DB_NAME);

        let rs = commons::mongo::seed(&db, Some(5)).await;
        assert!(rs.ok().is_some(), "seed failed")
    }
}
