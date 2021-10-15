use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct UserToSignUpDto {
    pub email: String,
    pub username: String,
    pub password: String,
}
