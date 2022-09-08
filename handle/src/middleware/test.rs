use axum::{
    body::Body,
    http::{header::{self, HeaderMap}, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};

use crate::form::protocol::{json_response, JsonProtocol, JsonProtocolError};

pub struct Test {}

impl Test {
    pub async fn auth(req: Request<Body>, next: Next<Body>) -> Result<impl IntoResponse, impl IntoResponse> {
        let auth = if let Some(auto) = req.headers().get("authorization") {
            if let Ok(token) = auto.to_str() {
                token
            } else {
                ""
            }
        } else {
            ""
        };
        println!("{}",auth.len());
        if auth.len() < 6 {
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());
            return Err(json_response(StatusCode::UNAUTHORIZED, headers, JsonProtocol {
                code: 500,
                message: "err".to_string(),//动态修改
                data: JsonProtocolError {
                    errors: vec![String::from("没有权限")]
                },
            }));
        }

        let res = next.run(req).await;
        Ok(res)
    }
}