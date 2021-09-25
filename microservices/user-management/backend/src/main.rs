mod persistence;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    persistence::connection::establish_connection();
    controller::server::run().await
}
