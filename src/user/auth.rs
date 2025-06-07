use crate::user::{USER_KEY, User};
use actix_session::SessionExt;
use actix_web::{
    Error, Responder,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error,
    middleware::Next,
    web::Json,
};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

type RegisterRequest = LoginRequest;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    if cfg!(debug_assertions) {
        req.get_session()
            .insert(
                "user",
                User::default()
            )
            .unwrap();
    }

    return match req.get_session().get::<User>(USER_KEY) {
        Ok(Some(_)) => next.call(req).await,
        _ => Err(error::ErrorUnauthorized("Unauthorized")),
    };
}

// pub async fn register(register_request: Json<RegisterRequest>) -> impl Responder {
// }
// pub async fn login(login_request: Json<LoginRequest>) -> impl Responder {}
