use crate::persistence::database::migrator::{DatabaseMigrator, TDatabaseMigrator};

mod controller;
mod di;
mod domain;
mod persistence;

extern crate openssl;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate autowire_derive;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    DatabaseMigrator::new().migrate_database();
    controller::server::run().await
}
