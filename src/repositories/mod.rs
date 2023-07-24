use async_trait::async_trait;
use sqlx::{FromRow, Row};

pub mod user;

#[async_trait]
pub trait Repository<T> {
    const SELECT_ALL_QUERY: &'static str;
    async fn get_all() -> Option<Vec<T>>;

    const SELECT_BY_ID_QUERY: &'static str;
    async fn get_by_id(id: u64) -> Option<T>;

    const INSERT_QUERY: &'static str;
    async fn create(entity: &T) -> Option<T>;

    const UPDATE_QUERY: &'static str;
    async fn update(id: u64, entity: &T) -> Option<T>;

    const DELETE_QUERY: &'static str;
    async fn delete(id: u64);
}