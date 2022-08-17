use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::form::test::{ValidatedJson,JsonResponese,JsonRequest};
pub struct Test {}

impl Test {
    // pub async fn json_data(Json(data): Json<JsonRequest>) -> impl IntoResponse {
    pub async fn json_data(ValidatedJson(data): ValidatedJson<JsonRequest>) -> impl IntoResponse {
        let result = JsonResponese {
            id: 9999_i32,
            name: data.name,
        };

        (StatusCode::CREATED, Json(result))
    }
}
