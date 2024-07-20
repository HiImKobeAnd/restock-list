use crate::data::AppData;
use actix_web::{get, web, Responder};

#[get("/get_data")]
pub async fn send_products_data(data: web::Data<AppData>) -> impl Responder {
    let data = data.as_ref();

    web::Json(serde_json::json!(data.products))
}
