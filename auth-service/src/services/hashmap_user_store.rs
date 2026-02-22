use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError
}

#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>
}

impl HashmapUserStore {
    pub async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        match self.users.entry(user.email.clone()) {
            std::collections::hash_map::Entry::Occupied(_) => Err(UserStoreError::UserAlreadyExists),
            std::collections::hash_map::Entry::Vacant(v) => {
                v.insert(user);
                Ok(())
            }
        }
    }

    pub async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        self.users
            .get(email)
            .cloned()
            .ok_or(UserStoreError::UserNotFound)
    }

    pub async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => if user.password.eq(password) {
                Ok(())
            } else {
                Err(UserStoreError::InvalidCredentials)
            }
            None => Err(UserStoreError::UserNotFound)
        }
    }
}

#[cfg(test)]
mod tests {

    use std::result;

    use super::*;

    fn setup() -> (HashmapUserStore, String, String, User) {
        let user_store = HashmapUserStore::default();
        let email = "kevin@mail.com".to_owned();
        let password = "supertricky".to_owned();
        let test_user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: false,
        };
        (user_store, email, password, test_user)
    }

    #[tokio::test]
    async fn test_add_user() {
        let (mut user_store, _, _, test_user) = setup();

        // Test adding a new user
        let result = user_store.add_user(test_user.clone()).await;
        assert!(result.is_ok());

        // Test adding an existing user (should Err)
        let result = user_store.add_user(test_user.clone()).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));
    }

    #[tokio::test]
    async fn test_get_user() {
        let (mut user_store, email, _, test_user) = setup();

        // Test getting a user that exists
        user_store.users.insert(email.clone(), test_user.clone());
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(test_user));

        // Test getting a user that doesn't exist
        let result = user_store.get_user("me@mail.com").await;
        assert_eq!(result, Err(UserStoreError::UserNotFound))
    }

    #[tokio::test]
    async fn test_validate_user() {
        let (mut user_store, email, password, test_user) = setup();

        // insert a valid user into user_store
        user_store.users.insert(email.clone(), test_user.clone());

        // validate user with correct email and password
        let result = user_store
            .validate_user(&email, &password)
            .await;
        assert!(result.is_ok());

        // validate user with correct email but wrong password
        let non_existent_email = "me@mail.com";
        let non_existent_password = "mywrongpassword";
        let result = user_store
            .validate_user(&email, non_existent_email)
            .await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // check if user doesn't exist
        let result = user_store
            .validate_user(non_existent_email, non_existent_password)
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));


    }
}