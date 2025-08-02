use crate::token_generator;
use actix_web::{HttpResponse, Responder, web};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserRequest {
    user_id: String,
}

#[derive(Serialize)]
pub struct TokenResponse {
    token: String,
}

pub async fn forgot_password(req: web::Json<UserRequest>) -> impl Responder {
    let token = token_generator::generate_vulnerable_token(&req.user_id);
    HttpResponse::Ok().json(TokenResponse { token })
}
