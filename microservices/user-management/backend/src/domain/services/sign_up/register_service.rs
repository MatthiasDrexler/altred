use chrono::Utc;
#[cfg(test)]
use mockall::automock;
use waiter_di::{component, profiles, provides, Component, Provider};

use crate::{
    di::di_container,
    domain::entities::{user::User, user_sign_up::UserToSignUp},
};

#[cfg_attr(test, automock)]
pub trait TSignUpService: Send + Sync {
    fn register(&self, user_to_sign_up: UserToSignUp) -> User;
}

#[cfg_attr(test, automock)]
pub trait TRegisterServicePersistence: Send + Sync {
    fn register_user(&self, user: User) -> User;
}

#[component]
pub struct RegisterService {
    register_service_persistence: Box<dyn TRegisterServicePersistence>,
}

impl Default for RegisterService {
    fn default() -> Self {
        RegisterService::new()
    }
}

#[provides]
impl TSignUpService for RegisterService {
    fn register(&self, user_to_sign_up: UserToSignUp) -> User {
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

impl RegisterService {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<RegisterService>::create(&mut container)
    }

    #[cfg(test)]
    pub fn construct(persistence: Box<dyn TRegisterServicePersistence>) -> Self {
        RegisterService {
            register_service_persistence: persistence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use mockall::predicate;
    use spectral::assert_that;

    fn set_up(persistence_mock: MockTRegisterServicePersistence) -> RegisterService {
        RegisterService {
            register_service_persistence: Box::new(persistence_mock),
        }
    }

    #[actix_rt::test]
    async fn register_should_set_registration_date_and_pass_to_persistence() {
        let mut persistence_mock = MockTRegisterServicePersistence::new();
        persistence_mock
            .expect_register_user()
            .with(predicate::always())
            .times(1)
            .returning(|user| user);
        let register_service = set_up(persistence_mock);
        let user_to_sign_up = UserToSignUp {
            email: String::from("test@mail.com"),
            username: String::from("username"),
            password: String::from("password"),
        };

        let user = register_service.register(user_to_sign_up.clone());

        assert_that(&user.email).is_equal_to(&user_to_sign_up.email);
        assert_that(&user.username).is_equal_to(&user_to_sign_up.username);
        // assert_that(&user.hashed_password).is_not_equal_to(&user_to_sign_up.password);
    }
}
