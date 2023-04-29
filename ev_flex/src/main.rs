use actix_web::{web, App, HttpServer, Responder, web::{Data}, post, get};
use chrono::prelude::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;


#[derive(Serialize, Deserialize)]
struct EnergyDemand {
    vehicle_id: String,
    target: i32,
    current: i32,
    soc: f32,
    max_charging_power: f32,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
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
async fn handle_energy_demand(db: Data<Demands>, new_demand: web::Json<EnergyDemand>) -> impl Responder {
    println!("{}", serde_json::to_string_pretty(&new_demand).unwrap());
    let response = format!("Received demand for {}!", new_demand.vehicle_id);
    db.insert(new_demand.into_inner());
    response
}

#[get("/aggregation")]
async fn handle_aggregation_request(db: Data<Demands>) -> impl Responder {
    let demands = db.demands.lock().unwrap();
    let mut aggregation_sum: i32 = 0;
    for demand in demands.iter() {
        aggregation_sum += demand.target;
    }
    format!("aggregation: {}", aggregation_sum)
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