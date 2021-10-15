use diesel::{pg::PgConnection, Connection};
use dotenv::dotenv;
#[cfg(test)]
use mockall::automock;
use std::env;
use waiter_di::{component, profiles, provides, Component, Provider};

use crate::di::di_container;

#[cfg_attr(test, automock)]
pub trait TPostgresConnection {
    fn establish_connection(&self) -> PgConnection;
}

#[component]
pub struct PostgresConnection {}

impl Default for PostgresConnection {
    fn default() -> Self {
        PostgresConnection::new()
    }
}

impl PostgresConnection {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<PostgresConnection>::create(&mut container)
    }

    #[cfg(test)]
    pub fn construct() -> Self {
        PostgresConnection {}
    }
}

#[provides]
impl TPostgresConnection for PostgresConnection {
    fn establish_connection(&self) -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    }
}
