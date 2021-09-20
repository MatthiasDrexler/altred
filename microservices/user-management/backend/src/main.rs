mod controller;

use actix_web::{web, App, HttpServer};
use std::env;

const ACCEPT_REQUESTS_FROM: &str = "0.0.0.0";
const PORT_ENVIRONMENT_VARIABLE_KEY: &str = "PORT";
const DEFAULT_PORT: &str = "8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/signup").to(controller::sign_up::sign_up))
    })
    .bind(bind_to())?
    .run()
    .await
}

fn bind_to() -> String {
    let at_port = env::var(PORT_ENVIRONMENT_VARIABLE_KEY).unwrap_or(String::from(DEFAULT_PORT));
    format!("{}:{}", ACCEPT_REQUESTS_FROM, at_port)
}
