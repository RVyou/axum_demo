use axum::{
    extract::Path,
    http::header::HeaderMap,
    Json,
    response::IntoResponse,
};
use crate::form::{user::UserResponse,protocol::{json_success_response,json_error_response}};
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
    pub async fn user_modify(Path(user_id): Path<i32>, headers: HeaderMap) -> Json<UserResponse> {
        println!("header is {:?}", headers);
        println!("header is {:?}", user_id);

        Json(UserResponse {
            id: 1,
            float: 1_f64,
            name: String::from("_"),
        })
    }
}
