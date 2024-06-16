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

    let sort_method = SortMethod::try_from(path.into_inner().as_str()).unwrap_or(SortMethod::Shelf);

    let mut unsorted_products = Vec::new();
    for product in products {
        match sort_method {
            SortMethod::Restock => {
                unsorted_products.push((product.name, product.restock_sort_number))
            }
            SortMethod::Shelf => unsorted_products.push((product.name, product.shelf_sort_number)),
        }
    }

    unsorted_products.sort_by(|a, b| a.1.cmp(&b.1));

    let mut form_map = HashMap::new();
    for product in form.iter() {
        form_map.insert(product.name.clone(), product.quantity);
    }

    let mut sorted_products = Vec::new();
    for product in unsorted_products {
        let product_name = product.0;
        let product = products_map.get(&product_name).unwrap();
        sorted_products.push(Product {
            name: product_name.clone(),
            start_value: *form_map.get(&product_name).unwrap(),
            required_amount: product.required_amount,
            shelf_sort_number: product.shelf_sort_number,
            restock_sort_number: product.restock_sort_number,
            daily_use: product.daily_use,
        });
    }

    let sorted_products = Products {
        products: sorted_products,
    };

    HttpResponse::Ok().body(sorted_products.render().unwrap())
}
