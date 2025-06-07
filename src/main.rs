use actix_session::{SessionMiddleware, storage::RedisSessionStore};
use actix_web::{App, HttpResponse, HttpServer, Responder, cookie::Key, get, web};

mod db;
mod user;

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::NotFound().body("Sorry, not implemented yet")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let secret_key = Key::generate();
    let store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(SessionMiddleware::new(store.clone(), secret_key.clone()))
            .service(hello_world)
            .configure(user::user_routes)
            .route("/bebe", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8083))?
    .run()
    .await
}
