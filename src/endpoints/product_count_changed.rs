use crate::data::{AppData, Item};
use actix_web::{post, web, HttpResponse, Responder};
use std::borrow::Borrow;

#[post("/product_count_changed")]
pub async fn product_count_changed(
    form: web::Form<Vec<Item>>,
    data: web::Data<AppData>,
) -> impl Responder {
    let data = data.borrow();
    let products_map = data.products_map.clone();

    let form_product = form.first().unwrap();
    let product = products_map.get(&form.first().unwrap().name).unwrap();

    HttpResponse::Ok().body(format!(
        "<p class=\"table_number\" id=\"purchase_amount_{}\">{}</p>",
        form_product.name,
        ((product.daily_use * 2 + product.required_amount) - form_product.quantity)
            .clamp(0, product.required_amount + product.daily_use)
    ))
}
