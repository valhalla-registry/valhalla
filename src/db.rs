use sqlx::{Connection, SqliteConnection, SqlitePool};

pub struct Database {}

pub struct TestRow {
    foo: String,
    bar: String,
}

impl Database {
    pub async fn new() -> Self {
        // let conn = SqliteConnection::connect("sqlite:test.db").await.unwrap();
        let mut conn = SqliteConnection::connect("test.db").await.unwrap();

        let recs = sqlx::query("create table api_tokens")
            .fetch_all(&mut conn)
            .await;

        Self {}
    }
}
