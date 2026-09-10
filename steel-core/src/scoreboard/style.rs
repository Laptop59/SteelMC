use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Style {
    todo: ()
}

impl Style {
    pub fn new() -> Self {
        todo!()
    }
}