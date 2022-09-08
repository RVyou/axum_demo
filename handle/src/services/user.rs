use mysql::{from_row, from_value, Value};
use time::{OffsetDateTime, PrimitiveDateTime};

use crate::form::comment::Page;
use crate::form::user::*;
use crate::model::user::User;

pub struct UserService {}

impl UserService {
    pub fn get_user_message(id: i32) -> Result<UserResponse, &'static str> {
        if let Some(data) = User::get_one_user("SELECT id,name,float_test,create_time FROM user WHERE id=?", (id, )) {
            let user_message: (i32, Value, f64, Value) = from_row(data);
            // let format = format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second]").unwrap();
            return Ok(UserResponse {
                id: user_message.0,
                name: from_value(user_message.1),
                float: user_message.2,
                // created_date:from_value::<PrimitiveDateTime>(user_message.3).format(&format).unwrap_or("".to_string()).to_string(),
                created_date: from_value::<PrimitiveDateTime>(user_message.3).assume_utc(),
            });
        }
        Err("not data")
    }

    pub fn modify(id: i32, data: UserModifyRequest) -> Result<(), &'static str> {
        let now = OffsetDateTime::now_utc();
        if 1 == User::modify_one_user("UPDATE user SET name=?,float_test=?,create_time=?  WHERE id=?",
                                      (data.name, data.float_test, PrimitiveDateTime::new(now.date(), now.time()), id)) {
            return Ok(());
        }
        Err("更新失败")
    }

    pub async fn get_users(query: UserListRequest) -> Result<UserListResponse, &'static str> {
        let list_fn = |result: &mut mysql::QueryResult<mysql::Binary>| -> Vec<UserListData> {
            let mut list = Vec::with_capacity(query.limit as usize);
            if let Some(result_set) = result.iter() {
                for result in result_set {
                    if result.is_ok() {
                        let user_message: (i32, String, f64, PrimitiveDateTime) = from_row(result.unwrap());
                        list.push(UserListData {
                            id: user_message.0,
                            name: user_message.1,
                            float: user_message.2,
                            created_date: user_message.3.assume_utc(),
                        })
                    }
                }
            }
            list
        };
        let user_count_result = User::user_count("SELECT count(*) FROM user ");
        let user_list_result = User::user_list::<(u64, u64), UserListData, _>
            ("SELECT id,name,float_test,create_time FROM user WHERE 1 LIMIT ?,?", (query.page, query.limit),
             list_fn);

        let (user_count_result, user_list_result) = tokio::join!(user_count_result, user_list_result);

        if let (Some(count_result), Some(list_result)) = (user_count_result, user_list_result) {
            let (count, ): (u64, ) = from_row(count_result);
            return Ok(
                UserListResponse {
                    list: list_result,
                    meta: Page {
                        count: count,
                        limit: query.limit,
                        page: query.page,
                    },
                }
            );
        }
        Err("not data")
    }
}