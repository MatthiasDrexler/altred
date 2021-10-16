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
#[derive(AutowireWithConstructor)]
pub(crate) struct DatabaseMigrator {
    postgres_connection: Box<dyn TConnectionEstablisher>,
    embedded_migrations: Box<dyn TEmbeddedMigrations>,
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
