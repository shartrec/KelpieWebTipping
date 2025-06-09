// frontend/src/models/tipper.rs
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize, Serialize, Debug)]
pub struct Tipper {
    pub id: i32,
    pub name: String,
    pub email: String,
}
