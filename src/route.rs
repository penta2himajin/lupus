use actix_web::{HttpResponse, Responder};

pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
    
pub async fn index2() -> impl Responder {
    HttpResponse::Ok().body("Hello world again!")
}
    
pub async fn cat() -> impl Responder {
    HttpResponse::Ok().body("NyanCat")
}

pub async fn authorization() -> impl Responder {
    HttpResponse::Ok().body("")
}