/// User model containing the data fields that we want
/// to store about users of our authentication service.
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool
}

impl User {
    /// Construct a new User.
    /// 
    /// TODO: Add logic to validate email format, password length, and so on.
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa
        }
    }
}

// TODO: Add test methods for the User model.