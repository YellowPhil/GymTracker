use actix_session::Session;
use actix_web::{get, post, web::{self,Json}, HttpResponse, Responder};
use chrono::NaiveDate;
use db::user::{Mutate, MutateError, View};
use entity::user::Plan;
use serde::Deserialize;

use crate::{AppState, auth};


#[derive(Debug, Deserialize)]
struct AddTrainingRequest {
    date: NaiveDate
}

#[post("/add_training")]
pub async fn add_training(state: web::Data<AppState>, session: Session, data: Json<AddTrainingRequest>) -> impl Responder {
    let conn = &state.connection;
    let user_id = session.get::<i32>(auth::USER_ID_KEY).unwrap().unwrap();

    match Mutate::add_training(conn, user_id, data.date).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(MutateError::NoPlan) => HttpResponse::BadRequest().json("User has no active plan"),
        Err(e) => {
            println!("Error adding training: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[get("/trainings")]
pub async fn trainings_left(state: web::Data<AppState>, session: Session) -> impl Responder {
    let conn = &state.connection;
    let user_id = session.get::<i32>(auth::USER_ID_KEY).unwrap().unwrap();

    match View::trainings_left(conn, user_id).await {
        Ok(t) => HttpResponse::Ok().json(t),
        Err(MutateError::NoPlan) => HttpResponse::BadRequest().json("User has no active plan"),
        Err(e) => {
            println!("Error retrieving trainings: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

#[post("/new_plan")]
pub async fn new_plan(state: web::Data<AppState>, session: Session, data: Json<Plan>) -> impl Responder {
    let conn = &state.connection;
    let user_id = session.get::<i32>(auth::USER_ID_KEY).unwrap().unwrap();

    match Mutate::new_plan(conn, user_id, data.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("Error updating the plan: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
