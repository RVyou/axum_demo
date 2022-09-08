use std::option::Option;

use mysql::*;
use mysql::prelude::*;

use super::db::POOL;

pub struct User {}

impl User {
    pub fn get_one_user<T: Into<Params>>(query: &str, params: T) -> Option<mysql::Row> {
        let mut connet = if let Ok(c) = POOL.get_connet() {
            c
        } else {
            return None;
        };

        connet.exec_first(query, params).unwrap_or_else(|_e| {
            Option::None
        })
    }

    pub fn modify_one_user<T: Into<Params>>(query: &str, params: T) -> u64 {
        let mut connet = if let Ok(c) = POOL.get_connet() {
            c
        } else {
            return 0;
        };
        if let Ok(result) = connet.exec_iter(query, params) {
            return result.affected_rows();
        }
        0
    }

    pub async fn user_list<T: Into<Params>, B, F>(query: &str, params: T, iter: F) -> Option<Vec<B>>
        where F: Fn(&mut mysql::QueryResult<mysql::Binary>) -> Vec<B> {
        let mut connet = if let Ok(c) = POOL.get_connet() {
            c
        } else {
            return None;
        };
        //exec  query
        if let Ok(mut data) = connet.exec_iter(query, params) {
            return Some(iter(&mut data));
        }
        return None;
    }

    pub async fn user_count(query: &str) -> Option<mysql::Row> {
        let mut connet = if let Ok(c) = POOL.get_connet() {
            c
        } else {
            return None;
        };

        connet.query_first(query).unwrap_or_else(|_e| {
            Option::None
        })
    }
}
