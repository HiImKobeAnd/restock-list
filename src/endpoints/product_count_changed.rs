use crate::data::{AppData, Item};
use actix_web::{post, web, HttpResponse, Responder};
use std::collections::HashMap;

#[post("/product_count_changed")]
pub async fn product_count_changed(
    form: web::Form<Vec<Item>>,
    data: web::Data<AppData>,
) -> impl Responder {
    let items = (*data.into_inner()).products.clone().products;
    let mut map = HashMap::new();
    for item in items {
        map.insert(item.name, item.required_amount);
    }

    let value = map.get(&form.first().unwrap().name).unwrap();

    HttpResponse::Ok().body(format!(
        "<p class=\"table_number\" id=\"purchase_amount_{}\">{}</p>",
        form.first().unwrap().name,
        *value as i32 - form.first().unwrap().quantity as i32
    ))
}
