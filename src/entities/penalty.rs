use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Penalty {
    pub content: String,
    pub redeemed: bool
}

impl Penalty {
    pub fn new(content: String) -> Self {
        Penalty { content, redeemed: false }
    }
}