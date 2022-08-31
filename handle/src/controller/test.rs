use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::form::test::{JsonResponese,JsonRequest,FromRequest};
use crate::form::validate::{ValidatedJson,ValidatedFrom};
pub struct Test {}

impl Test {
    pub async fn json_data(ValidatedJson(data): ValidatedJson<JsonRequest>) -> impl IntoResponse {
        let result = JsonResponese {
            id: 9999_i32,
            name: data.name,
        };

        (StatusCode::CREATED, Json(result))
    }

    pub async fn form_data(ValidatedFrom(data): ValidatedFrom<FromRequest>) -> impl IntoResponse {
        let result = JsonResponese {
            id: 9000_i32,
            name: data.name,
        };

        (StatusCode::CREATED, Json(result))
    }
}
