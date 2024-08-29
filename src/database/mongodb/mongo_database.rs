use lazy_static::lazy_static;
use log::debug;
use mongodb::Client;
use mongodb::options::{ClientOptions};
use tokio::sync::OnceCell;

use crate::config::AppConfig;

lazy_static! {
    static ref DATABASE: OnceCell<mongodb::Database> = OnceCell::new();
}

#[derive(Clone)]
pub struct MongoDatabase {
    client_options: ClientOptions
}

impl MongoDatabase {

    pub async fn new() -> Self {
        let client_options = ClientOptions::parse(AppConfig::get().db_url.clone()).await
            .expect(format!("Cannot parse database URL: '{}'", AppConfig::get().db_url).as_str());

        Self { client_options }
    }

    pub async fn establish_connection(self) -> Result<Self, mongodb::error::Error> {
        let database = Client::with_options(self.client_options.clone())?
            .database("just_clean_up");

        DATABASE.set(database).expect("Cannot create database instance");
        debug!("Mongo database connection established successfully");

        Ok(self)
    }

    pub async fn create_collections(self) -> Result<Self, mongodb::error::Error> {
        let collection_names = vec![
            "users",
            "user_duties",
            "cleaning_plans"
        ];

        for collection_name in collection_names {
            debug!("Creating database collection '{collection_name}'");
            self.get_connection().create_collection(collection_name).await?;
            debug!("Database collection '{collection_name}' created");
        }
        debug!("Database collections created successfully");

        Ok(self)
    }

    pub fn get_connection(&self) -> &'static mongodb::Database {
        DATABASE.get().expect("Cannot get database instance")
    }
}
