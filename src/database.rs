use std::env;
use sqlx::{MySql, MySqlPool, Pool};
use sqlx::migrate::MigrateDatabase;

pub struct Database {
    database_url: String,
}

impl Database {

    pub fn new() -> Self{
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

    pub async fn get_connection(&self) -> Pool<MySql> {
        MySqlPool::connect(self.database_url.as_str()).await.unwrap()
    }
}

fn get_db_url() -> String {
    env::var("DATABASE_URL").expect("$DATABASE_URL is not set!")
}
