use actix_web::{get, post, web, web::Data, App, HttpServer, Responder};
use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct EnergyDemand {
    vehicle_id: String,
    min_soc: i32,            // minimum state of charge in percent
    max_soc: i32,            // maximum state of charge in percent
    target_soc: i32,         // target state of charge in percent
    current_soc: i32,        // current state of charge in percent
    capacity: i32,           // capacity in Wh
    max_charging_power: i32, // maximum charging power in W
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
struct AggregationDT {
    min_soe: i32,            // minimum state of charge in Wh
    max_soe: i32,            // maximum state of charge in Wh
    max_charging_power: i32, // maximum charging power in W
}

#[derive(Serialize, Deserialize)]
struct Aggregation {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
    series: Vec<AggregationDT>,
}

impl Default for Aggregation {
    fn default() -> Self {
        Self {
            start: Utc::now(),
            end: Utc::now(),
            series: vec![],
        }
    }
}

struct Demands {
    demands: Mutex<Vec<EnergyDemand>>,
}

impl Demands {
    pub fn new() -> Self {
        let demands = Mutex::new(vec![]);
        Demands { demands }
    }

    pub fn insert(&self, demand: EnergyDemand) {
        let mut demands = self.demands.lock().unwrap();
        demands.push(demand);
    }
}

#[post("/demand")]
async fn handle_energy_demand(
    db: Data<Demands>,
    new_demand: web::Json<EnergyDemand>,
) -> impl Responder {
    println!("{}", serde_json::to_string_pretty(&new_demand).unwrap());
    let response = format!("Received demand for {}!", new_demand.vehicle_id);
    db.insert(new_demand.into_inner());
    response
}

#[get("/aggregation")]
async fn handle_aggregation_request(db: Data<Demands>) -> impl Responder {
    let demands = db.demands.lock().unwrap();
    let mut aggregation: Aggregation = Aggregation::default();
    for demand in demands.iter() {
        aggregation.series.push(AggregationDT {
            min_soe: demand.min_soc,
            max_soe: demand.max_soc,
            max_charging_power: demand.max_charging_power,
        });
    }
    serde_json::to_string(&aggregation)
}

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
