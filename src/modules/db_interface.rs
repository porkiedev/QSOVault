#![allow(dead_code)]
#![allow(unused_variables)]

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use surrealdb::engine::local::{Db, File};
use tokio::runtime::{Builder, Runtime};
use crate::modules::datatypes::Contact;


pub struct DatabaseInterface {
    async_runtime: Runtime,
    db_path: String,
    db: Surreal<Db>
}

impl DatabaseInterface {
    pub fn new(destination: String, credentials: Root) -> Result<Self, ()> {
        // Create async runtime with tokio and the current thread
        let async_runtime = match Self::setup_async_runtime() {
            Ok(rt) => rt,
            Err(_) => return Err(())
        };

        // Get path to local database
        let db_path = match Self::setup_database_path() {
            Ok(db_path) => db_path,
            Err(_) => return Err(())
        };

        // Connect to local db
        let db = match Self::setup_local_db(&async_runtime, db_path.clone()) {
            Ok(db) => db,
            Err(_) => return Err(())
        };

        let temp_self = Self {
            async_runtime,
            db_path,
            db
        };

        // Set namespace
        if let Err(_) = temp_self.use_namespace("main") {
            return Err(());
        };

        // Set database
        if let Err(_) = temp_self.use_database("primary") {
            return Err(());
        };

        return Ok(temp_self);
    }

    fn setup_async_runtime() -> Result<Runtime, ()> {
        return match Builder::new_current_thread().enable_all().build() {
            Ok(runtime) => {
                Ok(runtime)
            },
            Err(err) => {
                println!("[DATABASE:ERROR] Failed to initialize async runtime\n{err}");
                Err(())
            }
        }
    }

    fn setup_database_path() -> Result<String, ()> {
        // Get the current/working directory path
        let path_buffer = match std::env::current_dir() {
            Ok(path_buffer) => path_buffer,
            Err(err) => {
                println!("[DATABASE:INIT:ERROR] Failed to get the current working directory\n{err}");
                return Err(())
            }
        };

        // Append the database to the path and replace backslashes with forward slashes
        // Stupid Windows and its backslashes!
        let database_path = format!("{}/test.db", path_buffer.display()).replace("\\", "/");

        // Return the database path
        return Ok(database_path);
    }

    fn setup_local_db(rt: &Runtime, path: String) -> Result<Surreal<Db>, ()> {
        // Using the tokio async runtime
        return rt.block_on(async {
            // Connect to a local database
            return match Surreal::new::<File>(&path).await {
                Ok(db) => {
                    println!("[DATABASE:INFO] Opened database at '{path}'");
                    Ok(db)
                },
                Err(err) => {
                    println!("[DATABASE:ERROR] Failed to open database at '{path}'\n{err}");
                    Err(())
                }
            };
        });
    }

    fn setup_remote_db(rt: &Runtime, address: String) -> Result<Surreal<Client>, ()> {
        return rt.block_on(async {
            return match Surreal::new::<Ws>(address).await {
                Ok(db) => {
                    Ok(db)
                },
                Err(err) => {
                    println!("[DATABASE:ERROR] Failed to connect to database\n{err}");
                    Err(())
                }
            }
        });
    }

    fn use_namespace(&self, input_namespace_name: &str) -> Result<(), ()> {
        return self.async_runtime.block_on(async {
            return match self.db.use_ns(input_namespace_name).await {
                Ok(_) => {
                    println!("[DATABASE:INFO] Set namespace to '{input_namespace_name}'");
                    Ok(())
                }
                Err(err) => {
                    println!("[DATABASE:ERROR] Failed to set namespace to '{input_namespace_name}'\n{err}");
                    Err(())
                }
            }
        });
    }

    fn use_database(&self, input_database_name: &str) -> Result<(), ()> {
        return self.async_runtime.block_on(async {
            return match self.db.use_db(input_database_name).await {
                Ok(_) => {
                    println!("[DATABASE:INFO] Set database to '{input_database_name}'");
                    Ok(())
                }
                Err(err) => {
                    println!("[DATABASE:ERROR] Failed to set database to '{input_database_name}'\n{err}");
                    Err(())
                }
            }
        });
    }

    pub fn insert_contact(&self, input_contact: Contact) -> Result<(), ()> {
        return self.async_runtime.block_on(async {
            let input_contact_callsign = &input_contact.callsign.clone().unwrap_or("Undefined".to_string());
            let response: Result<Vec<Contact>, Error> = self.db.create("contacts").content(input_contact).await;

            return match response {
                Ok(_response) => {
                    println!("[DATABASE:INFO] Contact with '{input_contact_callsign}' been added to the database");
                    Ok(())
                },
                Err(err) => {
                    println!("[DATABASE:ERROR] Failed to add contact with '{input_contact_callsign}' into database\n{err}");
                    Err(())
                }
            }
        });
    }

    pub fn get_all_contacts(&mut self) -> Result<Vec<Contact>, ()> {
        return self.async_runtime.block_on(async {
            let response: Result<Vec<Contact>, Error> = self.db.select("contacts").await;

            return match response {
                Ok(resp) => {
                    println!("[DATABASE:INFO] Got all contacts");
                    Ok(resp)
                }
                Err(err) => {
                    println!("[DATABASE:ERROR] Failed to get all contacts\n{err}");
                    Err(())
                }
            };
        });
    }
}