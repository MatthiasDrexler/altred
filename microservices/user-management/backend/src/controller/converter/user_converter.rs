use crate::{controller::entities::user_dto::UserDto, domain::entities::user::User};

pub fn convert_to_user_dto(user: User) -> UserDto {
    UserDto {
        email: user.email,
        username: user.username,
        hashed_password: user.hashed_password,
        registration_date: user.registration_date,
        activation_date: user.activation_date,
        locked: user.locked,
    }
}
