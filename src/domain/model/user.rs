use crate::domain::model::domain_model::DomainModel;

#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct User {

    pub id: String,

    pub username: String,

    pub email: String,

    pub password: String,
}

impl User {

    pub fn new(id: String, username: String, email: String, password: String) -> Self {
        User { id, username, email, password }
    }
}

impl DomainModel for User {
    fn get_resource_name() -> &'static str {
        "User"
    }
}