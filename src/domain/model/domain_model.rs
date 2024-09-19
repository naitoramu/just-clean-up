pub trait DomainModel {
    fn id(&self) -> String;
    fn get_resource_name() -> &'static str;
}