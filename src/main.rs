#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// #![allow(unused)]

mod modules;
mod widgets;

use eframe::{egui, Frame};
use egui::Context;
use log::{info, trace, warn, error, debug};
use env_logger::Env;
// use modules::database_interface;
// use modules::contact_table_widget;
// use modules::insert_contact_widget;
// use rusqlite::Result;
// use crate::modules::database_interface::{get_contacts, get_db};
// use crate::modules::datatypes::*;
use crate::modules::datatypes::InputContact;
use widgets::*;
use modules::db_interface;

fn main() -> Result<(), eframe::Error> {

    // Initialize logger
    env_logger::Builder::new().filter_module(module_path!(), log::LevelFilter::Debug).init();
    info!("Logger initialized");

    // Create the window options
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 720.0)),
        ..Default::default()
    };

    // Create the native (desktop) app
    eframe::run_native(
        "QSO Vault",
        options,
        Box::new(|_cc| Box::<QSOVault>::default())
    )
}

pub struct QSOVault {
    db: Result<db_interface::DatabaseInterface, ()>,
    input_contact: InputContact,
    // contacts: Vec<Contact>
}

impl Default for QSOVault {
    fn default() -> Self {
        Self {
            db: db_interface::DatabaseInterface::new(),
            input_contact: InputContact::default(),
            // contacts: Vec::new()
        }
    }
}

impl eframe::App for QSOVault {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {

        egui::TopBottomPanel::top("contacts_table").resizable(true).show(ctx, |ui| {
            // ui.set_height(frame.info().window_info.size.y * 0.50);
            ui.set_height(ui.available_height());
            contact_table_widget::render(ui, self);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // match &mut self.db {
            //     Ok(_) => {ui.label("Connected to DB");},
            //     Err(_) => {ui.label("Not connected to DB");}
            // }

            insert_contact_widget::render(ui, self);
        });
    }
}
