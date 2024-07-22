pub use self::user::User;

mod user;
pub mod cleaning_plan;
pub mod duty;
pub mod penalty;

pub trait Entity {
    fn get_struct_name() -> String {
        let full_struct_name: &str = std::any::type_name::<Self>();
        full_struct_name
            .rsplitn(2, "::")
            .next()
            .unwrap_or(full_struct_name)
            .to_string()
    }
}