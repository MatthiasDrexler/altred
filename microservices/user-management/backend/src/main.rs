mod server;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}
