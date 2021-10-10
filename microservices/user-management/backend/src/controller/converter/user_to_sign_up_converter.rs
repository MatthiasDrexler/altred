use crate::{
    controller::entities::user_sign_up_dto::UserToSignUpDto,
    domain::entities::user_sign_up::UserToSignUp,
};

pub fn convert_from_user_sign_up_dto(user: UserToSignUpDto) -> UserToSignUp {
    UserToSignUp {
        email: user.email,
        username: user.username,
        password: user.password,
    }
}
