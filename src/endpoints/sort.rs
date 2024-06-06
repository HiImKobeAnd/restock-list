use crate::data::{AppData, Item};
use crate::products::{Product, Products};
use actix_web::{post, web, HttpResponse, Responder};
use askama::Template;
use std::collections::HashMap;

#[derive(Debug)]
enum SortMethod {
    Restock,
    Shelf,
}

impl TryFrom<&str> for SortMethod {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "restock" => Ok(SortMethod::Restock),
            "shelf" => Ok(SortMethod::Shelf),
            _ => Err("Unknown sorting method"),
        }
    }
}

#[post("/sort/{sorting_method}")]
pub async fn sort(
    form: web::Form<Vec<Item>>,
    data: web::Data<AppData>,
    path: web::Path<String>,
) -> impl Responder {
    let products_map = data.products_map.clone();
    let products = data.products.products.clone();
    let sort_method_string = path.into_inner();

    let sort_method =
        SortMethod::try_from(sort_method_string.as_str()).unwrap_or(SortMethod::Shelf);

    let mut vec = Vec::new();
    for product in products.clone() {
        match sort_method {
            SortMethod::Restock => vec.push((product.name, product.list_sort_number)),
            SortMethod::Shelf => vec.push((product.name, product.restock_sort_number)),
        }
    }

    vec.sort_by(|a, b| a.1.cmp(&b.1));

    let mut form_map = HashMap::new();
    for product in form.iter() {
        form_map.insert(product.name.clone(), product.quantity);
    }

    let mut sorted_product = Vec::new();
    for product in vec {
        sorted_product.push(Product {
            name: product.0.clone(),
            start_value: *form_map.get(&product.0).unwrap(),
            required_amount: products_map.get(&product.0).unwrap().required_amount,
            list_sort_number: 0,
            restock_sort_number: 0,
            daily_use: 0,
        });
    }

    let sorted_products = Products {
        products: sorted_product,
    };

    HttpResponse::Ok().body(sorted_products.render().unwrap())
}
