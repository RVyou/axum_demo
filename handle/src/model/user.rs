use super::db::POOL;
use mysql::PooledConn;
use mysql::prelude::Queryable;



#[derive(Debug, PartialEq, Eq)]
pub struct User {
    id: i32,
    name: Option<String>,
}

impl User {
    pub fn get_user_by_id(id: i32) -> Result<User,()> {
        let mut connet = POOL.get_connet().unwrap();

        // let selected_payments = connet
        //     .query_first_opt("SELECT * from table_name ")
        //     .unwrap();

        Ok(User {
            id: 1,
            name: Some("aa".to_string()),
        })
    }
}
