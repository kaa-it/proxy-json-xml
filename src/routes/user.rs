use actix_web::{web, Responder, HttpResponse};
use anyhow::Result;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserInput {
    pub first_name: String,
    pub second_name: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/user").route(web::post().to(user)));
}

pub async fn user(user: web::Json<UserInput>) -> impl Responder {
    println!("User: {:?}", user);
    HttpResponse::Ok().json(format!("Welcome {} {}!", user.first_name, user.second_name))
}