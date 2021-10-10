#[cfg_attr(test, derive(Clone))]
pub struct UserToSignUp {
    pub email: String,
    pub username: String,
    pub password: String,
}
