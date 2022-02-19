use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .route("/users", web::get().to(handlers::get_users))
        .route("/users/{id}", web::get().to(handlers::get_user_by_id))
        .route("/users", web::post().to(handlers::add_user))
        .route("/users/{id}", web::delete().to(handlers::delete_user))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}