use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Queryable)]
pub struct UserEntity {
    pub email: String,
    pub username: String,
    pub hashed_password: String,
    pub registration_date: DateTime<Utc>,
    pub activation_date: Option<DateTime<Utc>>,
    pub locked: bool,
}
