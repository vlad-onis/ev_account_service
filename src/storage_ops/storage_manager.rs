use crate::configuration::get_configuration;
use crate::model::account::Account;

use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

/// Handles db operations on accounts
pub struct StorageManager {
    connection_pool: Pool<Postgres>,
}

impl StorageManager {
    /// Creates a connection pool and uses it to construct the StorageManager
    /// Panics if it could not connect to the db.

    // TODO: Error handling instead of panic.
    pub async fn new() -> StorageManager {
        let db_config = get_configuration().expect("Could not load config").database;
        let con_pool = PgPool::connect(&db_config.connection_string())
            .await
            .expect("Could not connect to the database");
        StorageManager {
            connection_pool: con_pool,
        }
    }

    /// Returns a list of all accounts in the db
    pub async fn get_all_accounts(&self) -> Option<Vec<Account>> {
        let saved = sqlx::query_as!(Account, "SELECT username, email, password FROM accounts")
            .fetch_all(&self.connection_pool)
            .await
            .ok();

        // TODO: Replace with tracing logs
        if let Some(accounts) = saved.clone() {
            for (index, account) in accounts.into_iter().enumerate() {
                println!("Account {index}:\n    {account}");
            }
        }

        saved
    }

    pub async fn get_account_by_username(&self, username: &str) -> Option<Account> {
        let saved = sqlx::query_as!(
            Account,
            "SELECT username, email, password FROM accounts WHERE username = $1",
            username
        )
        .fetch_one(&self.connection_pool) // -> Vec<{ country: String, count: i64 }>
        .await
        .ok();

        // TODO: Replace with tracing/logging
        if let Some(account) = saved.clone() {
            println!("Account:\n    {account}");
        }

        saved
    }

    pub async fn insert_account(&self, account: Account) -> bool {
        let id = Uuid::new_v4();
        let res = sqlx::query!(
            "INSERT INTO accounts (id, username, email, password) VALUES ($1, $2, $3, $4)",
            id,
            account.username,
            account.email,
            account.password
        )
        .execute(&self.connection_pool)
        .await;

        if let Err(ref error) = res {
            println!("{}", error);
        }

        res.is_ok()
    }
}
