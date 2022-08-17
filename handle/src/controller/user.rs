use axum::{extract::Path, http::{header::HeaderMap},};

pub struct UserController {}

impl UserController {
    pub async fn users_teams_show(Path(user_id): Path<String>, headers: HeaderMap) -> String {
        println!("header is {:?}", headers);
        user_id
    }
}
