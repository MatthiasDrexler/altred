use actix_web::{web, HttpResponse};

use crate::{
    controller::{
        converter::{
            user_converter::convert_to_user_dto,
            user_to_sign_up_converter::convert_from_user_sign_up_dto,
        },
        entities::user_sign_up_dto::UserToSignUpDto,
    },
    domain::services::sign_up::register_service::RegisterService,
};

pub async fn sign_up(user_sign_up_dto: web::Json<UserToSignUpDto>) -> HttpResponse {
    let user = convert_from_user_sign_up_dto(user_sign_up_dto.into_inner());

    let register_service = RegisterService::default();
    let registered_user = register_service.register(user);

    let registered_user_dto = convert_to_user_dto(registered_user);
    HttpResponse::Ok().json(registered_user_dto)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::services::sign_up::register_service::*;

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
            .returning(|user| user);
        let register_service = RegisterService::construct(Box::new(persistence_mock));
        let user_sign_up_dto = UserToSignUpDto {
            email: String::from("test@mail.com"),
            username: String::from("username"),
            password: String::from("password"),
        };
        let user_sign_up_json = web::Json(user_sign_up_dto);

        let response = sign_up(user_sign_up_json).await;

        assert_that(&response.status()).is_equal_to(&StatusCode::OK);
    }
}
