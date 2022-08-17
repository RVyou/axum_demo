use async_trait::async_trait;
use axum::{
    extract::{FromRequest, RequestParts},
    http::{StatusCode,header},
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;
use validator::Validate;

#[derive(Deserialize, Debug, Validate)]
pub struct JsonRequest {
    pub id: i32,
    #[validate(length(min = 1, message = "不能为空"))]
    pub name: String,
}
#[derive(Serialize, Debug)]
pub struct JsonResponese {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;

        Ok(ValidatedJson(value))
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::JsonRejection),
}
impl IntoResponse for ServerError {
    //处理返回json错误信息
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(_) => {
                let message = json!({
                    "code": 400,
                    
                    "err": format!("{}",self)
                });
                (StatusCode::BAD_REQUEST ,  [(header::CONTENT_TYPE, "application/json")], message.to_string())
            }
            ServerError::AxumFormRejection(_) => (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "application/json")],
                json!({
                    "code": 400,
                    "err":  self.to_string()
                })
                .to_string(),
            ),
        }
        .into_response()
    }
}
