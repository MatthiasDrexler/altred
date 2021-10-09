use chrono::Utc;

use crate::domain::entities::{user::User, user_sign_up::UserSignUp};

pub fn register(user: UserSignUp) -> User {
    User {
        email: user.email,
        username: user.username,
        hashed_password: user.password,
        registration_date: Utc::now(),
        activation_date: None,
        locked: false,
    }
}
