use chrono::DateTime;
use surrealdb::sql::Thing;
use serde::{Serialize, Deserialize};

pub struct InputContact {
    pub callsign: String,
    pub name: String,
    pub frequency_hz: u64,
    pub mode: String,
    pub grid: String,
    pub distance: String,
    pub t_pwr: String,
    pub r_pwr: String,
    pub t_rst: String,
    pub r_rst: String,
    pub date_time_utc: String,
    pub notes: String
}

impl Default for InputContact {
    fn default() -> Self {
        Self {
            callsign: String::new(),
            name: String::new(),
            frequency_hz: 0,
            mode: String::new(),
            grid: String::new(),
            distance: String::new(),
            t_pwr: String::new(),
            r_pwr: String::new(),
            t_rst: String::new(),
            r_rst: String::new(),
            date_time_utc: String::new(),
            notes: String::new()
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Option<Thing>,
    pub callsign: Option<String>,
    pub name: Option<String>,
    pub frequency: Option<u64>,
    pub mode: Option<String>,
    pub grid: Option<String>,
    pub distance: Option<String>,
    pub t_pwr: Option<String>,
    pub r_pwr: Option<String>,
    pub t_rst: Option<String>,
    pub r_rst: Option<String>,
    pub date_time_utc: Option<String>,
    pub notes: Option<String>
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            id: None,
            callsign: None,
            name: None,
            frequency: None,
            mode: None,
            grid: None,
            distance: None,
            t_pwr: None,
            r_pwr: None,
            t_rst: None,
            r_rst: None,
            date_time_utc: None,
            notes: None
        }
    }
}

impl Contact {
    pub fn new(callsign: String, frequency_hz: u64, date_time_utc: DateTime<chrono::Utc>) -> Self {
        Self {
            callsign: Some(callsign),
            frequency: Some(frequency_hz),
            date_time_utc: Some(date_time_utc.to_rfc3339()),
            ..Default::default()
        }
    }
}
