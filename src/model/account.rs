use std::fmt::{Debug, Display};

use thiserror::Error;
use uuid::Uuid;

const USERNAME_MIN_LEN: usize = 4;
const USERNAME_MAX_LEN: usize = 20;

#[derive(Error, Debug)]
pub enum AccountValidationError {
    #[error("Username length should be between 4 and 20 characters")]
    InvalidUsernameLength,

    #[error("Email field cannot be empty")]
    EmptyEmail,
}

#[derive(Debug, Clone)]
pub struct Account {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn new(
        username: String,
        email: String,
        password: String,
    ) -> Result<Account, AccountValidationError> {
        let account = Account {
            id: Uuid::new_v4(),
            username,
            email,
            password,
        };

        account.is_valid()?;
        Ok(account)
    }

    // #[allow(dead_code)]
    // pub fn update_username(&mut self, username: String) -> Result<Account, Box<dyn Error>> {
    //     if username.is_empty() {
    //         return Err("Username cannot be empty".into());
    //     }

    //     let account = Account {
    //         id: self.id,
    //         username,
    //         email: self.email.clone(),
    //         password: self.password.clone(),
    //     };

    //     Ok(account)
    // }

    fn is_valid(&self) -> Result<(), AccountValidationError> {
        if (self.username.len() < USERNAME_MIN_LEN) || (self.username.len() > USERNAME_MAX_LEN) {
            return Err(AccountValidationError::InvalidUsernameLength);
        }

        if self.email.is_empty() {
            return Err(AccountValidationError::EmptyEmail);
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
mod tests {

    use super::Account;

    #[test]
    fn random_tests() {
        let account = Account::new(
            String::from("vladonis"),
            String::from("vladonis@gmail.com"),
            String::from("test1234"),
        );

        println!("{}", account);
        assert!(true);
    }
}
