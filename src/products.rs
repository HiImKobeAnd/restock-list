use std::collections::HashMap;

use askama::Template;

#[derive(Template, Clone)]
#[template(path = "products.html")]
pub struct Products {
    pub products: Vec<Product>,
}

#[derive(Clone)]
pub struct Product {
    pub name: String,
    pub start_value: i32,
    pub required_amount: i32,
    pub list_sort_number: i32,
    pub restock_sort_number: i32,
    pub daily_use: i32,
}

pub fn get_products() -> Products {
    let onsdags_snegl = Product {
        name: String::from("Onsdags_snegl"),
        start_value: 3,
        required_amount: 10,
        list_sort_number: 30,
        restock_sort_number: 0,
        daily_use: 0,
    };
    let kaernemaelkshorn = Product {
        name: String::from("Kernemelkshorn"),
        start_value: 2,
        required_amount: 2,
        list_sort_number: 32,
        restock_sort_number: 1,
        daily_use: 0,
    };
    let skagenshorn = Product {
        name: String::from("Skagenshorn"),
        start_value: 2,
        required_amount: 2,
        list_sort_number: 31,
        restock_sort_number: 2,
        daily_use: 0,
    };
    let kanelstang = Product {
        name: String::from("Kanelstang"),
        start_value: 5,
        required_amount: 8,
        list_sort_number: 19,
        restock_sort_number: 3,
        daily_use: 1,
    };
    let lille_poelsehorn = Product {
        name: String::from("Smo_polsehorn"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 10,
        restock_sort_number: 4,
        daily_use: 0,
    };
    let cremestang = Product {
        name: String::from("Cremestang"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 25,
        restock_sort_number: 5,
        daily_use: 0,
    };
    let kagemand_stang = Product {
        name: String::from("Kagemand_stang"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 20,
        restock_sort_number: 6,
        daily_use: 0,
    };
    let brunsvigerstang = Product {
        name: String::from("Brunsvigerstang"),
        start_value: 6,
        required_amount: 6,
        list_sort_number: 17,
        restock_sort_number: 7,
        daily_use: 0,
    };
    let hoej_snegl = Product {
        name: String::from("Hoj_snegl"),
        start_value: 4,
        required_amount: 8,
        list_sort_number: 8,
        restock_sort_number: 8,
        daily_use: 1,
    };
    let pizza_pepperoni = Product {
        name: String::from("Pizza_pepperoni"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 12,
        restock_sort_number: 9,
        daily_use: 0,
    };
    let brunsviger_knude = Product {
        name: String::from("Brunsviger_knude"),
        start_value: 6,
        required_amount: 8,
        list_sort_number: 24,
        restock_sort_number: 10,
        daily_use: 0,
    };
    let choco_donut = Product {
        name: String::from("Confetii_donut"),
        start_value: 2,
        required_amount: 3,
        list_sort_number: 3,
        restock_sort_number: 11,
        daily_use: 0,
    };
    let kakaocroissaint = Product {
        name: String::from("Kakaocroissaint"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 22,
        restock_sort_number: 12,
        daily_use: 0,
    };
    let pink_donut = Product {
        name: String::from("Pink_donut"),
        start_value: 2,
        required_amount: 3,
        list_sort_number: 1,
        restock_sort_number: 13,
        daily_use: 0,
    };
    let pizzasnegl = Product {
        name: String::from("Pizzasnegl"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 14,
        restock_sort_number: 14,
        daily_use: 0,
    };
    let sprinkled_donut = Product {
        name: String::from("Sprinkled_donut"),
        start_value: 2,
        required_amount: 3,
        list_sort_number: 2,
        restock_sort_number: 15,
        daily_use: 0,
    };
    let birkes = Product {
        name: String::from("Birkes"),
        start_value: 3,
        required_amount: 4,
        list_sort_number: 6,
        restock_sort_number: 16,
        daily_use: 0,
    };
    let grovbirkes = Product {
        name: String::from("Grovbirkes"),
        start_value: 3,
        required_amount: 4,
        list_sort_number: 5,
        restock_sort_number: 17,
        daily_use: 0,
    };
    let kanelsnurre = Product {
        name: String::from("Kanelsnurre"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 15,
        restock_sort_number: 18,
        daily_use: 0,
    };
    let kbh_birkes = Product {
        name: String::from("KBH_birkes"),
        start_value: 5,
        required_amount: 8,
        list_sort_number: 7,
        restock_sort_number: 19,
        daily_use: 0,
    };
    let snurre_med_remonce = Product {
        name: String::from("Snurre_med_remonce"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 16,
        restock_sort_number: 20,
        daily_use: 0,
    };
    let cremebolle = Product {
        name: String::from("Cremebolle"),
        start_value: 2,
        required_amount: 4,
        list_sort_number: 33,
        restock_sort_number: 21,
        daily_use: 0,
    };
    let poelsehorn_med_kylling = Product {
        name: String::from("Polsehorn_med_kylling"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 9,
        restock_sort_number: 22,
        daily_use: 0,
    };
    let poelsehorn_med_ketchup = Product {
        name: String::from("Polsehorn_med_ketchup"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 11,
        restock_sort_number: 23,
        daily_use: 0,
    };
    let macaron = Product {
        name: String::from("Macaron"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 4,
        restock_sort_number: 24,
        daily_use: 0,
    };
    let morgenstang = Product {
        name: String::from("Morgenstang"),
        start_value: 2,
        required_amount: 8,
        list_sort_number: 18,
        restock_sort_number: 25,
        daily_use: 2,
    };
    let chokolade_croissaint = Product {
        name: String::from("Chokolade_croissaint"),
        start_value: 2,
        required_amount: 2,
        list_sort_number: 21,
        restock_sort_number: 26,
        daily_use: 0,
    };
    let croissaint = Product {
        name: String::from("Crossaint"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 23,
        restock_sort_number: 27,
        daily_use: 0,
    };
    let products = [
        onsdags_snegl,
        kaernemaelkshorn,
        skagenshorn,
        kanelstang,
        lille_poelsehorn,
        cremestang,
        kagemand_stang,
        brunsvigerstang,
        hoej_snegl,
        pizza_pepperoni,
        brunsviger_knude,
        choco_donut,
        kakaocroissaint,
        pink_donut,
        pizzasnegl,
        sprinkled_donut,
        birkes,
        grovbirkes,
        kanelsnurre,
        kbh_birkes,
        snurre_med_remonce,
        cremebolle,
        poelsehorn_med_kylling,
        poelsehorn_med_ketchup,
        macaron,
        morgenstang,
        chokolade_croissaint,
        croissaint,
    ];

    Products {
        products: products.to_vec(),
    }
}

pub fn get_products_map() -> HashMap<String, Product> {
    let products = get_products();

    let mut map = HashMap::new();
    for product in products.products {
        map.insert(product.name.clone(), product);
    }

    map
}
