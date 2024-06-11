use actix_web::{web, App, HttpServer};
use restock_list::configuration::get_configuration;
use restock_list::endpoints::{
    index::index, product_count_changed::product_count_changed, sort::sort,
};

use restock_list::data::AppData;
use restock_list::products::{get_products_map, import_products_from_file};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let products = import_products_from_file();
    let products_map = get_products_map(products.clone());

    let configuration = get_configuration().expect("Failed to get configuration.");

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
    .bind(("127.0.0.1", configuration.application_port))?
    .workers(2)
    .run()
    .await
}
