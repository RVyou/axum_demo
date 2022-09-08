use axum::{
    extract::{Multipart, TypedHeader},
    headers::{Authorization, authorization::Bearer},
    http::StatusCode,
    Json,
    response::IntoResponse,
};

use crate::form::test::{FromRequest, JsonRequest, JsonResponese};
use crate::form::validate::{ValidatedFrom, ValidatedJson};

pub struct Test {}

impl Test {
    pub async fn json_data(ValidatedJson(data): ValidatedJson<JsonRequest>) -> impl IntoResponse {
        let result = JsonResponese {
            id: 9999_i32,
            name: data.name,
        };

        (StatusCode::CREATED, Json(result))
    }

    pub async fn auth(TypedHeader(token): TypedHeader<Authorization<Bearer>>) -> impl IntoResponse {
        (StatusCode::CREATED, format!("{:?}", token.token().to_string()))
    }

    pub async fn form_file(mut multipart: Multipart) -> impl IntoResponse {
        while let Some(field) = multipart.next_field().await.unwrap() {
            let name = field.name().unwrap().to_string();
            let data = field.bytes().await.unwrap();

            println!("Length of `{}` is {} bytes", name, data.len());
        }


        (StatusCode::CREATED, format!("ok"))
    }

    pub async fn form_data(ValidatedFrom(data): ValidatedFrom<FromRequest>) -> impl IntoResponse {
        let result = JsonResponese {
            id: 9000_i32,
            name: data.name,
        };

        (StatusCode::CREATED, Json(result))
    }
}
