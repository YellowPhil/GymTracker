use std::env::var;

use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpServer, cookie::Key, middleware::from_fn, web};
use sea_orm::{Database, DatabaseConnection};

use crate::auth::auth_middleware;

mod auth;
mod user;

#[derive(Debug, Clone)]
struct AppState {
    // TODO: set the user_id in AppState
    // user_id: i32,
    connection: DatabaseConnection,
}
fn auth_config(cfg: &mut web::ServiceConfig) {
    cfg.service(auth::register);
    cfg.service(auth::login);
    cfg.service(auth::logout);
}
fn user_config(cfg: &mut web::ServiceConfig) {
    cfg.service(user::trainings_left);
    cfg.service(user::add_training);
    cfg.service(user::new_plan);
}

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    let secret_key = Key::generate();
    let store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    let connection_string = format!(
        "postgres://{}:{}@{}:{}/{}?sslmode=disable",
        var("DB_USER").unwrap(),
        var("DB_PASSWORD").unwrap(),
        var("DB_HOST").unwrap_or("127.0.0.1".to_string()),
        var("DB_PORT").unwrap_or("5432".to_string()),
        var("DB_NAME").unwrap()
    );

    let conn = Database::connect(connection_string).await.unwrap();
    let app_state = AppState {
        connection: conn,
        // TODO: set the user_id in AppState
        // user_id: -1,
    };

    HttpServer::new(move || {
        App::new().service(
            web::scope("/api")
                .app_data(web::Data::new(app_state.clone()))
                .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
                .service(web::scope("/auth").configure(auth_config))
                .service(
                    web::scope("/user")
                        .wrap(from_fn(auth_middleware))
                        .configure(user_config),
                ),
        )
    })
    .bind("127.0.0.1:8085")?
    .run()
    .await
}

pub fn main() {
    if let Some(e) = start_server().err() {
        println!("{:?}", e);
    }
}
