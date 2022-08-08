use serde::Deserialize;

#[derive(Deserialize)]
pub struct Todo {
    id: u64,
    name: String,
    is_checked: bool,
}

pub struct TodoPayload {
    pub name: String,
    pub is_checked: bool,
}

impl Deserialize for Todo{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {

    }
};

impl Todo {
    pub fn new(id: u64, name: String, is_checked: bool) -> Self {
        Todo {
            id,
            name,
            is_checked
        }
    }
}