use diesel::PgConnection;
use diesel_migrations::RunMigrationsError;
#[cfg(test)]
use mockall::automock;
use waiter_di::{component, profiles, provides, Component, Provider};

use crate::di::di_container;

embed_migrations!("src/persistence/database/migrations");

#[cfg_attr(test, automock)]
pub(crate) trait TEmbeddedMigrations: Send + Sync {
    fn run(&self, connection: &PgConnection) -> Result<(), RunMigrationsError>;
}

#[component]
pub(crate) struct EmbeddedMigrations {}

impl Default for EmbeddedMigrations {
    fn default() -> Self {
        EmbeddedMigrations::new()
    }
}

impl EmbeddedMigrations {
    pub(crate) fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<EmbeddedMigrations>::create(&mut container)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn construct() -> Self {
        EmbeddedMigrations {}
    }
}

#[provides]
impl TEmbeddedMigrations for EmbeddedMigrations {
    fn run(&self, connection: &PgConnection) -> Result<(), RunMigrationsError> {
        embedded_migrations::run(connection)
    }
}
