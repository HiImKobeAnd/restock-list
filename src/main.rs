mod products;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;
use products::get_items;
use products::{Product, Products};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Item {
    #[serde(rename = "type")]
    name: String,
    quantity: i32,
}

#[get("/")]
async fn index() -> impl Responder {
    let items = get_items();
    HttpResponse::Ok().body(items.render().unwrap())
}

#[post("/value_changed")]
async fn value_changed(form: web::Form<Vec<Item>>) -> impl Responder {
    let items = get_items().products;
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

#[post("/sort_count")]
async fn sort_count(form: web::Form<Vec<Item>>) -> impl Responder {
    let items = get_items().products;
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

#[post("/sort_restock")]
async fn sort_restock(form: web::Form<Vec<Item>>) -> impl Responder {
    let items = get_items().products;
    let mut vec = Vec::new();
    for item in items.clone() {
        vec.push((item.name, item.restock_sort_number));
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

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(value_changed)
            .service(sort_count)
            .service(sort_restock)
    })
    .bind(("127.0.0.1", 8050))?
    .workers(4)
    .run()
    .await
}
