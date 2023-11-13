use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
use crate::db;
use crate::db::models::User;

#[get("api/user/{id}")]
async fn user(name: web::Path<String>) -> HttpResponse {
    let users: Vec<User> = db::get_users().await;
    for current in users {
        if current.tg_id.to_string() == name.to_string() {
            return HttpResponse::Ok().body(serde_json::to_string(&current).expect("Can't \
            serialize user"));
        }
    }
    HttpResponse::Accepted().finish()
}

#[get("api/models")]
async fn models() -> HttpResponse {
    let models = &db::get_models().await;
    let json = serde_json::to_string(models).unwrap();
    HttpResponse::Ok().body(json)
}

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(user).service(models))
        .bind(("127.0.0.1", 3001))?
        .run()
        .await
}