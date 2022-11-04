use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub struct Conn {}
impl Conn {
    pub fn establish_connection() -> MysqlConnection {
        dotenv().ok().expect("error in env file");
        let database_url = env::var("DATABASE_URL").expect("No variable with this name");
        return MysqlConnection::establish(&database_url).unwrap();
    }
}
