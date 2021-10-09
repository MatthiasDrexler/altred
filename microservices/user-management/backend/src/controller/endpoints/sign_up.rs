use actix_web::{web, HttpResponse};

use crate::controller::converter::user_converter::convert_to_user_dto;
use crate::controller::converter::user_sign_up_converter::convert_from_user_sign_up_dto;
use crate::controller::entities::user_sign_up_dto::UserSignUpDto;
use crate::domain::sign_up::register_service::register;

pub async fn sign_up(user_sign_up_dto: web::Json<UserSignUpDto>) -> HttpResponse {
    let user = convert_from_user_sign_up_dto(user_sign_up_dto.into_inner());
    let registered_user = register(user);
    let registered_user_dto = convert_to_user_dto(registered_user);

    HttpResponse::Ok().json(registered_user_dto)
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;
    use spectral::prelude::*;

    #[actix_rt::test]
    async fn sign_up_should_return_ok_with_hello_world() {
        let user_sign_up_dto = UserSignUpDto {
            email: String::from("test@mail.com"),
            username: String::from("username"),
            password: String::from("password"),
        };
        let user_sign_up_json = web::Json(user_sign_up_dto);

        let response = sign_up(user_sign_up_json).await;

        assert_that(&response.status()).is_equal_to(&StatusCode::OK);
    }
}
