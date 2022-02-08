use std::fmt::{Debug};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub uid: String,
    pub message: String,
}