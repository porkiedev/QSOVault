use std::default::Default;
use std::ops::RangeFrom;
use std::str::FromStr;
use chrono::{DateTime, ParseResult, TimeZone, Utc};
use rusqlite::{Connection, Error, Result};

pub fn get_db() -> Result<Connection> {
    let database_connection_result = Connection::open("contacts.db3");

    match database_connection_result {
        Ok(_) => println!("[DATABASE] INFO: Successfully opened database"),
        Err(_) => println!("[DATABASE] ERROR: Failed to open database")
    };

    if let Ok(db) = &database_connection_result {
        initialize_database(db);
    }

    return database_connection_result;

    // if let db = Ok(database_connection) {
    //     return db;
    // }
}

pub fn initialize_database(db: &Connection) {
    let res = db.execute(
        "CREATE TABLE contacts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                callsign TEXT NOT NULL,
                name TEXT,
                frequency_hz UNSIGNED BIGINT NOT NULL,
                mode TEXT,
                grid TEXT,
                distance UNSIGNED INTEGER,
                t_pwr UNSIGNED INTEGER,
                r_pwr UNSIGNED INTEGER,
                t_rst UNSIGNED INTEGER,
                r_rst UNSIGNED INTEGER,
                date_time_utc DATETIME NOT NULL,
                notes TEXT)", ()
    );

    match res {
        Ok(_) => println!("[DATABASE] INFO: Successfully crated contacts table"),
        Err(_) => println!("[DATABASE] WARN: Failed to create contacts table (It probably already exists)")
    }
}

pub fn insert_contact(db: &Connection, input_contact: Contact) {

    let sql_command = "
        INSERT INTO contacts (callsign, name, frequency_hz, mode, grid, distance, t_pwr, r_pwr, t_rst, r_rst, date_time_utc, notes)
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
        ";

    let values_format = (
        input_contact.callsign.clone(),
        input_contact.name,
        input_contact.frequency_hz,
        input_contact.mode,
        input_contact.grid,
        input_contact.distance,
        input_contact.t_pwr,
        input_contact.r_pwr,
        input_contact.t_rst,
        input_contact.r_rst,
        input_contact.date_time_utc,
        input_contact.notes
    );

    match db.execute(sql_command, values_format) {
        Ok(_) => println!("[DATABASE] INFO: Successfully added QSO with {:?} to the database", input_contact.callsign),
        Err(_) => println!("[DATABASE] ERROR: Failed to add QSO with {:?} to the database", input_contact.callsign)
    }
}

pub fn get_contacts(db: &Connection) -> Vec<Contact> {
    let mut res = db.prepare("SELECT * from contacts").unwrap();

    let aya = res.query_map([], |row| {


        // let date_time_utc_not_parsed: String = row.get(11)?;
        // let date_time_utc = match DateTime::parse_from_rfc3339(&date_time_utc_not_parsed) {
        //     Ok(rfc3339_time_utc) => rfc3339_time_utc.with_timezone(&chrono::Utc),
        //     Err(_) => DateTime::default()
        // };

        return Ok(
            Contact {
                callsign: row.get(1)?,
                name: row.get(2)?,
                frequency_hz: row.get(3)?,
                mode: row.get(4)?,
                grid: row.get(5)?,
                distance: row.get(6)?,
                t_pwr: row.get(7)?,
                r_pwr: row.get(8)?,
                t_rst: row.get(9)?,
                r_rst: row.get(10)?,
                date_time_utc: row.get(11)?,
                notes: row.get(12)?,
            }
        );
    });

    let mut contacts_vec: Vec<Contact> = Vec::new();

    if let Ok(resp) = aya {
        for row in resp {
            if let Ok(cont) = row {
                // println!("{:?}", cont.date_time_utc);
                contacts_vec.push(cont);
            }
        }
    }

    return contacts_vec;
}

// Structs

pub struct Contact {
    pub callsign: Option<String>,
    pub name: Option<String>,
    pub frequency_hz: Option<u64>,
    pub mode: Option<String>,
    pub grid: Option<String>,
    pub distance: Option<u32>,
    pub t_pwr: Option<u32>,
    pub r_pwr: Option<u32>,
    pub t_rst: Option<u32>,
    pub r_rst: Option<u32>,
    pub date_time_utc: Option<String>,
    pub notes: Option<String>
}

impl Default for Contact {
    fn default() -> Self {
        Self {
            callsign: None,
            name: None,
            frequency_hz: None,
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
    pub fn new(callsign: String, frequency_hz: u64, date_time_utc: DateTime<Utc>) -> Self {
        Self {
            callsign: Some(callsign),
            frequency_hz: Some(frequency_hz),
            date_time_utc: Some(date_time_utc.to_rfc3339()),
            ..Default::default()
        }
    }
}
