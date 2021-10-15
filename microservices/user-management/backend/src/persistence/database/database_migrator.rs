#[cfg(test)]
use mockall::automock;
use waiter_di::{component, profiles, provides, Component, Provider};

use crate::{
    di::di_container,
    persistence::database::connection::TPostgresConnection,
};

embed_migrations!("src/persistence/database/migrations");

pub trait TDatabaseMigrator: Send + Sync {
    fn migrate_database(&self);
}

#[component]
pub struct DatabaseMigrator {
    postgres_connection: Box<dyn TPostgresConnection>,
}

impl Default for DatabaseMigrator {
    fn default() -> Self {
        DatabaseMigrator::new()
    }
}

impl DatabaseMigrator {
    pub fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<DatabaseMigrator>::create(&mut container)
    }

    #[cfg(test)]
    pub fn construct(postgresConnection: Box<dyn TPostgresConnection>) -> Self {
        DatabaseMigrator { postgres_connection: postgresConnection }
    }
}

#[provides]
impl TDatabaseMigrator for DatabaseMigrator {
    fn migrate_database(&self) {
        let connection = self.postgres_connection.establish_connection();
        let migration_result = embedded_migrations::run(&connection);
        match migration_result {
            Ok(_) => (),
            Err(migration_error) => panic!("Could not migrate database: {:?}", migration_error),
        };
    }
}
