use crate::data::{AppData, Item};
use crate::products::{Product, Products};
use actix_web::{post, web, HttpResponse, Responder};
use askama::Template;
use std::collections::HashMap;

#[post("/sort_by_shelf")]
pub async fn sort_by_shelf(form: web::Form<Vec<Item>>, data: web::Data<AppData>) -> impl Responder {
    let items = (*data.into_inner()).products.clone().products;
    let mut vec = Vec::new();
    for item in items.clone() {
        vec.push((item.name, item.list_sort_number));
    }
    vec.sort_by(|a, b| a.1.cmp(&b.1));

    let mut form_map = HashMap::new();
    for item in form.iter() {
        form_map.insert(item.name.clone(), item.quantity);
    }

    let mut product_map = HashMap::new();
    for item in items {
        product_map.insert(item.name, item.required_amount);
    }

    let mut sorted_product = Vec::new();
    for item in vec {
        sorted_product.push(Product {
            name: item.0.clone(),
            start_value: *form_map.get(&item.0).unwrap(),
            required_amount: *product_map.get(&item.0).unwrap(),
            list_sort_number: 0,
            restock_sort_number: 0,
        });
    }

    let sorted_products = Products {
        products: sorted_product,
    };

    HttpResponse::Ok().body(sorted_products.render().unwrap())
}
