use actix_web::{web, App, HttpServer};
use restock_list::endpoints::{
    index::index, product_count_changed::product_count_changed, sort::sort,
};

use restock_list::products::get_products_map;
use restock_list::{data::AppData, products::get_products};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let products = get_products();
    let products_map = get_products_map();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData {
                products: products.clone(),
                products_map: products_map.clone(),
            }))
            .service(index)
            .service(product_count_changed)
            .service(sort)
    })
    .bind(("127.0.0.1", 8050))?
    .workers(2)
    .run()
    .await
}
