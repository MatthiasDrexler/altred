use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
#[cfg(test)]
use mockall::automock;
use std::env;
use waiter_di::{component, profiles, provides, Component, Provider};

use crate::di::di_container;

#[cfg_attr(test, automock)]
pub(crate) trait TConnectionEstablisher: Send + Sync {
    fn establish_connection(&self) -> PgConnection;
}

#[component]
#[derive(AutowireWithConstructor)]
pub(crate) struct ConnectionEstablisher {}

#[provides]
impl TConnectionEstablisher for ConnectionEstablisher {
    fn establish_connection(&self) -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
