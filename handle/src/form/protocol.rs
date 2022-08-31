use axum::{
    http::{header::{self, HeaderMap}, StatusCode},
    Json,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct JsonProtocol<T: Serialize> {
    pub code: u32,
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct JsonProtocolError {
    pub error: Vec<String>,
}

#[inline(always)]
pub fn json_response<T>(code: StatusCode, header: HeaderMap, data: JsonProtocol<T>) -> Response where T: Serialize {
    (code, header, Json(data)).into_response()
}


pub fn json_success_response<T>(data: T) -> Response where T: Serialize {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    json_response(StatusCode::OK, headers, JsonProtocol {
        code: 200,
        message: "ok".to_string(),
        data: data,
    })
}

pub fn json_error_response(err: Vec<String>) -> Response {
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    json_response(StatusCode::INTERNAL_SERVER_ERROR, headers, JsonProtocol {
        code: 500,
        message: "err".to_string(),//动态修改
        data: JsonProtocolError {
            error: err
        },
    })
}