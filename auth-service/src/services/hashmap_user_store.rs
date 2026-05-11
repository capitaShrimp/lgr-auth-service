use std::{collections::HashMap};

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError
}
/// Data collection containing a Hash Map of Users.
/// 
/// HashMap of emails 'String's (Key) mapped to 'User' objects (Value) 
#[derive(Default)]
pub struct HashMapUserStore {
    users: HashMap<String, User>
}

impl HashMapUserStore {
    /// Adds a user to the Hash Map of users if the user does not already exist.
    /// 
    /// This function assumes the User info coming in is valid.
    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // check if user already exists
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        }
        // insert the User into the HashMap
        self.users.insert(user.email.clone(), user);
        Result::Ok(())
    }

    /// Retrieves a user from the HashMap using their email address, if the user exists.
    /// If the user does not exist, returns a UserNotFound error.
    pub fn get_user(&self, email: &str) -> Result<&User, UserStoreError> {
        // search the HashMap for the User with that email (returns an Option)
        // transform the Option to a Result
        // -- wrap the User in OK() or wrap the UserNotFound error in Err()
        // -- ? operator propogates the error, should an error occur
        let user = self.users.get(email).ok_or(UserStoreError::UserNotFound)?;
        Result::Ok(user)
    }

    /// Validates a user's credentials by checking if the provided email and password match.
    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        // retrieve the user from the HashMap -- propogate UserStoreError is user does not exist
        let user = self.get_user(email)?;
        // user exists -- validate the password
        if user.password == password {
            Result::Ok(())
        } else {
            Err(UserStoreError::InvalidCredentials)
        }
    }
}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {
    // pull the HashmapUserStore module into scope for unit tests
    use super::*;

    #[tokio::test]
    async fn test_add_user_success() {
        // initialize a new, empty HashmapUserStore
        let mut store = HashMapUserStore::default();

        // add a random user to the store
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        // add the user to the store
        let result = store.add_user(user);
        // assert that adding the user was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_add_user_failure() {
        // initialize a new, empty HashmapUserStore
        let mut store = HashMapUserStore::default();

        // add a user to the store
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        let _result = store.add_user(user);

        // attempt to add the same user to the store again
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        let result = store.add_user(user);
        // assert that adding the user was unsuccessful
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_user_success() {
        // initialize a new, empty HashmapUserStore
        let mut store = HashMapUserStore::default();

        // add a random user to the store
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        // add the user to the store
        let _result = store.add_user(user);
        // get the user from the store
        let result = store.get_user("akon@lgr.com");
        // assert that getting the user was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_user_does_not_exist() {
        // initialize a new, empty HashmapUserStore
        let mut store = HashMapUserStore::default();

        // add a random user to the store
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        // add the user to the store
        let _result = store.add_user(user);
        // get the user from the store
        let result = store.get_user("none@lgr.com");
        // assert that getting the user was unsuccessful
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_validate_user_success() {
        // initialize a new, empty HashmapUserStore
        let mut store = HashMapUserStore::default();

        // add a random user to the store
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        // add the user to the store
        let _result = store.add_user(user);
        // validate the user
        let result = store.validate_user("akon@lgr.com", "password1234567890");
        // assert that validating the user was successful
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_user_failure() {
        // initialize a new, empty HashmapUserStore
        let mut store = HashMapUserStore::default();

        // add a random user to the store
        let user = User::new(
            "akon@lgr.com".to_string(),
            "password1234567890".to_string(),
            false,
        );
        // add the user to the store
        let _result = store.add_user(user);
        // validate the user
        let result = store.validate_user("akon@lgr.com", "wrong-password");
        // assert that validating the user was successful
        assert!(result.is_err());
    }
}