use chrono::Utc;
use waiter_di::{component, profiles, Component, Provider};

use crate::{
    di::di_container,
    domain::entities::{user::User, user_sign_up::UserToSignUp},
};

pub trait TRegisterServicePersistence: Send + Sync {
    fn register_user(&self, user: User) -> User;
}

#[component]
pub struct RegisterService {
    register_service_persistence: Box<dyn TRegisterServicePersistence>,
}

impl RegisterService {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        let register_service = Provider::<RegisterService>::create(&mut container);

        register_service
    }

    pub fn register(&self, user_to_sign_up: UserToSignUp) -> User {
        let user = User {
            email: user_to_sign_up.email,
            username: user_to_sign_up.username,
            hashed_password: user_to_sign_up.password,
            registration_date: Utc::now(),
            activation_date: None,
            locked: false,
        };

        self.register_service_persistence.register_user(user)
    }
}
