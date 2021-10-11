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
    set_up_di_container();
    migrate_database();

    controller::server::run().await
}

fn set_up_di_container() {}

fn migrate_database() {
    let connection = persistence::connection::establish_connection();
    persistence::migrator::migrate_database(connection);
}
