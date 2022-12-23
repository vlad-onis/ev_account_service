use regex::Regex;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::fmt::{Debug, Display};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum EmailValidationError {
    #[error("Could not construct regex for email validation")]
    InvalidRegexConstruct,

    #[error("Could not validate the provided email address")]
    InvalidEmail,

    #[error("Email cannot be empty")]
    EmptyEmail,
}

#[derive(Debug, Clone, sqlx::Type)]
pub struct Email {
    pub email_address: String,
}

impl Email {
    pub fn new(email_address: String) -> Result<Email, EmailValidationError> {
        Email::validate_email(&email_address)?;

        let email = Email { email_address };

        Ok(email)
    }

    fn validate_email(email_address: &str) -> Result<(), EmailValidationError> {
        if email_address.is_empty() {
            return Err(EmailValidationError::EmptyEmail);
        }

        let Ok(email_regex) = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})") else {
            return Err(EmailValidationError::InvalidRegexConstruct);
        };

        if !email_regex.is_match(email_address) {
            return Err(EmailValidationError::InvalidEmail);
        }

        Ok(())
    }
}

impl From<String> for Email {
    fn from(email_address: String) -> Self {
        Email { email_address }
    }
}

impl Display for Email {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.email_address)
    }
}

#[cfg(test)]
pub mod tests {
    use super::Email;

    #[test]
    pub fn test_email_regex() {
        let email_addresses = [
            "",
            "foo at bar.com",
            "foo.bar42@c.com",
            "42@c.com",
            "f@42.co",
            "foo@4-2.team",
            ".x@c.com",
            "x.@c.com",
            "foo_bar@bar.com",
            "_bar@bar.com",
            "foo_@bar.com",
            "foo+bar@bar.com",
            "+bar@bar.com",
            "foo+@bar.com",
            "foo.lastname@bar.com",
        ];

        for email in email_addresses {
            if let Err(e) = Email::validate_email(email) {
                println!("{e}");
            } else {
                println!("{email} is valid");
            }
        }
    }
}
