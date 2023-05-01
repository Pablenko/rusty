use crate::api::{handle_aggregation_request, handle_energy_demand};
use crate::demand::Demands;
use actix_web::{web::Data, App, HttpServer};

mod aggregation;
mod api;
mod demand;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = Data::new(Demands::new());
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(handle_energy_demand)
            .service(handle_aggregation_request)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
