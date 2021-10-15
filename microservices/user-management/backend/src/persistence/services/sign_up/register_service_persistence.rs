use diesel::prelude::*;
use waiter_di::{component, provides, Component};

use crate::{
    domain::{
        entities::user::User, services::sign_up::register_service::TRegisterServicePersistence,
    },
    persistence::{database::connection_pool::TPostgresConnectionPool, entities::user_entity::UserEntity, database::schema::*},
};

#[component]
pub struct RegisterServicePersistence {
    connection_pool: Box<dyn TPostgresConnectionPool>,
}

#[provides]
impl TRegisterServicePersistence for RegisterServicePersistence {
    fn register_user(&self, user: &User) -> User {
        println!("registered user");

        let connection = &*(self.connection_pool.get_connection());

        let number = users::table
            .filter(users::locked.eq(true))
            .limit(5)
            .load::<UserEntity>(connection)
            .expect("error loading users");
        println!("{}", number.len());

        User {
            email: String::from(&user.email),
            username: String::from(&user.username),
            hashed_password: String::from(&user.hashed_password),
            registration_date: user.registration_date,
            activation_date: user.activation_date,
            locked: user.locked,
        }
    }
}
