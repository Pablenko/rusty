use crate::demand::EnergyDemand;
use crate::utils::MinuteDateRange;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AggregationDT {
    pub min_soe: i32,            // minimum state of charge in Wh
    pub max_soe: i32,            // maximum state of charge in Wh
    pub max_charging_power: i32, // maximum charging power in W
    pub time: DateTime<Utc>,     // time
}

#[derive(Serialize, Deserialize)]
pub struct Aggregation {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub series: Vec<AggregationDT>,
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

pub fn create_flex_series(demand: &EnergyDemand) -> Option<Vec<AggregationDT>> {
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
