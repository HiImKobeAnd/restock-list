use std::{collections::HashMap, fs};

use askama::Template;

#[derive(Template, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[template(path = "products.html")]
pub struct Products {
    pub products: Vec<Product>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Product {
    pub name: String,
    pub start_value: i32,
    pub required_amount: i32,
    pub shelf_sort_number: i32,
    pub restock_sort_number: i32,
    pub daily_use: i32,
}

pub fn get_products_map(products: Products) -> HashMap<String, Product> {
    let mut map = HashMap::new();
    for product in products.products {
        map.insert(product.name.clone(), product);
    }

    map
}

pub fn export_products_to_file(products: Products) {
    let products_yaml =
        serde_yml::to_string(&products).expect("Failed to serialize products to YAML");
    fs::write("products.yaml", products_yaml).expect("Failed to write products.yaml");
}

pub fn import_products_from_file() -> Products {
    let products_string =
        fs::read_to_string("products.yaml").expect("Failed to read products.yaml");
    serde_yml::from_str(products_string.as_str()).expect("Failed to deserialize products from YAML")
}
