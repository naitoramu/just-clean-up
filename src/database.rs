use lazy_static::lazy_static;
use mongodb::Client;
use mongodb::options::{ClientOptions, Credential, ServerAddress, ServerApi, ServerApiVersion};
use tokio::sync::OnceCell;

use crate::config::AppConfig;

lazy_static! {
    pub static ref DATABASE: OnceCell<mongodb::Database> = OnceCell::new();
}

pub struct Database {
    client_options: ClientOptions
}

impl Database {

    pub fn new() -> Self {
        let credentials = Credential::builder()
            .username(Some(AppConfig::get().db_username.clone()))
            .password(Some(AppConfig::get().db_password.clone()))
            .build();

        let hosts = vec!(ServerAddress::Tcp {
            host: AppConfig::get().db_url.clone(),
            port: None
        });

        let client_options = ClientOptions::builder()
            .app_name(Some("JustCleanUp-API".to_string()))
            .credential(Some(credentials))
            .hosts(hosts)
            .server_api(ServerApi::builder().version(ServerApiVersion::V1).build())
            .build();

        Self { client_options }
    }

    pub async fn establish_connection(&self) -> Result<(), mongodb::error::Error> {
        let client = Client::with_options(self.client_options.clone())?
            .database("just-clean-up");

        DATABASE.set(client).expect("Cannot create database instance");

        Ok(())
    }

    pub fn get_connection() -> &'static mongodb::Database {
        DATABASE.get().expect("Cannot get database instance")
    }
}
