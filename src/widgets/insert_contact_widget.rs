use std::ops::Mul;
use std::str::FromStr;
use eframe::emath::Align;
use egui::{FontSelection, TextBuffer};
use crate::QSOVault;

fn single_line_text_box(input_str: &mut String) -> egui::TextEdit {
    egui::TextEdit::singleline(input_str)
        .desired_width(100.0)
}

pub fn render(ui: &mut egui::Ui, app: &mut QSOVault) {

    ui.horizontal(|ui| {
        // Callsign input
        ui.vertical(|ui| {
            ui.strong("Callsign");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.callsign)
                .desired_width(60.0)
                .horizontal_align(Align::Center));
        });
        // Name input
        ui.vertical(|ui| {
            ui.strong("Name");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.name)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // Frequency input
        ui.vertical(|ui| {
            ui.strong("Freq");
            ui.add(egui::DragValue::new(&mut app.input_contact.frequency_hz)
                // Format the frequency into something more readable
                .custom_formatter(|n, _| {
                    return match n.to_string().len() {
                        4..=6 => format!("{}KHz", (n / 1_000.0)), // KHz
                        7..=9 => format!("{}MHz", (n / 1_000_000.0)), // MHz
                        10..=12 => format!("{}GHz", (n / 1_000_000_000.0)), // GHz
                        13..=15 => format!("{}THz", (n / 1_000_000_000_000.0)), // THz (Do THz radios even exist?)
                        _ => format!("{}Hz", n)
                    };
            })
                // Parse user input into units of Hz
                .custom_parser(|input_str| {
                    // Remove any spaces from the input string
                    let sanitized_input_str = input_str.replace(" ", "");

                    // Find the unit suffix if it exists (GHz, MHz, KHz, etc)
                    let split_index = sanitized_input_str
                        .chars()
                        .position(|c| c.is_alphabetic())
                        .unwrap_or(sanitized_input_str.len());

                    // Split the string into it's frequency and unit suffix parts
                    let (freq, unit) = sanitized_input_str.split_at(split_index);

                    // Try to convert the frequency to a f64, otherwise just set it to 0.0
                    let freq: f64 = freq.parse().unwrap_or(0.0);

                    // Convert the input frequency to its corresponding value in Hz
                    let freq_hz: f64 = match unit.to_lowercase().as_str() {
                        "hz" | "h" => { freq },
                        "khz" | "k" => { freq.mul(1000.0) },
                        "mhz" | "m" => { freq.mul(1000000.0) },
                        "ghz" | "g" => { freq.mul(1000000000.0) },
                        _ => { freq }
                    };

                    Some(freq_hz)
                }));
        });
        // Mode input
        ui.vertical(|ui| {
            ui.strong("Mode");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.mode)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // Grid square input
        ui.vertical(|ui| {
            ui.strong("Grid");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.grid)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // Distance input
        ui.vertical(|ui| {
            ui.strong("Distance");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.distance)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // TX Power
        ui.vertical(|ui| {
            ui.strong("TX Pwr");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.t_pwr)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // RX Power
        ui.vertical(|ui| {
            ui.strong("RX Pwr");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.r_pwr)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // TX RST
        ui.vertical(|ui| {
            ui.strong("TX RST");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.t_rst)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // RX RST
        ui.vertical(|ui| {
            ui.strong("RX RST");
            ui.add(egui::TextEdit::singleline(&mut app.input_contact.r_rst)
                .desired_width(80.0)
                .horizontal_align(Align::Center));
        });
        // Date and Time
        ui.vertical(|ui| {
            ui.strong("Date/Time");
            ui.label(&app.input_contact.date_time_utc);
            if ui.button("Now").clicked() {
                // &mut app.input_contact.date_time_utc = utc
            }
            // ui.add(egui::TextEdit::singleline(&mut app.input_contact.date_time_utc)
            //     .desired_width(80.0)
            //     .horizontal_align(Align::Center));
        });
    });
}
