use std::fmt::{Debug, Display};

use anyhow::Result;
use thiserror::Error;
use uuid::Uuid;

use super::email::Email;

const USERNAME_MIN_LEN: usize = 4;
const USERNAME_MAX_LEN: usize = 20;

#[derive(Error, Debug)]
pub enum AccountValidationError {
    #[error("Username length should be between 4 and 20 characters")]
    InvalidUsernameLength,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: Email,
    pub password: String,
}

impl Account {
    pub fn new(username: String, email: String, password: String) -> Result<Account> {
        Account::is_valid(&username)?;
        let email = Email::new(email)?;
        let account = Account {
            id: Uuid::new_v4(),
            username,
            email,
            password,
        };

        Ok(account)
    }

    fn is_valid(username: &str) -> Result<(), AccountValidationError> {
        if (username.len() < USERNAME_MIN_LEN) || (username.len() > USERNAME_MAX_LEN) {
            return Err(AccountValidationError::InvalidUsernameLength);
        }

        Ok(())
    }
}

impl Display for Account {
    // TODO: Remove password or let it only for debug version
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Find a nicer way to format, not with 4 whitespaces in between words.
        let display_account = format!(
            "Username: {}\n    \
             Email: {}\n    \
             Password: {}\n",
            self.username, self.email, self.password
        );
        write!(f, "{}", display_account)
    }
}

// TODO: Add unit tests.
