use actix_session::{Session, SessionExt};
use actix_web::{
    Error, HttpResponse, Responder,
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorForbidden,
    middleware::Next,
    post,
    web::{self, Data, Json},
};
use db::user::Mutate as MutateUser;
use db::user::View as ViewUser;
use entity::user::Plan;
use serde::Deserialize;

use crate::AppState;

pub const USER_ID_KEY: &'static str = "user_id";

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

pub type RegisterRequest = LoginRequest;

pub async fn auth_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    // TODO: set the user_id in AppState
    let state: Option<&web::Data<AppState>> = req.app_data();
    match req.get_session().get::<i32>(USER_ID_KEY) {
        Ok(Some(_)) => next.call(req).await,
        _ => Err(ErrorForbidden("Forbidden")),
    }
}

#[post("/login")]
pub async fn login(
    state: web::Data<AppState>,
    session: Session,
    data: Json<LoginRequest>,
) -> impl Responder {
    let Ok(Some(user)) = ViewUser::by_username(&state.connection, &data.username).await else {
        return HttpResponse::Forbidden().json("Invalid login or password");
    };
    if !user.verify_password(&data.password) {
        return HttpResponse::Forbidden().json("Invalid login or password");
    }
    let _ = session.insert(USER_ID_KEY, user.id);
    HttpResponse::Ok().finish()
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    session: Session,
    data: Json<RegisterRequest>,
) -> impl Responder {
    let conn = &state.connection;
    if let Ok(Some(_)) = ViewUser::by_username(conn, &data.username).await {
        return HttpResponse::BadRequest().json("User with this username already exists");
    };
    match ViewUser::by_username(conn, &data.username).await {
        Ok(Some(_)) => {
            return HttpResponse::BadRequest().json("User with this username already exists");
        }
        Err(e) => {
            println!("Error: registering: {:?}", e);
            return HttpResponse::InternalServerError().finish();
        }
        _ => (),
    }
    if let Ok(id) =
        MutateUser::create_user(conn, data.username.clone(), data.password.clone(), None).await
    {
        let _ = session.insert(USER_ID_KEY, id);
        HttpResponse::Ok().finish()
    } else {
        println!("error creating user");
        HttpResponse::InternalServerError().finish()
    }
}

#[post("/logout")]
pub async fn logout(session: Session) -> impl Responder {
    match session.get::<i32>(USER_ID_KEY) {
        Ok(Some(_)) => {
            session.purge();
            HttpResponse::Ok().finish()
        }
        _ => HttpResponse::Forbidden().finish(),
    }
}
