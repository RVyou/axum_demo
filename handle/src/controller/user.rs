use axum::{
    extract::Path,
    http::header::HeaderMap,
    response::IntoResponse,
};

use crate::form::{protocol::{json_error_response, json_success_response}, user::{UserListRequest, UserModifyRequest}};
use crate::form::validate::ValidatedFrom;
use crate::services::user::UserService;

pub struct UserController {}

impl UserController {
    pub async fn user_message(Path(user_id): Path<i32>, headers: HeaderMap) -> impl IntoResponse {
        tracing::info!("header is {:?}",headers);
        if let Ok(user_data) = UserService::get_user_message(user_id) {
            return json_success_response(user_data);
        }
        json_error_response(vec!["not user".to_string()])
    }
    pub async fn user_modify(Path(user_id): Path<i32>, ValidatedFrom(user_data): ValidatedFrom<UserModifyRequest>) -> impl IntoResponse {
        tracing::info!("user data {:?}",user_data);
        match UserService::modify(user_id, user_data) {
            Ok(user_data) => {
                return json_success_response(user_data);
            }
            Err(e) => {
                json_error_response(vec![e.to_string()])
            }
        }
    }
    //get_users 这样是必填字段原生 axum::extract::Query,
    pub async fn get_users(ValidatedFrom(user_data): ValidatedFrom<UserListRequest>) -> impl IntoResponse {
        match UserService::get_users(user_data).await {
            Ok(user_data) => {
                return json_success_response(user_data);
            }
            Err(e) => {
                json_error_response(vec![e.to_string()])
            }
        }
    }
}
