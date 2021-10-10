use diesel::{
    pg::PgConnection,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenv::dotenv;
use lazy_static::lazy_static;
use std::{env, sync::Mutex};
use waiter_di::{component, provides, Component};

lazy_static! {
    static ref POOL: Mutex<Pool<ConnectionManager<PgConnection>>> =
        Mutex::new(establish_connection_pool());
}

pub trait TConnectionPool: Send + Sync {
    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>>;
}

#[component]
pub struct ConnectionPool {}

#[provides]
impl TConnectionPool for ConnectionPool {
    fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        POOL.lock().unwrap().clone().get().unwrap()
    }
}

pub fn establish_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    diesel::r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Connection pool could not be instantiated")
}
