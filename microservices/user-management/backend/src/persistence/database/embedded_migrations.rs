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
#[derive(FullAutowire)]
pub(crate) struct EmbeddedMigrations {}

#[provides]
impl TEmbeddedMigrations for EmbeddedMigrations {
    fn run(&self, connection: &PgConnection) -> Result<(), RunMigrationsError> {
        embedded_migrations::run(connection)
    }
}
