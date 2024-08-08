#[derive(Clone)]
#[non_exhaustive]
pub struct Duty {

    pub id: String,

    pub title: String,

    pub todo_list: Vec<String>,

    pub img_src: Option<String>,

    pub repetition: String,

    pub offset: String,

    pub penalty: String,
}

impl Duty {

    pub fn new(
        id: String,
        title: String,
        todo_list: Vec<String>,
        img_src: Option<String>,
        repetition: String,
        offset: String,
        penalty: String,
    ) -> Self {
        Duty { id, title, todo_list, img_src, repetition, offset, penalty }
    }
}