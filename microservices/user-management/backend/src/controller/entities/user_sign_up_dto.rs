use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserToSignUpDto {
    pub email: String,
    pub username: String,
    pub password: String,
}
