#![allow(dead_code)]
#![allow(unused_variables)]

use std::time::Duration;
use log::{debug, error, info, trace};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use surrealdb::engine::local::{Db, File};
use tokio::runtime::{Builder, Runtime};
use crate::modules::datatypes::Contact;


struct CachedData<T> {
    data: T,
    last_updated: std::time::Instant
}

impl<T> CachedData<T> {
    fn new(data: T) -> Self {
        Self {
            data,
            last_updated: std::time::Instant::now()
        }
    }
}

pub struct DatabaseInterface {
    async_runtime: Runtime,
    db_path: String,
    db: Surreal<Db>,
    caching: bool,
    get_all_contacts_cache: CachedData<Vec<Contact>>
}

impl DatabaseInterface {
    pub fn new() -> Result<Self, ()> {
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
            db,
            caching: true,
            get_all_contacts_cache: CachedData::new(Vec::new())
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
                debug!("Successfully initialized async runtime");
                Ok(runtime)
            },
            Err(err) => {
                error!("Failed to initialize async runtime\n{err}");
                Err(())
            }
        }
    }

    fn setup_database_path() -> Result<String, ()> {
        // Get the current/working directory path
        let path_buffer = match std::env::current_dir() {
            Ok(path_buffer) => path_buffer,
            Err(err) => {
                error!("Failed to get the current working directory\n{err}");
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
                    info!("Using local database at '{path}'");
                    Ok(db)
                },
                Err(err) => {
                    error!("Failed to open database at '{path}'\n{err}");
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
                    error!("Failed to connect to remote database\n{err}");
                    Err(())
                }
            }
        });
    }

    fn use_namespace(&self, input_namespace_name: &str) -> Result<(), ()> {
        return self.async_runtime.block_on(async {
            return match self.db.use_ns(input_namespace_name).await {
                Ok(_) => {
                    debug!("Set namespace '{input_namespace_name}'");
                    Ok(())
                }
                Err(err) => {
                    error!("Failed to set namespace to '{input_namespace_name}'\n{err}");
                    Err(())
                }
            }
        });
    }

    fn use_database(&self, input_database_name: &str) -> Result<(), ()> {
        return self.async_runtime.block_on(async {
            return match self.db.use_db(input_database_name).await {
                Ok(_) => {
                    debug!("Set database '{input_database_name}'");
                    Ok(())
                }
                Err(err) => {
                    error!("Failed to set database to '{input_database_name}'\n{err}");
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
                    info!("Contact with '{input_contact_callsign}' has been added to the database");
                    Ok(())
                },
                Err(err) => {
                    error!("Failed to add contact with '{input_contact_callsign}' to the database\n{err}");
                    Err(())
                }
            }
        });
    }

    pub fn get_all_contacts(&mut self) -> Result<Vec<Contact>, ()> {
        if self.caching && self.get_all_contacts_cache.last_updated.elapsed() < Duration::from_secs(1) {
            trace!("Caching is enabled and is not expired, returning the cache instead of querying the database");
            let data = self.get_all_contacts_cache.data.clone();
            return Ok(data);
        };

        return self.async_runtime.block_on(async {
            let response: Result<Vec<Contact>, Error> = self.db.select("contacts").await;

            return match response {
                Ok(resp) => {
                    trace!("Successfully retrieved all contacts from the database");
                    if self.caching {
                        self.get_all_contacts_cache = CachedData::new(resp.clone());
                        trace!("Caching is enabled, the contacts cache has been updated");
                    };
                    Ok(resp)
                }
                Err(err) => {
                    error!("Failed to retrieve all contacts from the database\n{err}");
                    Err(())
                }
            };
        });
    }
}