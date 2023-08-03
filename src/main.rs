#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod modules;

use chrono::Utc;
use eframe::{egui, Frame};
use egui::Context;
use egui_extras::Column;
use modules::database_interface;
use modules::contact_table_widget;
use rusqlite::{Connection, Result};
use crate::modules::database_interface::{Contact, get_contacts, get_db};

fn main() -> Result<(), eframe::Error> {

    // Create the window options
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    //let sample_qso = Contact::new("zaz".to_string(), 146520000, chrono::Utc::now());
    // let sample_qso = Contact {
    //     callsign: Some("KF0CZM".to_string()),
    //     name: Some("Elijah Fry".to_string()),
    //     frequency_hz: Some(146520000),
    //     mode: Some("NBFM".to_string()),
    //     grid: Some("EM17GQ".to_string()),
    //     distance: Some(12345),
    //     t_pwr: Some(75),
    //     r_pwr: Some(50),
    //     t_rst: Some(599),
    //     r_rst: Some(599),
    //     date_time_utc: Some(Utc::now().to_rfc3339()),
    //     notes: Some("This is a sample note about myself, I guess..".to_string()),
    // };
    // if let Ok(db) = database_interface::get_db() {
    //     database_interface::insert_contact(&db, sample_qso);
    //     // get_contacts(&db);
    // }

    // Create the native (desktop) app
    eframe::run_native(
        "QSO Vault",
        options,
        Box::new(|_cc| Box::<QSOVault>::default())
    )

}

pub struct QSOVault {
    db: Result<rusqlite::Connection>,
    contacts: Vec<Contact>
}

impl Default for QSOVault {
    fn default() -> Self {
        Self {
            db: get_db(),
            contacts: Vec::new()
        }
    }
}

impl eframe::App for QSOVault {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        egui::TopBottomPanel::top("contacts_table").resizable(true).show(ctx, |ui| {
            // ui.set_height(frame.info().window_info.size.y * 0.50);
            ui.set_height(ui.available_height());

            // egui::ScrollArea::vertical().show(ui, |ui| {
            //     contact_table_widget::render(ui, self);
            // });
            contact_table_widget::render(ui, self);
        });


        egui::CentralPanel::default().show(ctx, |ui| {
            let db_connect_button_text = egui::RichText::new("Connect to DB");

            match &mut self.db {
                Ok(_) => {ui.label("Connected to DB");},
                Err(_) => {ui.label("Not connected to DB");}
            }

            frame.set_window_title("QSO Vault - Not connected to DB!");
        });
    }
}
