use std::fmt::{Debug, Display};

#[derive(Debug, Clone)]
pub struct Account {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl Account {
    pub fn new(username: String, email: String, password: String) -> Account {
        Account {
            username,
            email,
            password,
        }
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
