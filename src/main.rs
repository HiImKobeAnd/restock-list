use actix_web::{web, App, HttpServer};
use restock_list::configuration::get_configuration;
use restock_list::endpoints::send_products_data;
use restock_list::endpoints::{index::index, sort::sort};

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
            .service(sort)
            .service(send_products_data)
    })
    .bind(("127.0.0.1", configuration.application_port))?
    .workers(2)
    .run()
    .await
}
