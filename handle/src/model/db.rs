use lazy_static::lazy_static;
use mysql::*;

#[derive(Clone, Debug)]
pub struct Mysql {
    pool: mysql::Pool,
}

lazy_static! {
    pub static ref POOL: Mysql = Mysql::new("mysql://root:mysqlpw@192.168.43.147:49153/test",10,30);
}

impl Mysql {
    pub fn new(url:&str,min:usize,max:usize) -> Mysql {
        // let url = "mysql://root:mysqlpw@127.0.0.1:49153/test";
        let pool = Pool::new_manual(min, max, url).unwrap(); // 底层实现 clone 是线程、协程安全的
        Mysql { pool }
    }

    pub fn get_connet(&self) -> Result<mysql::PooledConn> {
        self.pool.clone().get_conn()
    }
}
