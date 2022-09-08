use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use time::serde::rfc3339;
use validator::Validate;
use validator::ValidationError;

use super::comment::Page;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserResponse {
    pub id: i32,
    pub name: String,
    pub float: f64,
    #[serde(with = "rfc3339")]
    pub created_date: OffsetDateTime,
}


#[derive(Deserialize, PartialEq, Debug, Validate)]
pub struct UserModifyRequest {
    #[validate(length(min = 1, message = "不能为空"))]
    pub name: String,
    #[validate(range(min = 1.0, max = 10.9, message = "范围只允许1.0-10.9"))]
    #[serde(rename = "rename_float")]
    pub float_test: f64,
    #[validate(length(min = 1), custom(function = "only_message", message = "message 只允许 only"))]
    pub message: String,
}

#[derive(Deserialize, PartialEq, Debug, Validate)]
pub struct UserListRequest {
    pub page: u64,
    pub limit: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserListResponse {
    pub list: Vec<UserListData>,
    pub meta: Page,
}


#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct UserListData {
    pub id: i32,
    pub name: String,
    pub float: f64,
    #[serde(with = "rfc3339")]
    pub created_date: OffsetDateTime,
}

fn only_message(message: &str) -> Result<(), ValidationError> {
    if message != "olny" {
        return Err(ValidationError::new("message 只允许 only"));
    }
    Ok(())
}