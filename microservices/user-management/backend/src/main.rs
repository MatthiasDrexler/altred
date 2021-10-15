use crate::persistence::database::database_migrator::{DatabaseMigrator, TDatabaseMigrator};

mod controller;
mod di;
mod domain;
mod persistence;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    DatabaseMigrator::new().migrate_database();
    controller::server::run().await
}
