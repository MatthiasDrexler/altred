use waiter_di::{component, profiles, provides, Component, Provider};

use crate::{
    di::di_container,
    persistence::database::{
        connection_establisher::TConnectionEstablisher, embedded_migrations::TEmbeddedMigrations,
    },
};

pub(crate) trait TDatabaseMigrator: Send + Sync {
    fn migrate_database(&self);
}

#[component]
pub(crate) struct DatabaseMigrator {
    postgres_connection: Box<dyn TConnectionEstablisher>,
    embedded_migrations: Box<dyn TEmbeddedMigrations>,
}

impl Default for DatabaseMigrator {
    fn default() -> Self {
        DatabaseMigrator::new()
    }
}

impl DatabaseMigrator {
    pub(crate) fn new() -> Self {
        let mut container = di_container::get::<profiles::Default>();
        Provider::<DatabaseMigrator>::create(&mut container)
    }

    #[cfg(test)]
    #[allow(dead_code)]
    pub(crate) fn construct(
        postgres_connection: Box<dyn TConnectionEstablisher>,
        embedded_migrations: Box<dyn TEmbeddedMigrations>,
    ) -> Self {
        DatabaseMigrator {
            postgres_connection,
            embedded_migrations,
        }
    }
}

#[provides]
impl TDatabaseMigrator for DatabaseMigrator {
    fn migrate_database(&self) {
        let connection = self.postgres_connection.establish_connection();
        let migration_result = self.embedded_migrations.run(&connection);
        match migration_result {
            Ok(_) => (),
            Err(migration_error) => panic!("Could not migrate database: {:?}", migration_error),
        };
    }
}
