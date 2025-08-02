use crate::token_generator;
use actix_web::{HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}

pub async fn forgot_password() -> impl Responder {
    let token = token_generator::generate_secure_token();
    HttpResponse::Ok().json(TokenResponse { token })
}
