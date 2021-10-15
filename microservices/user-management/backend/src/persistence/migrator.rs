use diesel::PgConnection;

use crate::persistence::connection;

embed_migrations!("src/persistence/migrations");

pub fn migrate_database() {
    run_migration_on(setup_connection());
}

fn setup_connection() -> PgConnection {
    return connection::establish_connection()
}

fn run_migration_on(connection: PgConnection) {
    match embedded_migrations::run(&connection) {
        Ok(_) => (),
        Err(migration_error) => panic!("Could not migrate database: {:?}", migration_error),
    };
}
