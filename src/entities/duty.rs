use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[non_exhaustive]
pub struct Duty {

    pub title: String,

    pub todo_list: Vec<String>,

    pub img_src: Option<String>,

    pub repetition: String,

    pub offset: String,

    pub penalty: String,
}

impl Duty {

    pub fn new(
        title: String,
        todo_list: Vec<String>,
        img_src: Option<String>,
        repetition: String,
        offset: String,
        penalty: String,
    ) -> Self {
        Duty { title, todo_list, img_src, repetition, offset, penalty }
    }
}