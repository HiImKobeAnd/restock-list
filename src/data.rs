use std::collections::HashMap;

use crate::products::{Product, Products};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Item {
    #[serde(rename = "type")]
    pub name: String,
    pub quantity: i32,
}

pub struct AppData {
    pub products: Products,
    pub products_map: HashMap<String, Product>,
}
