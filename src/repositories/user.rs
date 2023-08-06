use async_trait::async_trait;
use axum::http::StatusCode;
use sqlx::{query, query_as};
use crate::database::{Database};
use crate::entities::User;
use crate::error::CustomError;
use crate::repositories::Repository;

pub struct UserRepository {}

#[async_trait]
impl Repository<User> for UserRepository {
    const SELECT_ALL_QUERY: &'static str = "\
        SELECT id, username, email, password, wallet \
        FROM user";
    async fn get_all() -> Result<Vec<User>, CustomError> {
        match query_as::<_, User>(Self::SELECT_ALL_QUERY)
            .fetch_all(Database::get_connection())
            .await {
            Ok(users) => Ok(users),
            Err(err) => Err(CustomError::new(
                StatusCode::NOT_FOUND,
                "Cannot fetch users".to_string(),
                err.to_string())
            )
        }
    }

    const SELECT_BY_ID_QUERY: &'static str = "\
        SELECT id, username, email, password, wallet \
        FROM user \
        WHERE id = (?)";
    async fn get_by_id(id: u64) -> Result<User, CustomError> {
        match query_as::<_, User>(Self::SELECT_BY_ID_QUERY)
            .bind(id)
            .fetch_one(Database::get_connection())
            .await {
            Ok(user) => Ok(user),
            Err(err) => Err(CustomError::new(
                StatusCode::NOT_FOUND,
                "User with given ID does not exists".to_string(),
                err.to_string()))
        }
    }

    const INSERT_QUERY: &'static str = "\
        INSERT INTO user (username, email, password, wallet) \
        VALUES (?, ?, ?, ?)";
    async fn create(entity: &User) -> Result<User, CustomError> {
        let insert_result = query(Self::INSERT_QUERY)
            .bind(entity.username.as_str())
            .bind(entity.email.as_str())
            .bind(entity.password.as_str())
            .bind(entity.wallet)
            .execute(Database::get_connection())
            .await;

        match insert_result {
            Ok(result) => Self::get_by_id(result.last_insert_id()).await,
            Err(err) => Err(CustomError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Cannot create user entity".to_string(),
                err.to_string()
            ))
        }
    }

    const UPDATE_QUERY: &'static str = "\
        UPDATE user \
        SET username = ?, email = ?, password = ?, wallet = ? \
        WHERE id = ?";
    async fn update(id: u64, entity: &User) -> Result<User, CustomError> {
        match query(Self::UPDATE_QUERY)
            .bind(entity.username.as_str())
            .bind(entity.email.as_str())
            .bind(entity.password.as_str())
            .bind(entity.wallet)
            .bind(id)
            .execute(Database::get_connection()).await {
            Ok(_) => Self::get_by_id(id).await,
            Err(err) => Err(CustomError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "Cannot update desired user entity".to_string(),
                err.to_string()
            ))
        }
    }

    const DELETE_QUERY: &'static str = "\
        DELETE FROM user \
        WHERE id = ?";
    async fn delete(id: u64) -> Result<(), CustomError> {
        match query(Self::DELETE_QUERY)
            .bind(id)
            .execute(Database::get_connection())
            .await {
            Ok(_) => Ok(()),
            Err(err) => Err(CustomError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Cannot delete user entity with given ID".to_string(),
                err.to_string()
            ))
        }
    }
}