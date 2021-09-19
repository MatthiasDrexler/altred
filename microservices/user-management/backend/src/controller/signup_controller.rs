use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn sign_up() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
