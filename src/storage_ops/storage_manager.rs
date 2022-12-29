use crate::configuration::{get_configuration, DatabaseSettings};
use crate::model::account::Account;
use crate::model::email::Email;

use anyhow::Result;
use sqlx::{Error, PgPool};
use tracing;
use uuid::Uuid;

/// Handles db operations on accounts
pub struct StorageManager {
    // connection_pool: Pool<Postgres>,
    pub db_settings: DatabaseSettings,
}

impl StorageManager {
    /// Initiates the StorageManager object with the db config from the provided config file
    pub fn new() -> StorageManager {
        // TODO: Error handling instead of panic.
        let db_settings = get_configuration().expect("Could not load config").database;

        tracing::info!("Storage manage created");

        StorageManager { db_settings }
    }

    /// returns a connection pool
    pub async fn connection(&self) -> Result<PgPool, Error> {
        PgPool::connect(&self.db_settings.connection_string()).await
    }

    /// Returns a list of all accounts in the db
    pub async fn get_all_accounts(&self) -> Result<Vec<Account>> {
        let connection = self.connection().await?;

        let saved = sqlx::query_as!(
            Account,
            r#"
            SELECT id, username, email as "email: Email", password  FROM accounts"#
        )
        .fetch_all(&connection)
        .await?;

        tracing::info!("Retrieved all accounts from the database");

        // TODO: Replace with tracing logs
        for (index, account) in saved.iter().enumerate() {
            println!("Account {index}:\n    {account}");
        }
        Ok(saved)
    }

    pub async fn get_account_by_username(&self, username: &str) -> Result<Account> {
        let connection = self.connection().await?;
        let saved = sqlx::query_as!(
            Account,
            r#"SELECT id, username, email as "email: Email", password FROM accounts WHERE username = $1"#,
            username
        )
        .fetch_one(&connection) // -> Vec<{ country: String, count: i64 }>
        .await?;

        tracing::info!("Retrieved account: {} from the database", saved);
        Ok(saved)
    }

    pub async fn insert_account(&self, account: Account) -> Result<Account> {
        let connection = self.connection().await?;
        let id = Uuid::new_v4();
        let _ = sqlx::query!(
            "INSERT INTO accounts (id, username, email, password) VALUES ($1, $2, $3, $4)",
            id,
            account.username,
            account.email.email_address.clone(),
            account.password,
        )
        .execute(&connection)
        .await?;

        tracing::info!("Inserted account: {} into the database", account);

        Ok(account)
    }

    pub async fn delete_account(&self, account: Account) -> Result<Account> {
        let connection = self.connection().await?;
        let _ = sqlx::query!("DELETE FROM accounts WHERE username=$1", account.username,)
            .execute(&connection)
            .await?;

        tracing::info!("Deleted account: {} from the database", account);

        Ok(account)
    }

    pub async fn update_account_by_username(
        &self,
        old_account: Account,
        new_account: Account,
    ) -> Result<Account> {
        let connection = self.connection().await?;
        let _ = sqlx::query!(
            "UPDATE accounts SET id=$1, username=$2, email=$3, password=$4",
            old_account.id,
            new_account.username,
            new_account.email.email_address.clone(),
            new_account.password
        )
        .execute(&connection)
        .await?;

        tracing::info!("Updated account: {} from the database", old_account);

        Ok(new_account)
    }
}

impl Default for StorageManager {
    fn default() -> Self {
        Self::new()
    }
}
