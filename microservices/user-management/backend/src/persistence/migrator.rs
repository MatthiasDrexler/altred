use diesel::PgConnection;

embed_migrations!("src/persistence/migrations");

pub fn migrate_database(connection: PgConnection) {
    match embedded_migrations::run(&connection) {
        Ok(_) => (),
        Err(migration_error) => panic!("Could not migrate database: {:?}", migration_error)
    };
}
