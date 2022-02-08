use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct User {
    pub id: String
}