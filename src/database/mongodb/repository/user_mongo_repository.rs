use crate::database::mongodb::entity::user_entity::UserEntity;
use crate::database::mongodb::repository::mongo_repository::MongoRepository;
use crate::database::user_repository::UserRepository;
use crate::domain::model::user::User;
use crate::error::json_problem::JsonProblem;
use async_trait::async_trait;
use mongodb::bson::doc;
use mongodb::Database;

pub struct UserMongoRepository {
    repository: MongoRepository<User, UserEntity>
}

impl UserMongoRepository {

    pub fn new(database: &Database) -> Self {
        UserMongoRepository {
            repository: MongoRepository::<User, UserEntity>::new(database)
        }
    }
}

#[async_trait]
impl UserRepository for UserMongoRepository {
    async fn get_all_users(&self) -> Result<Vec<User>, JsonProblem> {
        self.repository.get_all().await
    }

    async fn get_user_by_id(&self, id: String) -> Result<Option<User>, JsonProblem> {
        self.repository.get_by_id(id).await
    }

    async fn get_user_by_email_and_passwd(&self, email: String, passwd: String) -> Result<Option<User>, JsonProblem> {
        self.repository.find_first_matching(doc! {
                "email": email,
                "password": passwd
        }).await

    }

    async fn create_user(&self, user: &User) -> Result<User, JsonProblem> {
        self.repository.create(user).await
    }

    async fn update_user(&self, id: String, user: &User) -> Result<User, JsonProblem> {
        self.repository.update(id, user).await
    }

    async fn delete_user(&self, id: String) -> Result<(), JsonProblem> {
        self.repository.delete(id).await
    }
}