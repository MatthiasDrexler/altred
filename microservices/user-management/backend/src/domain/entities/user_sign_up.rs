#[cfg_attr(test, derive(Clone))]
pub(crate) struct UserToSignUp {
    pub email: String,
    pub username: String,
    pub password: String,
}
