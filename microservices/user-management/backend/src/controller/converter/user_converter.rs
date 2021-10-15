use crate::{controller::dtos::user_dto::UserDto, domain::entities::user::User};

pub(crate) fn convert_to_user_dto(user: &User) -> UserDto {
    UserDto {
        email: String::from(&user.email),
        username: String::from(&user.username),
        hashed_password: String::from(&user.hashed_password),
        registration_date: user.registration_date,
        activation_date: user.activation_date,
        locked: user.locked,
    }
}
