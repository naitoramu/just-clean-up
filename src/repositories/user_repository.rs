use std::collections::HashMap;
use async_trait::async_trait;
use sqlx::{query, query_as};
use crate::database::{Database};
use crate::entities::{Entity, User};
use crate::error::http_error::HttpError;
use crate::error::http_error_kind::HttpErrorKind;
use crate::repositories::CrudRepository;

pub struct UserRepository {}

#[async_trait]
impl CrudRepository<User> for UserRepository {
    const SELECT_ALL_QUERY: &'static str = "\
        SELECT id, username, email, password, wallet \
        FROM user";
    async fn get_all() -> Result<Vec<User>, HttpError> {
        match query_as::<_, User>(Self::SELECT_ALL_QUERY)
            .fetch_all(Database::get_connection())
            .await {
            Ok(users) => Ok(users),
            Err(err) => Err(HttpError::from_type(
                HttpErrorKind::InternalServerError(Box::try_from(err).unwrap())
            ))
        }
    }

    const SELECT_BY_ID_QUERY: &'static str = "\
        SELECT id, username, email, password, wallet \
        FROM user \
        WHERE id = (?)";
    async fn get_by_id(id: u64) -> Result<User, HttpError> {
        match query_as::<_, User>(Self::SELECT_BY_ID_QUERY)
            .bind(id)
            .fetch_one(Database::get_connection())
            .await {
            Ok(user) => Ok(user),
            Err(err) => Err(
                HttpError::from_type(HttpErrorKind::ResourceNotFound(err))
                    .with_properties(HashMap::from([
                        ("resource", User::get_struct_name()),
                        ("resource_id", id.to_string())
                    ]))
            )
        }
    }

    const INSERT_QUERY: &'static str = "\
        INSERT INTO user (username, email, password, wallet) \
        VALUES (?, ?, ?, ?)";
    async fn create(entity: &User) -> Result<User, HttpError> {
        let insert_result = query(Self::INSERT_QUERY)
            .bind(entity.username.as_str())
            .bind(entity.email.as_str())
            .bind(entity.password.as_str())
            .bind(entity.wallet)
            .execute(Database::get_connection())
            .await;

        match insert_result {
            Ok(result) => Self::get_by_id(result.last_insert_id()).await,
            Err(err) => Err(HttpError::from_type(
                HttpErrorKind::InternalServerError(Box::try_from(err).unwrap())
            ))
        }
    }

    const UPDATE_QUERY: &'static str = "\
        UPDATE user \
        SET username = ?, email = ?, password = ?, wallet = ? \
        WHERE id = ?";
    async fn update(id: u64, entity: &User) -> Result<User, HttpError> {
        match query(Self::UPDATE_QUERY)
            .bind(entity.username.as_str())
            .bind(entity.email.as_str())
            .bind(entity.password.as_str())
            .bind(entity.wallet)
            .bind(id)
            .execute(Database::get_connection()).await {
            Ok(_) => Self::get_by_id(id).await,
            Err(err) => Err(HttpError::from_type(
                HttpErrorKind::InternalServerError(Box::try_from(err).unwrap())
            ))

        }
    }

    const DELETE_QUERY: &'static str = "\
        DELETE FROM user \
        WHERE id = ?";
    async fn delete(id: u64) -> Result<(), HttpError> {
        Self::get_by_id(id).await?;

        match query(Self::DELETE_QUERY)
            .bind(id)
            .execute(Database::get_connection())
            .await {
            Ok(_) => Ok(()),
            Err(err) => Err(HttpError::from_type(
                HttpErrorKind::InternalServerError(Box::try_from(err).unwrap())
            ))

        }
    }
}