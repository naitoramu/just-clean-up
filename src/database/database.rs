use std::sync::Arc;
use axum::BoxError;
use crate::database::crud_repository::CrudRepository;
use crate::database::mongodb::entity::cleaning_plan::CleaningPlanEntity;
use crate::database::mongodb::entity::entity::MongoEntity;
use crate::database::mongodb::entity::user::UserEntity;
use crate::database::mongodb::mongo_database::MongoDatabase;
use crate::database::mongodb::repository::mongo_repository::MongoRepository;
use crate::domain::model::cleaning_plan::CleaningPlan;
use crate::domain::model::domain_model::DomainModel;
use crate::domain::model::user::User;

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

    pub fn get_cleaning_plan_repository(&self) -> Arc<dyn CrudRepository<CleaningPlan>> {
        self.get_repository::<CleaningPlanEntity, CleaningPlan>()
    }

    pub fn get_user_repository(&self) -> Arc<dyn CrudRepository<User>> {
        self.get_repository::<UserEntity, User>()
    }

    fn get_repository<E, D>(&self) -> Arc<dyn CrudRepository<D>>
    where
        E: MongoEntity + Clone + TryFrom<D> + 'static,
        D: DomainModel + Sync + Clone + From<E>,
        <E as TryFrom<D>>::Error: Into<BoxError>,
    {
        Arc::new(MongoRepository::<E>::new(
            self.mongo_database.as_ref()
                .expect("Database not initialized")
                .get_connection(),
        ))
    }
}