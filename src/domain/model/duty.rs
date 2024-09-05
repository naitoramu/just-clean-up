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
}

#[derive(Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub struct Duty {
    pub id: String,

    pub title: String,

    pub todo_list: Vec<String>,

    pub img_src: Option<String>,

    pub penalty: String,
}

impl Duty {
    pub fn new(
        id: String,
        title: String,
        todo_list: Vec<String>,
        img_src: Option<String>,
        penalty: String,
    ) -> Self {
        Duty { id, title, todo_list, img_src, penalty }
    }
}