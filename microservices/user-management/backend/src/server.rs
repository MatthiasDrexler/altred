use actix_web::{web, App, HttpServer};
use std::env;

use crate::controller;

const ACCEPT_REQUESTS_FROM: &str = "0.0.0.0";
const PORT_ENVIRONMENT_VARIABLE_KEY: &str = "PORT";
const DEFAULT_PORT: &str = "8080";

pub(crate) async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new().service(web::resource("/signup").to(controller::sign_up::sign_up))
    })
    .bind(bind_to())?
    .run()
    .await
}

fn bind_to() -> String {
    let at_port =
        env::var(PORT_ENVIRONMENT_VARIABLE_KEY).unwrap_or_else(|_| String::from(DEFAULT_PORT));
    let bind_to = format!("{}:{}", ACCEPT_REQUESTS_FROM, at_port);
    bind_to
}