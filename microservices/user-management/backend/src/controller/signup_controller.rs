use actix_web::{HttpResponse};

pub async fn sign_up() -> HttpResponse {
    HttpResponse::Ok().body("Hello World")
}

#[cfg(test)]
mod tests {
    use super::*;

    use actix_web::http::StatusCode;
    use spectral::prelude::*;

    #[actix_rt::test]
    async fn sign_up_should_return_ok_with_hello_world() {
        let response = sign_up().await;

        assert_that(&response.status()).is_equal_to(&StatusCode::OK);
    }
}
