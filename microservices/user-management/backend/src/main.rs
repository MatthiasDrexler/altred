mod controller;
mod persistence;

#[macro_use]
extern crate diesel_migrations;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let connection = persistence::connection::establish_connection();
    persistence::migrator::migrate_database(connection);
    controller::server::run().await
}
