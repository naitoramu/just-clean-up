use crate::database::mongodb::entity::user_duty_entity::UserDutyEntity;
use crate::database::mongodb::repository::mongo_repository::MongoRepository;
use crate::database::mongodb::repository::ObjectIdMapper;
use crate::database::user_duty_repository::UserDutyRepository;
use crate::domain::model::user_duty::UserDuty;
use crate::error::json_problem::JsonProblem;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;

pub struct UserDutyMongoRepository {
    repository: MongoRepository<UserDuty, UserDutyEntity>
}

impl UserDutyMongoRepository {

    pub fn new(database: &Database) -> Self {
        UserDutyMongoRepository {
            repository: MongoRepository::<UserDuty, UserDutyEntity>::new(database)
        }
    }
}

#[async_trait]
impl UserDutyRepository for UserDutyMongoRepository {
    async fn get_all_duties(&self) -> Result<Vec<UserDuty>, JsonProblem> {
        self.repository.get_all().await
    }

    async fn get_all_user_duties(&self, user_id: String) -> Result<Vec<UserDuty>, JsonProblem> {
        let object_id = ObjectIdMapper::map_to_object_id(user_id.as_str())?;
        self.repository.find_all_matching(doc! {
            "user_id": object_id
        }).await
    }

    async fn get_user_duties_by_duty_template(&self, user_id: String, template_id: String) -> Result<Vec<UserDuty>, JsonProblem> {
        let user_object_id = ObjectIdMapper::map_to_object_id(user_id.as_str())?;
        let template_object_id = ObjectIdMapper::map_to_object_id(template_id.as_str())?;

        self.repository.find_all_matching(doc! {
            "user_id": user_object_id,
            "template_id": template_object_id,
        }).await
    }

    async fn get_user_duty_by_id(&self, id: String) -> Result<Option<UserDuty>, JsonProblem> {
        self.repository.get_by_id(id).await
    }

    async fn create_user_duty(&self, user: &UserDuty) -> Result<UserDuty, JsonProblem> {
        self.repository.create(user).await
    }

    async fn update_user_duty(&self, id: String, user: &UserDuty) -> Result<UserDuty, JsonProblem> {
        self.repository.update(id, user).await
    }

    async fn delete_user_duty(&self, id: String) -> Result<(), JsonProblem> {
        self.repository.delete(id).await
    }
}