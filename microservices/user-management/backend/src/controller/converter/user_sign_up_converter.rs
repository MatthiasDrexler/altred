use crate::controller::entities::user_sign_up_dto::UserSignUpDto;
use crate::domain::entities::user_sign_up::UserSignUp;

pub fn convert_from_user_sign_up_dto(user: UserSignUpDto) -> UserSignUp {
    UserSignUp {
        email: user.email,
        username: user.username,
        password: user.password,
    }
}
