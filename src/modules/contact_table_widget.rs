use egui_extras::Column;
use crate::QSOVault;
use crate::database_interface;

pub fn render(ui: &mut egui::Ui, app: &mut QSOVault) {

    // Contacts table
    {
        egui::ScrollArea::new([true; 2]).show(ui, |new_ui| {
            let table = egui_extras::TableBuilder::new(new_ui)
                .striped(true)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::exact(10.0))
                .column(Column::at_least(Column::auto(), 60.0))
                .columns(Column::auto(), 10)
                .column(Column::remainder());
                //.auto_shrink([true, true]);

            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("ID");
                    });
                    header.col(|ui| {
                        ui.strong("Callsign");
                    });
                    header.col(|ui| {
                        ui.strong("Name");
                    });
                    header.col(|ui| {
                        ui.strong("Freq");
                    });
                    header.col(|ui| {
                        ui.strong("Mode");
                    });
                    header.col(|ui| {
                        ui.strong("Grid");
                    });
                    header.col(|ui| {
                        ui.strong("Distance");
                    });
                    header.col(|ui| {
                        ui.strong("TX Pwr");
                    });
                    header.col(|ui| {
                        ui.strong("RX Pwr");
                    });
                    header.col(|ui| {
                        ui.strong("TX RST");
                    });
                    header.col(|ui| {
                        ui.strong("RX RST");
                    });
                    header.col(|ui| {
                        ui.strong("Date/Time");
                    });
                    header.col(|ui| {
                        ui.strong("Notes");
                    });
                })
                .body(|mut body| {

                    if let Ok(db) = &app.db {
                        let contacts = database_interface::get_contacts(db);

                        for (contact_index, contact) in contacts.iter().enumerate() {

                            body.row(18.0, |mut row| {

                                row.col(|ui| {
                                    ui.label(contact_index.to_string());
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.callsign {
                                        ui.label(val);
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.name {
                                        ui.label(val);
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.frequency_hz {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.mode {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.grid {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.distance {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.t_pwr {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.r_pwr {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.t_rst {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.r_rst {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.date_time_utc {
                                        ui.label(val.to_string());
                                    }
                                });
                                row.col(|ui| {
                                    if let Some(val) = &contact.notes {
                                        ui.label(val.to_string());
                                    }
                                });

                            });
                        }
                    }
                });
        });

    }



}
