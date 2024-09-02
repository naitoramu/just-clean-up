use crate::domain::model::user::User;
use mongodb::bson::oid::ObjectId;

#[cfg(test)]
mod auth_service_test {
    use super::*;
    use crate::domain::service::auth_service::AuthService;
    use crate::tests::mocks::user_repository_mock::UserRepositoryMock;
    use std::sync::Arc;
    use crate::error::json_problems::JsonProblems;



    #[tokio::test]
    async fn returns_user_when_matching_email_and_password() {
        let test_user = mock_user();
        let user_repository = UserRepositoryMock::new();
        user_repository.get_by_email_and_passwd_fn.return_value(Ok(Some(test_user.clone())));

        let auth_service = AuthService::new(Arc::new(user_repository));
        let result = auth_service.get_user_by_email_and_password(mock_user().email, mock_user().password).await;

        assert_eq!(result, Ok(Some(test_user)))
    }

    #[tokio::test]
    async fn returns_none_when_match_not_exists() {
        let user_repository = UserRepositoryMock::new();
        user_repository.get_by_email_and_passwd_fn.return_value(Ok(None));

        let auth_service = AuthService::new(Arc::new(user_repository));
        let result = auth_service.get_user_by_email_and_password(mock_user().email, mock_user().password).await;

        assert_eq!(result, Ok(None))
    }

    #[tokio::test]
    async fn returns_problem_when_error_occurred() {
        let json_problem = JsonProblems::resource_not_found("User", ObjectId::new().to_hex());
        let user_repository = UserRepositoryMock::new();
        user_repository.get_by_email_and_passwd_fn.return_value(Err(json_problem.clone()));

        let auth_service = AuthService::new(Arc::new(user_repository));
        let result = auth_service.get_user_by_email_and_password(mock_user().email, mock_user().password).await;

        assert_eq!(result, Err(json_problem))
    }


    fn mock_user() -> User {
        User::new(
            ObjectId::new().to_hex(),
            "mock-username".to_string(),
            "mock-email@gmail.com".to_string(),
            "mock-passwd".to_string(),
        )
    }
}