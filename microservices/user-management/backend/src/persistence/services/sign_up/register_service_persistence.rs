use diesel::prelude::*;
use waiter_di::{component, provides, Component};

use crate::{
    domain::{
        entities::user::User, services::sign_up::register_service::TRegisterServicePersistence,
    },
    persistence::{connection_pool::TConnectionPool, entities::user_entity::UserEntity, schema::*},
};

#[component]
pub struct RegisterServicePersistence {
    connection_pool: Box<dyn TConnectionPool>,
}

#[provides]
impl TRegisterServicePersistence for RegisterServicePersistence {
    fn register_user(&self, user: User) -> User {
        println!("registered user");

        let connection = &*(self.connection_pool.get_connection());

        let number = users::table
            .filter(users::locked.eq(true))
            .limit(5)
            .load::<UserEntity>(connection)
            .expect("error loading users");
        println!("{}", number.len());

        user
    }
}