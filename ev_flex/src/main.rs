use actix_web::{get, post, web, web::Data, App, HttpServer, Responder};
use chrono::prelude::{DateTime, Utc};
use chrono::Duration;
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
    time: DateTime<Utc>,     // time
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

struct MinuteDateRange(DateTime<Utc>, DateTime<Utc>);

impl Iterator for MinuteDateRange {
    type Item = DateTime<Utc>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::minutes(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

fn create_flex_series(demand: &EnergyDemand) -> Option<Vec<AggregationDT>> {
    let mut aggregation_series = vec![];
    let one_percent_energy = demand.capacity / 100;
    let energy_demand = (demand.target_soc - demand.current_soc) * one_percent_energy;
    let critical_demand = match demand.min_soc > demand.current_soc {
        true => (demand.min_soc - demand.current_soc) * one_percent_energy,
        false => 0,
    };
    let time_to_charge_max_soe = (energy_demand * 60 / demand.max_charging_power) as i64;
    let time_to_charge_critical_soe = (critical_demand * 60 / demand.max_charging_power) as i64;
    let one_minute_energy_state_change = demand.max_charging_power / 60;

    let asap_charge_time_end = demand.start + Duration::minutes(time_to_charge_max_soe);
    let alap_charge_time_start = demand.end - Duration::minutes(time_to_charge_max_soe)
        + Duration::minutes(time_to_charge_critical_soe);

    if asap_charge_time_end > demand.end {
        println!("UNFEASIBLE DEMAND");
        return None;
    }

    let mut max_soe_state = demand.current_soc * one_percent_energy;
    let mut min_soe_state = demand.current_soc * one_percent_energy;

    let min_soe = demand.min_soc * one_percent_energy;

    // Fill asap line
    for time in MinuteDateRange(demand.start, asap_charge_time_end) {
        let aggregation_dt = AggregationDT {
            min_soe: min_soe_state,
            max_soe: max_soe_state,
            max_charging_power: demand.max_charging_power,
            time: time,
        };

        aggregation_series.push(aggregation_dt);

        max_soe_state += one_minute_energy_state_change;

        if min_soe_state + one_minute_energy_state_change < min_soe {
            min_soe_state += one_minute_energy_state_change;
        }
    }

    // Fill middle area
    for time in MinuteDateRange(asap_charge_time_end, alap_charge_time_start) {
        aggregation_series.push(AggregationDT {
            min_soe: min_soe_state,
            max_soe: max_soe_state,
            max_charging_power: demand.max_charging_power,
            time: time,
        });
    }

    // Fill alap line
    for time in MinuteDateRange(alap_charge_time_start, demand.end) {
        aggregation_series.push(AggregationDT {
            min_soe: min_soe_state,
            max_soe: max_soe_state,
            max_charging_power: demand.max_charging_power,
            time: time,
        });
        min_soe_state += one_minute_energy_state_change;
    }

    Some(aggregation_series)
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
        aggregation.start = std::cmp::min(aggregation.start, demand.start);
        aggregation.end = std::cmp::max(aggregation.end, demand.end);
        if let Some(mut series) = create_flex_series(demand) {
            aggregation.series.append(&mut series);
        }
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
