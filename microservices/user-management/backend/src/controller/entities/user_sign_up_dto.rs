use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserSignUpDto {
    pub email: String,
    pub username: String,
    pub password: String,
}
