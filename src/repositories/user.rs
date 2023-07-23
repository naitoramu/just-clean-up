use async_trait::async_trait;
use sqlx::MySqlPool;
use crate::entities::User;
use crate::repositories::Repository;

pub struct UserRepository<'a> {
    db: &'a MySqlPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a MySqlPool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<'a> Repository<User> for UserRepository<'a> {
    async fn get_all(&self) -> Vec<User> {
    sqlx::query_as::<_, User>("SELECT id, username, email, password, wallet FROM user")
        .fetch_all(self.db)
        .await
        .unwrap().to_owned()
    }

    fn get_by_id(id: u64) -> User {
        todo!()
    }

    fn create(entity: User) {
        todo!()
    }

    fn update(entity: User) {
        todo!()
    }

    fn delete(entity: User) {
        todo!()
    }
}