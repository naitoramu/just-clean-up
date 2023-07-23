use async_trait::async_trait;
use crate::database::{CONNECTION, Database};
use crate::entities::User;
use crate::repositories::Repository;

pub struct UserRepository {}

#[async_trait]
impl Repository<User> for UserRepository {
    async fn get_all() -> Vec<User> {
    sqlx::query_as::<_, User>("SELECT id, username, email, password, wallet FROM user")
        .fetch_all(Database::get_connection())
        .await
        .unwrap().to_owned()
    }

    // fn get_by_id(id: u64) -> User {
    //     todo!()
    // }
    //
    // fn create(entity: User) {
    //     todo!()
    // }
    //
    // fn update(entity: User) {
    //     todo!()
    // }
    //
    // fn delete(entity: User) {
    //     todo!()
    // }
}