use actix_web::{HttpResponse, Responder};

pub async fn publish_newsletter_admin() -> impl Responder {
    HttpResponse::Ok()
}
