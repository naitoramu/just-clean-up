#[derive(Clone)]
#[non_exhaustive]
pub struct UserTasks {
    pub tasks: Vec<UserTask>
}

impl UserTasks {

    pub fn new(
        tasks: Vec<UserTask>
    ) -> Self {
        Self { tasks }
    }

    pub fn from_template(tasks: &Vec<String>) -> UserTasks {
        let mut user_tasks: Vec<UserTask> = Vec::new();

        for task in tasks {
            user_tasks.push(UserTask::new(
                "".to_string(),
                task.clone(),
                Vec::new(),
                Vec::new()
            ));
        }

        Self::new(user_tasks)
    }
}

#[derive(Clone)]
#[non_exhaustive]
pub struct UserTask {
    pub id: String,
    pub task: String,
    pub accepting_user_ids: Vec<String>,
    pub rejecting_user_ids: Vec<String>,
}

impl UserTask {

    pub fn new(
        id: String,
        task: String,
        accepting_user_ids: Vec<String>,
        rejecting_user_ids: Vec<String>,
    ) -> Self {
        Self { id, task, accepting_user_ids, rejecting_user_ids, }
    }
}