use actix_web::{web, HttpResponse};
use waiter_di::{component, profiles, Component, Provider};

use crate::{
    controller::{
        converter::{
            user_converter::convert_to_user_dto,
            user_to_sign_up_converter::convert_from_user_sign_up_dto,
        },
        dtos::user_sign_up_dto::UserToSignUpDto,
    },
    di::di_container,
    domain::services::sign_up::register_service::TRegisterService,
};

pub async fn sign_up(user_sign_up_dto: web::Json<UserToSignUpDto>) -> HttpResponse {
    SignUpEndpoint::new().sign_up(user_sign_up_dto)
}

#[component]
pub struct SignUpEndpoint {
    register_service: Box<dyn TRegisterService>,
}

impl Default for SignUpEndpoint {
    fn default() -> Self {
        SignUpEndpoint::new()
    }
}

impl SignUpEndpoint {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<SignUpEndpoint>::create(&mut container)
    }

    #[cfg(test)]
    pub fn construct(service: Box<dyn TRegisterService>) -> Self {
        SignUpEndpoint {
            register_service: service,
        }
    }

    pub fn sign_up(&self, user_sign_up_dto: web::Json<UserToSignUpDto>) -> HttpResponse {
        let user = convert_from_user_sign_up_dto(&user_sign_up_dto.into_inner());

        let registered_user = self.register_service.register(&user);

        let registered_user_dto = convert_to_user_dto(&registered_user);
        HttpResponse::Ok().json(registered_user_dto)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{entities::user::User, services::sign_up::register_service::*};

    use actix_web::http::StatusCode;
    use mockall::predicate;
    use spectral::assert_that;

    #[actix_rt::test]
    async fn sign_up_should_return_ok_with_hello_world() {
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
        let register_service = RegisterService::construct(Box::new(persistence_mock));
        let sign_up_endpoint = SignUpEndpoint::construct(Box::new(register_service));
        let user_sign_up_dto = UserToSignUpDto {
            email: String::from("test@mail.com"),
            username: String::from("username"),
            password: String::from("password"),
        };
        let user_sign_up_json = web::Json(user_sign_up_dto);

        let response = sign_up_endpoint.sign_up(user_sign_up_json);

        assert_that(&response.status()).is_equal_to(&StatusCode::OK);
    }
}
