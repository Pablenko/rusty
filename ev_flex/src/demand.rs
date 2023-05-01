use chrono::prelude::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
pub struct EnergyDemand {
    pub vehicle_id: String,
    pub min_soc: i32,            // minimum state of charge in percent
    pub max_soc: i32,            // maximum state of charge in percent
    pub target_soc: i32,         // target state of charge in percent
    pub current_soc: i32,        // current state of charge in percent
    pub capacity: i32,           // capacity in Wh
    pub max_charging_power: i32, // maximum charging power in W
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

pub struct Demands {
    pub demands: Mutex<Vec<EnergyDemand>>,
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
