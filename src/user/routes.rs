use super::User;
use actix_session::Session;
use actix_web::{HttpResponse, Responder, web::Json};


pub async fn index(session: Session) -> impl Responder {
    let Ok(Some(user)) = session.get::<User>("user") else {
        return HttpResponse::PermanentRedirect()
            .reason("You are not logged in")
            .finish();
    };
    match user.current_plan {
        Some(plan) => HttpResponse::Ok().body(format!(
            "You have {} trainings left",
            plan as u8 - user.visited_trainings
        )),
        None => HttpResponse::Ok().body(format!(
            "You don't have an active plan",
        )),
    }
}

// async fn add_training(session: Session) -> impl Responder {
// }
