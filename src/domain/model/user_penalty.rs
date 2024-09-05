#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct UserPenalty {
    pub id: String,
    pub content: String,
    pub fulfilled: bool
}

impl UserPenalty {

    pub fn new(
        id: String,
        content: String,
        fulfilled: bool
    ) -> Self {
        Self { id, content, fulfilled }
    }
}
