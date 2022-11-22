use super::super::configuration::get_configuration;

use sqlx::{PgPool, Pool, Postgres};

pub struct StorageManager {
    connection_pool: Pool<Postgres>,
}

impl StorageManager {
    pub async fn new() -> StorageManager {
        let db_config = get_configuration().expect("Could not load config").database;
        let con_pool = PgPool::connect(&db_config.connection_string())
            .await
            .expect("Could not connect to the database");
        StorageManager {
            connection_pool: con_pool,
        }
    }

    pub async fn get_all_accounts(&self) {
        let saved = sqlx::query!("SELECT * FROM accounts")
            .fetch_all(&self.connection_pool)
            .await
            .expect("Failed to get accounts");

        println!("Accounts:\n{:?}", saved);
    }
}
