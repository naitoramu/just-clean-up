use lazy_static::lazy_static;
use mongodb::Client;
use mongodb::options::{ClientOptions, Credential};
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
        let mut client_options = ClientOptions::parse(AppConfig::get().db_url.clone()).await
            .expect("Cannot parse database URL.");

        let credentials = Credential::builder()
            .username(Some(AppConfig::get().db_username.clone()))
            .password(Some(AppConfig::get().db_password.clone()))
            .build();

        client_options.credential = Some(credentials);

        Self { client_options }
    }

    pub async fn establish_connection(self) -> Result<Self, mongodb::error::Error> {
        let database = Client::with_options(self.client_options.clone())?
            .database("just-clean-up");

        DATABASE.set(database).expect("Cannot create database instance");

        Ok(self)
    }

    pub async fn create_collections(self) -> Result<Self, mongodb::error::Error> {
        let collection_names = vec![
            "users"
        ];

        for collection_name in collection_names {
            self.get_connection().create_collection(collection_name).await?;
        }

        Ok(self)
    }

    pub fn get_connection(&self) -> &'static mongodb::Database {
        DATABASE.get().expect("Cannot get database instance")
    }
}
