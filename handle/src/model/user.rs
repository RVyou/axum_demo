use std::option::Option;

use mysql::*;
use mysql::prelude::*;

use super::db::POOL;

pub struct User {}

impl User {
    pub fn get_one_user<T: Into<Params>>(query: &str, params: T) -> Option<mysql::Row> {
        let mut connet = if let Ok(c) = POOL.clone().get_connet() {
            c
        } else {
            return None;
        };

        connet.exec_first(query, params).unwrap_or_else(|_e| {
            Option::None
        })
    }

    pub fn modify_one_user<T: Into<Params>>(query: &str, params: T) -> Option<mysql::Row> {
        let mut connet = if let Ok(c) = POOL.clone().get_connet() {
            c
        } else {
            return None;
        };

        connet.exec_first(query, params).unwrap_or_else(|_e| {
            Option::None
        })
    }
}
