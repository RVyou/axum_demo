use mysql::{from_row, from_value, Value};
use chrono::NaiveDate;
use crate::form::user::UserResponse;
use crate::model::user::User;

pub struct UserService {}

// `id` int NOT NULL AUTO_INCREMENT COMMENT '主表id',
// `name` varchar(255) NOT NULL COMMENT '名称',
// `float_test` double(13,3) NOT NULL DEFAULT '1.100' COMMENT 'float_test',
// `create_time` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP COMMENT '创建时间',
impl UserService {
    pub fn get_user_message(id: i32) -> Result<UserResponse, &'static str> {
        if let Some(data) = User::get_one_user("SELECT id,name,float_test,create_time FROM user WHERE id=?", (id,)) {
            let user_message: (i32, Value, f64, Value) = from_row(data);
            return Ok(UserResponse {
                id: user_message.0,
                name: from_value(user_message.1),
                float: user_message.2,
            });
        }
        Err("not data")
    }
}