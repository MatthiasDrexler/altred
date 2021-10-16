use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) struct UserDto {
    pub email: String,
    pub username: String,
    pub hashed_password: String,
    pub registration_date: DateTime<Utc>,
    pub activation_date: Option<DateTime<Utc>>,
    pub locked: bool,
}
