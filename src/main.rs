mod products;

use std::collections::HashMap;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use askama::Template;

use products::get_items;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Item {
    value: i32,
}

#[get("/")]
async fn index() -> impl Responder {
    let items = get_items();
    HttpResponse::Ok().body(items.render().unwrap())
}

#[post("/value_changed/{name}")]
async fn value_changed(path: web::Path<String>, form: web::Form<Item>) -> impl Responder {
    let items = get_items().products;
    let mut map = HashMap::new();
    for item in items {
        map.insert(item.name, item.required_amount);
    }

    let name = path.into_inner();

    let value = map.get(&name).unwrap();

    HttpResponse::Ok().body(format!(
        "<p class=\"table_number\" id=\"purchase_amount_{name}\">{}</p>",
        *value as i32 - form.value as i32
    ))
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(value_changed))
        .bind(("127.0.0.1", 8050))?
        .workers(4)
        .run()
        .await
}
