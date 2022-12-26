use std::fmt::{Debug, Display};

use thiserror::Error;
use uuid::Uuid;

use super::email::{Email, EmailValidationError};

const USERNAME_MIN_LEN: usize = 4;
const USERNAME_MAX_LEN: usize = 20;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AccountValidationError {
    #[error("Username length should be between 4 and 20 characters")]
    InvalidUsernameLength,

    #[error("Failed validating the email because: {0}")]
    InvalidEmail(#[from] EmailValidationError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: Email,
    pub password: String,
}

impl Account {
    pub fn new(
        username: String,
        email: String,
        password: String,
    ) -> Result<Account, AccountValidationError> {
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

#[cfg(test)]
pub mod tests {

    use crate::model::email::EmailValidationError;

    use super::{Account, AccountValidationError};

    #[test]
    fn valid_account() {
        let acc = Account::new(
            String::from("test_name"),
            String::from("test_mail@gmail.com"),
            String::from("justatest1234"),
        );

        assert!(acc.is_ok())
    }

    #[test]
    fn invalid_account_short_username() {
        let acc_username_too_short = Account::new(
            String::from("tes"),
            String::from("test_mail@gmail.com"),
            String::from("justatest1234"),
        );

        assert!(acc_username_too_short.is_err());
        let err = acc_username_too_short.unwrap_err();
        assert_eq!(err, AccountValidationError::InvalidUsernameLength);
    }

    #[test]
    fn invalid_account_empty_email() {
        let acc_username_too_short = Account::new(
            String::from("test"),
            String::from(""),
            String::from("justatest1234"),
        );

        assert!(acc_username_too_short.is_err());
        let err = acc_username_too_short.unwrap_err();
        assert_eq!(
            err,
            AccountValidationError::InvalidEmail(EmailValidationError::EmptyEmail)
        );
    }

    #[test]
    fn invalid_account_invalid_email() {
        let acc_username_too_short = Account::new(
            String::from("test"),
            String::from("test_at_gmail.com"),
            String::from("justatest1234"),
        );

        assert!(acc_username_too_short.is_err());
        let err = acc_username_too_short.unwrap_err();
        assert_eq!(
            err,
            AccountValidationError::InvalidEmail(EmailValidationError::InvalidEmail)
        );
    }
}
