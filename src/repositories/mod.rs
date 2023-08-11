use async_trait::async_trait;
use sqlx::{FromRow, Row};
use crate::error::CustomError;

pub mod user_repository;

#[async_trait]
pub trait Repository<T> {
    const SELECT_ALL_QUERY: &'static str;
    async fn get_all() -> Result<Vec<T>, CustomError>;

    const SELECT_BY_ID_QUERY: &'static str;
    async fn get_by_id(id: u64) -> Result<T, CustomError>;

    const INSERT_QUERY: &'static str;
    async fn create(entity: &T) -> Result<T, CustomError>;

    const UPDATE_QUERY: &'static str;
    async fn update(id: u64, entity: &T) -> Result<T, CustomError>;

    const DELETE_QUERY: &'static str;
    async fn delete(id: u64) -> Result<(), CustomError>;
}