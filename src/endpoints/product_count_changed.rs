use crate::data::{AppData, Item};
use actix_web::{post, web, HttpResponse, Responder};
use std::borrow::Borrow;

#[post("/product_count_changed")]
pub async fn product_count_changed(
    form: web::Form<Vec<Item>>,
    data: web::Data<AppData>,
) -> impl Responder {
    let data = data.borrow();
    let map = data.products_map.clone();

    let product = map.get(&form.first().unwrap().name).unwrap();

    HttpResponse::Ok().body(format!(
        "<p class=\"table_number\" id=\"purchase_amount_{}\">{}</p>",
        form.first().unwrap().name,
        (((product.required_amount * 2) + product.daily_use) - form.first().unwrap().quantity)
            .clamp(0, product.required_amount + product.daily_use)
    ))
}
