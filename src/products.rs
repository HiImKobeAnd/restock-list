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
}

pub fn get_items() -> Products {
    let kanelstang = Product {
        name: String::from("Kanelstang"),
        start_value: 5,
        required_amount: 8,
        list_sort_number: 19,
        restock_sort_number: 100,
    };
    let morgenstang = Product {
        name: String::from("Morgenstang"),
        start_value: 2,
        required_amount: 8,
        list_sort_number: 18,
        restock_sort_number: 100,
    };
    let brunsvigerstang = Product {
        name: String::from("Brunsvigerstang"),
        start_value: 7,
        required_amount: 8,
        list_sort_number: 17,
        restock_sort_number: 100,
    };
    let cremestang = Product {
        name: String::from("Cremestang"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 25,
        restock_sort_number: 100,
    };
    let kakaocroissaint = Product {
        name: String::from("Kakaocroissaint"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 22,
        restock_sort_number: 100,
    };
    let hoej_snegl = Product {
        name: String::from("Høj_snegl"),
        start_value: 4,
        required_amount: 8,
        list_sort_number: 8,
        restock_sort_number: 100,
    };
    let croissaint = Product {
        name: String::from("Crossaint"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 23,
        restock_sort_number: 100,
    };
    let chokolade_croissaint = Product {
        name: String::from("Chokolade_croissaint"),
        start_value: 2,
        required_amount: 2,
        list_sort_number: 21,
        restock_sort_number: 100,
    };
    let kbh_birkes = Product {
        name: String::from("KBH_birkes"),
        start_value: 5,
        required_amount: 8,
        list_sort_number: 7,
        restock_sort_number: 100,
    };
    let birkes = Product {
        name: String::from("Birkes"),
        start_value: 3,
        required_amount: 4,
        list_sort_number: 6,
        restock_sort_number: 100,
    };
    let grovbirkes = Product {
        name: String::from("Grovbirkes"),
        start_value: 3,
        required_amount: 4,
        list_sort_number: 5,
        restock_sort_number: 100,
    };
    let onsdags_snegl = Product {
        name: String::from("Onsdags_snegl"),
        start_value: 3,
        required_amount: 10,
        list_sort_number: 30,
        restock_sort_number: 100,
    };
    let kanelsnurre = Product {
        name: String::from("Kanelsnurre"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 15,
        restock_sort_number: 100,
    };
    let snurre_med_remonce = Product {
        name: String::from("Snurre_med_remonce"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 16,
        restock_sort_number: 100,
    };
    let brunsviger_knude = Product {
        name: String::from("Brunsviger_knude"),
        start_value: 6,
        required_amount: 8,
        list_sort_number: 24,
        restock_sort_number: 100,
    };
    let skagenshorn = Product {
        name: String::from("Skagenshorn"),
        start_value: 2,
        required_amount: 2,
        list_sort_number: 31,
        restock_sort_number: 100,
    };
    let kaernemaelkshorn = Product {
        name: String::from("Kærnemælkshorn"),
        start_value: 2,
        required_amount: 2,
        list_sort_number: 32,
        restock_sort_number: 100,
    };
    let cremebolle = Product {
        name: String::from("Cremebolle"),
        start_value: 2,
        required_amount: 4,
        list_sort_number: 33,
        restock_sort_number: 100,
    };
    let choco_donut = Product {
        name: String::from("Choco_donut"),
        start_value: 2,
        required_amount: 3,
        list_sort_number: 3,
        restock_sort_number: 100,
    };
    let rainbow_donut = Product {
        name: String::from("Rainbow_donut"),
        start_value: 2,
        required_amount: 3,
        list_sort_number: 2,
        restock_sort_number: 100,
    };
    let pink_donut = Product {
        name: String::from("Pink_donut"),
        start_value: 2,
        required_amount: 3,
        list_sort_number: 1,
        restock_sort_number: 100,
    };
    let macaron = Product {
        name: String::from("Macaron"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 4,
        restock_sort_number: 100,
    };
    let stor_poelsehorn_med_ketchup = Product {
        name: String::from("Stor_pølsehorn_med_ketchup"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 11,
        restock_sort_number: 100,
    };
    let stor_poelsehorn_med_kylling = Product {
        name: String::from("Stor_pølsehorn_med_kylling"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 9,
        restock_sort_number: 100,
    };
    let lille_poelsehorn = Product {
        name: String::from("Lille_pølsehorn"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 10,
        restock_sort_number: 100,
    };
    let pizzasnegl = Product {
        name: String::from("Pizzasnegl"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 14,
        restock_sort_number: 100,
    };
    let pizza_pepperoni = Product {
        name: String::from("Pizza_pepperoni"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 12,
        restock_sort_number: 100,
    };
    let pizza_donut = Product {
        name: String::from("Pizza_donut"),
        start_value: 4,
        required_amount: 4,
        list_sort_number: 13,
        restock_sort_number: 100,
    };
    let wiener_kagemand_stang = Product {
        name: String::from("Wiener_kagemand_stang"),
        start_value: 3,
        required_amount: 3,
        list_sort_number: 20,
        restock_sort_number: 100,
    };
    let products = Products {
        products: [
            kanelstang,
            morgenstang,
            brunsvigerstang,
            cremestang,
            kakaocroissaint,
            hoej_snegl,
            croissaint,
            pink_donut,
            rainbow_donut,
            choco_donut,
            cremebolle,
            kaernemaelkshorn,
            skagenshorn,
            brunsviger_knude,
            snurre_med_remonce,
            kanelsnurre,
            onsdags_snegl,
            grovbirkes,
            birkes,
            kbh_birkes,
            chokolade_croissaint,
            macaron,
            stor_poelsehorn_med_ketchup,
            stor_poelsehorn_med_kylling,
            lille_poelsehorn,
            pizzasnegl,
            pizza_pepperoni,
            pizza_donut,
            wiener_kagemand_stang,
        ]
        .to_vec(),
    };
    products
}
