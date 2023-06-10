
use actix_web::{web, App, HttpServer, Responder,HttpResponse};



pub async fn health_check() -> impl Responder{
    HttpResponse::Ok().finish()
}