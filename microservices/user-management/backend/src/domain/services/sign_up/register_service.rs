use chrono::Utc;
#[cfg(test)]
use mockall::automock;
use waiter_di::{component, profiles, provides, Component, Provider};

use crate::{
    di::di_container,
    domain::entities::{user::User, user_sign_up::UserToSignUp},
};

#[cfg_attr(test, automock)]
pub(crate) trait TRegisterService: Send + Sync {
    fn register(&self, user_to_sign_up: &UserToSignUp) -> User;
}

#[cfg_attr(test, automock)]
pub(crate) trait TRegisterServicePersistence: Send + Sync {
    fn register_user(&self, user: &User) -> User;
}

#[component]
#[derive(AutowireWithConstructor)]
pub(crate) struct RegisterService {
    register_service_persistence: Box<dyn TRegisterServicePersistence>,
}

#[provides]
impl TRegisterService for RegisterService {
    fn register(&self, user_to_sign_up: &UserToSignUp) -> User {
        let user = User {
            email: String::from(&user_to_sign_up.email),
            username: String::from(&user_to_sign_up.username),
            hashed_password: String::from(&user_to_sign_up.password),
            registration_date: Utc::now(),
            activation_date: None,
            locked: false,
        };

        self.register_service_persistence.register_user(&user)
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
            .returning(|user| User {
                email: String::from(&user.email),
                username: String::from(&user.username),
                hashed_password: String::from(&user.hashed_password),
                registration_date: user.registration_date,
                activation_date: user.activation_date,
                locked: user.locked,
            });
        let register_service = set_up(persistence_mock);
        let user_to_sign_up = UserToSignUp {
            email: String::from("test@mail.com"),
            username: String::from("username"),
            password: String::from("password"),
        };

        let user = register_service.register(&user_to_sign_up);

        assert_that(&user.email).is_equal_to(&user_to_sign_up.email);
        assert_that(&user.username).is_equal_to(&user_to_sign_up.username);
        // assert_that(&user.hashed_password).is_not_equal_to(&user_to_sign_up.password);
    }
}
