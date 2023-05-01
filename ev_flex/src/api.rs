use crate::aggregation::{create_flex_series, resample_series, Aggregation};
use crate::demand::{Demands, EnergyDemand};
use actix_web::{get, post, web, web::Data, Responder};

#[post("/demand")]
pub async fn handle_energy_demand(
    db: Data<Demands>,
    new_demand: web::Json<EnergyDemand>,
) -> impl Responder {
    println!("{}", serde_json::to_string_pretty(&new_demand).unwrap());
    let response = format!("Received demand for {}!", new_demand.vehicle_id);
    db.insert(new_demand.into_inner());
    response
}

#[get("/aggregation")]
pub async fn handle_aggregation_request(db: Data<Demands>) -> impl Responder {
    let demands = db.demands.lock().unwrap();
    let mut aggregation: Aggregation = Aggregation::default();
    for demand in demands.iter() {
        aggregation.start = std::cmp::min(aggregation.start, demand.start);
        aggregation.end = std::cmp::max(aggregation.end, demand.end);
        if let Some(mut series) = create_flex_series(demand) {
            aggregation.series.append(&mut series);
        }
    }
    resample_series(&mut aggregation.series, 15);
    serde_json::to_string(&aggregation)
}
