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