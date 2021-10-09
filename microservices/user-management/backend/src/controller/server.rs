use crate::controller;

use actix_web::{
    error,
    web::{self, JsonConfig},
    App, HttpResponse, HttpServer,
};
use std::env;

const ACCEPT_REQUESTS_FROM: &str = "0.0.0.0";
const PORT_ENVIRONMENT_VARIABLE_KEY: &str = "PORT";
const DEFAULT_PORT: &str = "8080";

pub(crate) async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new().service(
            web::resource("/signup")
                .app_data(json_config())
                .route(web::post().to(controller::endpoints::sign_up::sign_up)),
        )
    })
    .bind(bind_to())?
    .run()
    .await
}

fn json_config() -> JsonConfig {
    web::JsonConfig::default()
        .limit(4096)
        .error_handler(|error, _| -> actix_web::Error {
            let message = error.to_string();
            println!("{}", message);
            error::InternalError::from_response(error, HttpResponse::BadRequest().body(message))
                .into()
        })
}

fn bind_to() -> String {
    let at_port =
        env::var(PORT_ENVIRONMENT_VARIABLE_KEY).unwrap_or_else(|_| String::from(DEFAULT_PORT));
    let bind_to = format!("{}:{}", ACCEPT_REQUESTS_FROM, at_port);
    bind_to
}
