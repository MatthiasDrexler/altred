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
pub(crate) struct ConnectionEstablisher {}

impl Default for ConnectionEstablisher {
    fn default() -> Self {
        ConnectionEstablisher::new()
    }
}

impl ConnectionEstablisher {
    pub(crate) fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<ConnectionEstablisher>::create(&mut container)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn construct() -> Self {
        ConnectionEstablisher {}
    }
}

#[provides]
impl TConnectionEstablisher for ConnectionEstablisher {
    fn establish_connection(&self) -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
