use actix_web::{web, App, HttpServer};
use restock_list::endpoints::{
    index::index, product_count_changed::product_count_changed, sort_by_restock::sort_by_restock,
    sort_by_shelf::sort_by_shelf,
};

use restock_list::{data::AppData, products::get_items};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let products = get_items();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppData {
                products: products.clone(),
            }))
            .service(index)
            .service(product_count_changed)
            .service(sort_by_shelf)
            .service(sort_by_restock)
    })
    .bind(("127.0.0.1", 8050))?
    .workers(2)
    .run()
    .await
}
