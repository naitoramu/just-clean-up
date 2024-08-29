use crate::domain::model::time_duration::TimeDuration;

#[derive(Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub struct Duty {

    pub id: String,

    pub title: String,

    pub todo_list: Vec<String>,

    pub img_src: Option<String>,

    pub repetition: TimeDuration,

    pub offset: TimeDuration,

    pub penalty: String,
}

impl Duty {

    pub fn new(
        id: String,
        title: String,
        todo_list: Vec<String>,
        img_src: Option<String>,
        repetition: TimeDuration,
        offset: TimeDuration,
        penalty: String,
    ) -> Self {
        Duty { id, title, todo_list, img_src, repetition, offset, penalty }
    }
}