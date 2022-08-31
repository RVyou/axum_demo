
use serde::{Deserialize, Serialize};
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



#[derive(Deserialize, Debug, Validate)]
pub struct FromRequest {
    pub id: i32,
    #[validate(length(min = 1, message = "不能为空"))]
    pub name: String,
}

