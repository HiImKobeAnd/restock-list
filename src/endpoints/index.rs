use crate::data::AppData;
use actix_web::{get, web, HttpResponse, Responder};
use askama::Template;

#[get("/")]
pub async fn index(data: web::Data<AppData>) -> impl Responder {
    let items = &(*data.into_inner()).products;
    HttpResponse::Ok().body(items.render().unwrap())
}
