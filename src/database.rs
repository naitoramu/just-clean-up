use std::env;
use lazy_static::lazy_static;
use sqlx::{MySql, MySqlPool};
use sqlx::migrate::MigrateDatabase;
use tokio::sync::OnceCell;

lazy_static! {
    pub static ref CONNECTION: OnceCell<MySqlPool> = OnceCell::new();
}

pub struct Database {
    database_url: String,
}

impl Database {
    pub fn new() -> Self {
        Self { database_url: get_db_url() }
    }

    pub async fn create_db_if_not_exists(&self) -> &Self {
        if !MySql::database_exists(self.database_url.as_str()).await.unwrap_or(false) {
            println!("Creating database {}", self.database_url);
            match MySql::create_database(self.database_url.as_str()).await {
                Ok(_) => println!("Database created successfully"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
        self
    }

    pub async fn establish_connection(&self) {
        let pool = MySqlPool::connect(self.database_url.as_str())
            .await
            .expect("Cannot establish db connection");
        CONNECTION.set(pool).unwrap();
    }

    pub fn get_connection() -> &'static MySqlPool {
        CONNECTION.get().unwrap()
    }
}

fn get_db_url() -> String {
    env::var("DATABASE_URL").expect("$DATABASE_URL is not set!")
}
