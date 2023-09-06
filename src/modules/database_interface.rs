// use std::default::Default;
// use chrono::{DateTime, Utc};
// use rusqlite::{Connection, Result};
// use crate::modules::datatypes::*;
//
// pub fn get_db() -> Result<Connection> {
//     let database_connection_result = Connection::open("contacts.db3");
//
//     match database_connection_result {
//         Ok(_) => println!("[DATABASE] INFO: Successfully opened database"),
//         Err(_) => println!("[DATABASE] ERROR: Failed to open database")
//     };
//
//     if let Ok(db) = &database_connection_result {
//         initialize_database(db);
//     }
//
//     return database_connection_result;
//
//     // if let db = Ok(database_connection) {
//     //     return db;
//     // }
// }
//
// pub fn initialize_database(db: &Connection) {
//     let res = db.execute(
//         "CREATE TABLE contacts (
//                 id INTEGER PRIMARY KEY AUTOINCREMENT,
//                 callsign TEXT NOT NULL,
//                 name TEXT,
//                 frequency UNSIGNED BIGINT NOT NULL,
//                 mode TEXT,
//                 grid TEXT,
//                 distance UNSIGNED INTEGER,
//                 t_pwr TEXT,
//                 r_pwr TEXT,
//                 t_rst TEXT,
//                 r_rst TEXT,
//                 date_time_utc DATETIME NOT NULL,
//                 notes TEXT)", ()
//     );
//
//     match res {
//         Ok(_) => println!("[DATABASE] INFO: Successfully crated contacts table"),
//         Err(err) => println!("[DATABASE] WARN: Failed to create contacts table (It probably already exists)")
//     }
// }
//
// pub fn insert_contact(db: &Connection, input_contact: Contact) {
//
//     let sql_command = "
//         INSERT INTO contacts (callsign, name, frequency, mode, grid, distance, t_pwr, r_pwr, t_rst, r_rst, date_time_utc, notes)
//         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)
//         ";
//
//     let values_format = (
//         input_contact.callsign.clone(),
//         input_contact.name,
//         input_contact.frequency,
//         input_contact.mode,
//         input_contact.grid,
//         input_contact.distance,
//         input_contact.t_pwr,
//         input_contact.r_pwr,
//         input_contact.t_rst,
//         input_contact.r_rst,
//         input_contact.date_time_utc,
//         input_contact.notes
//     );
//
//     match db.execute(sql_command, values_format) {
//         Ok(_) => println!("[DATABASE] INFO: Successfully added QSO with {:?} to the database", input_contact.callsign),
//         Err(err) => println!("[DATABASE] ERROR: Failed to add QSO with {:?} to the database: \n {err}", input_contact.callsign)
//     }
// }
//
// pub fn get_contacts(db: &Connection) -> Vec<Contact> {
//     let mut res = db.prepare("SELECT * from contacts").unwrap();
//
//     let aya = res.query_map([], |row| {
//
//
//         // let date_time_utc_not_parsed: String = row.get(11)?;
//         // let date_time_utc = match DateTime::parse_from_rfc3339(&date_time_utc_not_parsed) {
//         //     Ok(rfc3339_time_utc) => rfc3339_time_utc.with_timezone(&chrono::Utc),
//         //     Err(_) => DateTime::default()
//         // };
//
//         return Ok(
//             Contact {
//                 id: row.get(0)?,
//                 callsign: row.get(1)?,
//                 name: row.get(2)?,
//                 frequency: row.get(3)?,
//                 mode: row.get(4)?,
//                 grid: row.get(5)?,
//                 distance: row.get(6)?,
//                 t_pwr: row.get(7)?,
//                 r_pwr: row.get(8)?,
//                 t_rst: row.get(9)?,
//                 r_rst: row.get(10)?,
//                 date_time_utc: row.get(11)?,
//                 notes: row.get(12)?,
//             }
//         );
//     });
//
//     let mut contacts_vec: Vec<Contact> = Vec::new();
//
//     if let Ok(resp) = aya {
//         for row in resp {
//             if let Ok(cont) = row {
//                 // println!("{:?}", cont.date_time_utc);
//                 contacts_vec.push(cont);
//             }
//         }
//     }
//
//     return contacts_vec;
// }
