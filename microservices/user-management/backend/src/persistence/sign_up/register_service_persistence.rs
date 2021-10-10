use waiter_di::{component, provides, Component};

use crate::domain::{entities::user::User, sign_up::register_service::TRegisterServicePersistence};

#[component]
pub struct RegisterServicePersistence {}

#[provides]
impl TRegisterServicePersistence for RegisterServicePersistence {
    fn register_user(&self, user: User) -> User {
        println!("registered user");

        user
    }
}
