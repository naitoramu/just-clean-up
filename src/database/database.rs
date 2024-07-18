use std::sync::Arc;
use crate::database::mongo_database::MongoDatabase;
use crate::entities::User;
use crate::repositories::Repository;
use crate::repositories::user_repository::UserRepository;

pub struct Database {
    mongo_database: Option<MongoDatabase>,
}

impl Database {
    pub async fn mongo_db_connection() -> Self {
        Database {
            mongo_database: Some(
                MongoDatabase::new().await
                    .establish_connection().await
                    .expect("Cannot establish database connection")
                    .create_collections().await
                    .expect("Cannot create collection")
            )
        }
    }

    pub fn get_user_repository(&self) -> Arc<dyn Repository<User> + Send + Sync> {
        Arc::new(UserRepository::new(
            self.mongo_database.as_ref().expect("Database not initialized").get_connection(),
        ))
    }
}