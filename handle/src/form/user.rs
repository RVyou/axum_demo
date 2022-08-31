use serde::{Serialize,Deserialize};

#[derive(Debug, Clone, PartialEq, Serialize,Deserialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub float: f64,
}
