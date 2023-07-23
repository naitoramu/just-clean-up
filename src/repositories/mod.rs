use async_trait::async_trait;

pub mod user;

#[async_trait]
pub trait Repository<T> {
    async fn get_all(&self) -> Vec<T>;
    async fn get_by_id(id: u64) -> T;
    async fn create(entity: T);
    async fn update(entity: T);
    async fn delete(entity: T);
}