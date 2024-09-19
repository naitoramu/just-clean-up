#[cfg(test)]
mod make_schedule_test {
    use crate::domain::model::cleaning_plan::{CleaningPlan, CleaningPlanStatus};
    use crate::domain::model::duty::{Duties, Duty};
    use crate::domain::model::routines::{Routine, Routines};
    use crate::domain::model::time_duration::TimeDuration;
    use crate::domain::model::user_duty::UserDuty;
    use crate::domain::service::user_duty_service::UserDutyService;
    use crate::tests::fakes::user_duty_fake_repository::UserDutyFakeRepository;
    use crate::tests::mocks::cleaning_plan_repository_mock::CleaningPlanRepositoryMock;
    use chrono::{DateTime, TimeDelta, Utc};
    use mongodb::bson::oid::ObjectId;
    use std::collections::HashMap;
    use std::sync::Arc;
    use crate::database::cleaning_plan_repository::CleaningPlanRepository;
    use crate::database::user_duty_repository::UserDutyRepository;
    use crate::domain::model::duty_fulfilment::DutyFulfilment;
    use crate::domain::model::user_penalty::UserPenalty;
    use crate::domain::model::user_tasks::UserTasks;

    #[tokio::test]
    async fn assign_each_duty_to_each_other_user_when_users_and_duty_count_is_same() {
        let user_duty_service = UserDutyService::new(
            cleaning_plan_repository(mock_cleaning_plan(mock_ids(3), vec![mock_ids(3)])),
            user_duty_repository(None),
        );

        let result = user_duty_service.make_schedules().await;
        assert!(result.is_ok());

        let created_duties = result.unwrap();
        assert_eq!(created_duties.len(), 3);

        let mut users_with_assigned_duty = Vec::new();
        for created_duty in created_duties {
            assert!(!users_with_assigned_duty.contains(&created_duty.user_id));
            users_with_assigned_duty.push(created_duty.user_id)
        }
    }

    #[tokio::test]
    async fn assign_multiple_duties_to_single_user_when_more_duties_than_users() {
        let duty_count = 5;
        let user_duty_service = UserDutyService::new(
            cleaning_plan_repository(mock_cleaning_plan(mock_ids(3), vec![mock_ids(duty_count)])),
            user_duty_repository(None),
        );


        let result = user_duty_service.make_schedules().await;
        assert!(result.is_ok());

        let created_duties = result.unwrap();
        assert_eq!(created_duties.len(), duty_count);

        let user_to_duty_count = count_duties_assigned_to_users(created_duties);
        for (_, duty_count) in user_to_duty_count {
            assert!(duty_count <= 2)
        }
    }

    #[tokio::test]
    async fn left_some_user_without_duty_when_less_duties_than_users() {
        let duty_count = 3;
        let user_duty_service = UserDutyService::new(
            cleaning_plan_repository(mock_cleaning_plan(mock_ids(5), vec![mock_ids(duty_count)])),
            user_duty_repository(None),
        );

        let result = user_duty_service.make_schedules().await;
        assert!(result.is_ok());

        let created_duties = result.unwrap();
        assert_eq!(created_duties.len(), duty_count);

        let user_to_duty_count = count_duties_assigned_to_users(created_duties);
        assert_eq!(user_to_duty_count.len(), duty_count)
    }

    #[tokio::test]
    async fn assign_duties_from_different_routines_separately() {
        let first_routine_duties_count = 3;
        let second_routine_duties_count = 2;
        let user_duty_service = UserDutyService::new(
            cleaning_plan_repository(mock_cleaning_plan(mock_ids(3), vec![mock_ids(first_routine_duties_count), mock_ids(second_routine_duties_count)])),
            user_duty_repository(None),
        );

        let result = user_duty_service.make_schedules().await;
        assert!(result.is_ok());

        let created_duties = result.unwrap();
        assert_eq!(created_duties.len(), first_routine_duties_count + second_routine_duties_count);
    }

    #[tokio::test]
    async fn select_user_that_has_not_been_assigned_to_the_duty_the_longest() {
        let user_ids = mock_ids(3);
        let template_ids = mock_ids(5);
        let iters = mock_iterations(3, TimeDelta::days(7));

        let cleaning_plan = mock_cleaning_plan(
            user_ids.clone(),
            vec![template_ids[..3].to_vec(), template_ids[3..].to_vec()],
        );

        // Vec<Vec<(<user_index>, <template_index>)>>
        let iterations = vec![
            vec![(0, 0), (1, 1), (2, 2), (0, 3), (1, 4)], // Iteration 1
            vec![(0, 1), (1, 2), (2, 0), (2, 3), (0, 4)], // Iteration 2
            vec![(0, 2), (1, 0), (2, 1), (1, 3), (2, 4)], // Iteration 3
        ];

        let result = UserDutyService::new(
            cleaning_plan_repository(cleaning_plan),
            user_duty_repository(Some(mock_user_duties(iterations, &user_ids, &template_ids, &iters))),
        ).make_schedules().await;
        assert!(result.is_ok());

        let created_duties = result.unwrap();
        assert_eq!(created_duties.len(), 5);

        let template_id_to_user_id = created_duties.iter()
            .map(|d| (d.template_id.clone(), d.user_id.clone()))
            .collect::<HashMap<String, String>>();

        assert_eq!(template_id_to_user_id[&template_ids[0]], user_ids[0]);
        assert_eq!(template_id_to_user_id[&template_ids[1]], user_ids[1]);
        assert_eq!(template_id_to_user_id[&template_ids[2]], user_ids[2]);
        assert_eq!(template_id_to_user_id[&template_ids[3]], user_ids[0]);
        assert_eq!(template_id_to_user_id[&template_ids[4]], user_ids[1]);
    }

    fn mock_user_duties(
        indices: Vec<Vec<(usize, usize)>>,
        users: &[String],
        templates: &[String],
        timestamps: &[DateTime<Utc>]
    ) -> Vec<UserDuty> {
        let mut user_duties = Vec::new();

        for (iter_index, user_to_template) in indices.iter().enumerate() {
            for &(user_index, template_index) in user_to_template {
                let duty = mock_existing_user_duty(
                    &users[user_index],
                    &templates[template_index],
                    &timestamps[iter_index],
                );
                user_duties.push(duty);
            }
        }

        user_duties
    }


    fn count_duties_assigned_to_users(duties: Vec<UserDuty>) -> HashMap<String, i32> {
        let mut user_to_duty_count: HashMap<String, i32> = HashMap::new();

        for duty in duties {
            if let Some(count) = user_to_duty_count.get(&duty.user_id) {
                user_to_duty_count.insert(duty.user_id, count + 1);
            } else {
                user_to_duty_count.insert(duty.user_id, 1);
            }
        }

        user_to_duty_count
    }

    fn cleaning_plan_repository(cleaning_plan: CleaningPlan) -> Arc<(dyn CleaningPlanRepository + Send + Sync + 'static)> {
        let cleaning_plan_repository = CleaningPlanRepositoryMock::new();
        cleaning_plan_repository.get_by_status_fn.return_ok(vec![cleaning_plan.clone()]);
        cleaning_plan_repository.update_fn.return_ok(cleaning_plan);
        Arc::new(cleaning_plan_repository)
    }

    fn user_duty_repository(initial_content: Option<Vec<UserDuty>>) -> Arc<(dyn UserDutyRepository + Send + Sync + 'static)> {
        let mut user_duty_repository = UserDutyFakeRepository::new();
        if let Some(content) = initial_content {
            user_duty_repository.set_user_duties(content);
        }
        Arc::new(user_duty_repository)
    }

    fn mock_existing_user_duty(user_id: &String, template_id: &String, timestamp: &DateTime<Utc>) -> UserDuty {
        UserDuty::new(
            ObjectId::new().to_hex(),
            user_id.clone(),
            template_id.clone(),
            "".to_string(),
            UserTasks::new(vec![]),
            timestamp.clone(),
            timestamp.clone() + TimeDelta::days(1),
            DutyFulfilment::new(true, true),
            UserPenalty::new("".to_string(), "".to_string(), true),
        )
    }

    fn mock_cleaning_plan(user_ids: Vec<String>, duties_in_routine: Vec<Vec<String>>) -> CleaningPlan {
        CleaningPlan::new(
            ObjectId::new().to_hex(),
            "test plan".to_string(),
            "test address".to_string(),
            user_ids,
            mock_routines(duties_in_routine),
            Utc::now(),
            CleaningPlanStatus::PendingDutyAssignment,
        )
    }

    fn mock_routines(duties_in_routines: Vec<Vec<String>>) -> Routines {
        let mut routines = Vec::new();
        for template_ids in duties_in_routines {
            routines.push(Routine::new(
                TimeDuration::new(TimeDelta::weeks(2)),
                TimeDuration::new(TimeDelta::days(2)),
                mock_duty_templates(template_ids),
            ))
        }
        Routines::new(routines)
    }

    fn mock_duty_templates(template_ids: Vec<String>) -> Duties {
        let mut duties = Vec::new();
        for template_id in template_ids {
            duties.push(mock_duty_template(template_id))
        }
        Duties::new(duties)
    }

    fn mock_duty_template(template_id: String) -> Duty {
        Duty::new(
            template_id,
            "test duty".to_string(),
            Vec::new(),
            None,
            "".to_string(),
            Utc::now(),
        )
    }

    fn mock_ids(count: usize) -> Vec<String> {
        let mut ids = Vec::new();

        for i in 0..count {
            ids.push(i.to_string());
        }

        ids
    }

    fn mock_iterations(iterations_count: i32, time_delta: TimeDelta) -> Vec<DateTime<Utc>> {
        let mut timestamps = Vec::new();
        for i in (1..=iterations_count).rev() {
            timestamps.push(Utc::now() - (time_delta * i));
        }
        timestamps
    }
}