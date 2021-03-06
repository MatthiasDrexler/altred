use crate::{
    controller::dtos::user_sign_up_dto::UserToSignUpDto,
    domain::entities::user_sign_up::UserToSignUp,
};

pub(crate) fn convert_from_user_sign_up_dto(user: &UserToSignUpDto) -> UserToSignUp {
    UserToSignUp {
        email: String::from(&user.email),
        username: String::from(&user.username),
        password: String::from(&user.password),
    }
}
