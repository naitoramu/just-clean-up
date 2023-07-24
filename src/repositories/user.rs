use async_trait::async_trait;
use sqlx::{query, query_as};
use sqlx::mysql::MySqlQueryResult;
use crate::database::{Database};
use crate::entities::User;
use crate::repositories::Repository;

pub struct UserRepository {}

#[async_trait]
impl Repository<User> for UserRepository {
    const SELECT_ALL_QUERY: &'static str = "\
        SELECT id, username, email, password, wallet \
        FROM user";
    async fn get_all() -> Option<Vec<User>> {
        query_as::<_, User>(Self::SELECT_ALL_QUERY)
            .fetch_all(Database::get_connection())
            .await
            .ok()
    }

    const SELECT_BY_ID_QUERY: &'static str = "\
        SELECT id, username, email, password, wallet \
        FROM user \
        WHERE id = (?)";
    async fn get_by_id(id: u64) -> Option<User> {
        query_as::<_, User>(Self::SELECT_BY_ID_QUERY)
            .bind(id)
            .fetch_one(Database::get_connection())
            .await
            .ok()
    }

    const INSERT_QUERY: &'static str = "\
        INSERT INTO user (username, email, password, wallet) \
        VALUES (?, ?, ?, ?)";
    async fn create(entity: &User) -> Option<User> {
        let insert_result: MySqlQueryResult = query(Self::INSERT_QUERY)
            .bind(entity.username.as_str())
            .bind(entity.email.as_str())
            .bind(entity.password.as_str())
            .bind(entity.wallet)
            .execute(Database::get_connection()).await
            .expect(format!("Unable to create new User: {:?}", entity).as_str());

        Self::get_by_id(insert_result.last_insert_id()).await
    }

    const UPDATE_QUERY: &'static str = "\
        UPDATE user \
        SET username = ?, email = ?, password = ?, wallet = ? \
        WHERE id = ?";
    async fn update(id: u64, entity: &User) -> Option<User> {
        query(Self::UPDATE_QUERY)
            .bind(entity.username.as_str())
            .bind(entity.email.as_str())
            .bind(entity.password.as_str())
            .bind(entity.wallet)
            .bind(id)
            .execute(Database::get_connection()).await
            .expect(format!("Unable to create new User: {:?}", entity).as_str());

        Self::get_by_id(id).await
    }

    const DELETE_QUERY: &'static str = "\
        DELETE FROM user \
        WHERE id = ?";
    async fn delete(id: u64) {
        sqlx::query(Self::DELETE_QUERY)
            .bind(id)
            .execute(Database::get_connection())
            .await
            .expect(format!("Unable to delete user with id: {}", id).as_str());
    }
}