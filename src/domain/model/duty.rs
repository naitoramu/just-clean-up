use chrono::{DateTime, Utc};

#[derive(Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub struct Duties {
    duties: Vec<Duty>,
}

impl Duties {

    pub fn new(duties: Vec<Duty>) -> Self {
        Duties { duties }
    }

    pub fn vec(self) -> Vec<Duty> {
        self.duties
    }

    pub fn sort_by_creation_time(mut self) -> Self {
        self.duties.sort_by(|a, b| a.creation_time.cmp(&b.creation_time));
        self
    }
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub struct Duty {
    pub id: String,

    pub title: String,

    pub todo_list: Vec<String>,

    pub img_src: Option<String>,

    pub penalty: String,

    pub creation_time: DateTime<Utc>
}

impl Duty {
    pub fn new(
        id: String,
        title: String,
        todo_list: Vec<String>,
        img_src: Option<String>,
        penalty: String,
        creation_time: DateTime<Utc>
    ) -> Self {
        Duty { id, title, todo_list, img_src, penalty, creation_time }
    }

    pub fn without_creation_time(
        id: String,
        title: String,
        todo_list: Vec<String>,
        img_src: Option<String>,
        penalty: String
    ) -> Self {
        Duty { id, title, todo_list, img_src, penalty, creation_time: Utc::now() }
    }
}