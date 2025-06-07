use actix_web::{middleware::from_fn, web};
use anyhow::{Ok, Result, anyhow};
use serde::{Deserialize, Serialize};

mod auth;
mod routes;

use auth::auth_middleware;

const USER_KEY: &str = "user";

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
enum Plan {
    FifteenTrainings = 15,
    TenTrainings = 10,
    #[default]
    FiveTrainings = 5,
}

#[derive(Deserialize, Serialize, Debug, Default)]
enum UserStatus {
    #[default]
    Default,
    Premium,
}

#[derive(Deserialize, Serialize, Debug)]
struct User {
    login: String,
    status: UserStatus,
    current_plan: Option<Plan>,
    visited_trainings: u8,
}

impl Default for User {
    fn default() -> Self {
        Self {
            login: String::new(),
            current_plan: None,
            visited_trainings: 0,
            status: UserStatus::default(),
        }
    }
}

impl User {
    pub fn new_plan(&mut self, plan: Plan) {
        self.current_plan = Some(plan)
    }
    pub fn new_training(&mut self) -> Result<()> {
        let Some(plan) = self.current_plan.clone() else {
            return Err(anyhow!("You have no active plan"));
        };
        if self.visited_trainings >= plan as u8 {
            return Err(anyhow!("You have no more trainings left"));
        }
        self.visited_trainings += 1;
        Ok(())
    }
}

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/user")
            .wrap(from_fn(auth_middleware))
            .route(web::get().to(routes::index)),
    );
}
