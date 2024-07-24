use std::sync::Arc;

use crate::database::mongo_database::MongoDatabase;
use crate::entities::Entity;
use crate::repositories::crud_repository::CrudRepository;
use crate::repositories::mongo_repository::MongoRepository;

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

    pub fn get_repository<T>(&self) -> Arc<dyn CrudRepository<T>> where T: Entity + 'static {
        Arc::new(MongoRepository::new(
            self.mongo_database.as_ref()
                .expect("Database not initialized")
                .get_connection(),
        ))
    }
}