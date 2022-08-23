use async_trait::async_trait;
use axum::{
    extract::{Form,FromRequest, RequestParts},
    http::{StatusCode,header},
    response::{IntoResponse, Response},
    BoxError, Json, 
};
use serde::de::DeserializeOwned;
use serde_json::json;
use thiserror::Error;
use validator::Validate;

//暂时只弄2个验证类 form 和json
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedFrom<T>(pub T);



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


#[async_trait]
impl<T, B> FromRequest<B> for ValidatedFrom<T>
where
    T: DeserializeOwned + Validate,
    B: http_body::Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ServerError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Form(value) = Form::<T>::from_request(req).await?;
        value.validate()?;

        Ok(ValidatedFrom(value))
    }
}



#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),//validate 验证错误


    #[error(transparent)]
    AxumJsonRejection(#[from] axum::extract::rejection::JsonRejection),//axum json 验证错误

    #[error(transparent)]
    AxumFormRejection(#[from] axum::extract::rejection::FormRejection),//axum from 验证错误
}

//实现 IntoResponse 输出错误
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
            ServerError::AxumJsonRejection(_) => (
                StatusCode::BAD_REQUEST,
                [(header::CONTENT_TYPE, "application/json")],
                json!({
                    "code": 400,
                    "err":  self.to_string()
                })
                .to_string(),
            ),
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


