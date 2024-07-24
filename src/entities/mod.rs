use serde::{Deserialize, Serialize};
pub use self::user::User;

mod user;
pub mod cleaning_plan;
pub mod duty;
pub mod penalty;

pub trait Entity: Send + Sync + Serialize + for<'a> Deserialize<'a> {
    fn get_resource_name() -> &'static str;
    fn get_collection_name() -> &'static str;
}