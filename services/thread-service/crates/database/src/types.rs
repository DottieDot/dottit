use diesel::{
  r2d2::{ConnectionManager, Pool},
  MysqlConnection
};

pub type DbPool = Pool<ConnectionManager<MysqlConnection>>;
