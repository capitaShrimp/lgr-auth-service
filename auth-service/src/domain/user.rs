/// User model containing the fields of data that we want
/// to store about users of our authentication service.
pub struct User {
    pub email: String,
    pub password: String,
    pub requires_2fa: bool
}

impl User {
    /// Construct a new User.
    pub fn new(email: String, password: String, requires_2fa: bool) -> Self {
        User {
            email,
            password,
            requires_2fa
        }
    }
}