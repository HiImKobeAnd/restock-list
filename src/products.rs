use askama::Template;

#[derive(Template, Clone)]
#[template(path = "products.html")]
pub struct Products {
    pub products: Vec<Product>,
}

#[derive(Clone)]
pub struct Product {
    pub name: String,
    pub start_value: u16,
    pub required_amount: u16,
}

pub fn get_items() -> Products {
    let kanelstang = Product {
        name: String::from("Kanelstang"),
        start_value: 5,
        required_amount: 8,
    };
    let morgenstang = Product {
        name: String::from("Morgenstang"),
        start_value: 2,
        required_amount: 8,
    };
    let brunsvigerstang = Product {
        name: String::from("Brunsvigerstang"),
        start_value: 7,
        required_amount: 8,
    };
    let cremestang = Product {
        name: String::from("Cremestang"),
        start_value: 3,
        required_amount: 3,
    };
    let kakaocroissaint = Product {
        name: String::from("Kakaocroissaint"),
        start_value: 3,
        required_amount: 3,
    };
    let hoej_snegl = Product {
        name: String::from("Høj snegl"),
        start_value: 4,
        required_amount: 8,
    };
    let croissaint = Product {
        name: String::from("Crossaint"),
        start_value: 4,
        required_amount: 4,
    };
    let chokolade_croissaint = Product {
        name: String::from("Chokolade roissaint"),
        start_value: 2,
        required_amount: 2,
    };
    let kbh_birkes = Product {
        name: String::from("KBH birkes"),
        start_value: 5,
        required_amount: 8,
    };
    let birkes = Product {
        name: String::from("Birkes"),
        start_value: 3,
        required_amount: 4,
    };
    let grovbirkes = Product {
        name: String::from("Grovbirkes"),
        start_value: 3,
        required_amount: 4,
    };
    let onsdags_snegl = Product {
        name: String::from("Onsdags snegl"),
        start_value: 3,
        required_amount: 10,
    };
    let kanelsnurre = Product {
        name: String::from("Kanelsnurre"),
        start_value: 4,
        required_amount: 4,
    };
    let snurre_med_remonce = Product {
        name: String::from("Snurre med remonce"),
        start_value: 4,
        required_amount: 4,
    };
    let brunsviger_knude = Product {
        name: String::from("Brunsviger knude"),
        start_value: 6,
        required_amount: 8,
    };
    let skagenshorn = Product {
        name: String::from("Skagenshorn"),
        start_value: 2,
        required_amount: 2,
    };
    let kaernemaelkshorn = Product {
        name: String::from("Kærnemælkshorn"),
        start_value: 2,
        required_amount: 2,
    };
    let cremebolle = Product {
        name: String::from("Cremebolle"),
        start_value: 2,
        required_amount: 4,
    };
    let choco_donut = Product {
        name: String::from("Choco donut"),
        start_value: 2,
        required_amount: 3,
    };
    let rainbow_donut = Product {
        name: String::from("Rainbow donut"),
        start_value: 2,
        required_amount: 3,
    };
    let pink_donut = Product {
        name: String::from("Pink donut"),
        start_value: 2,
        required_amount: 3,
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
        ]
        .to_vec(),
    };
    products
}
