use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool},
    Connection,
};
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    connection
}

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    let connection_pool = diesel::r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Connection pool could not be instantiated");

    connection_pool
}
