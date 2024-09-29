use crate::config::Config;
use crate::error::Result;
use sqlx::migrate::Migrator;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::str::FromStr;

#[derive(Debug)]
pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn init(config: &Config) -> Result<Self> {
        tracing::debug!("connecting to database, {}", &config.server.db);
        let options = SqliteConnectOptions::from_str(&config.server.db)?
            .create_if_missing(true)
            .to_owned();

        let pool = SqlitePoolOptions::new().connect_with(options).await?;

        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    // pub async fn transaction(&self) -> Result<Transaction<'_, Sqlite>> {
    //     let t = self.pool.begin().await?;
    //     Ok(t)
    // }
}
