// use crate::models::users::NewUser;
use crate::models::users::User;
use crate::schema::schema::users;
use crate::schema::schema::users::dsl::users as all_users;
use actix_web::{get, post, web, Responder};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
// use diesel::result::Error;
use dotenv::dotenv;
use std::env;

// impl User {
//     pub fn find_books(id: i32, conn: &MysqlConnection) -> Vec<User> {
//         all_users
//             .find(id)
//             .load::<User>(conn)
//             .expect("Error in finding User")
//     }

//     pub fn update_by_id(id: i32, conn: &MysqlConnection, user: NewUser) -> () {
//         use crate::schema::schema::users::dsl::{name as n, rollno as r, sec as s};
//         let NewUser { name, rollno, sec } = user;
//         let update_count = diesel::update(all_users.find(id))
//             .set((n.eq(name), r.eq(rollno), s.eq(sec)))
//             .execute(conn);

//         println!("{:?}", update_count)
//     }
//     pub fn delete_by_id(id: i32, conn: &MysqlConnection) -> Result<usize, Error> {
//         let delete_count = diesel::delete(all_users.find(id)).execute(conn);
//         if delete_count == Ok(0) {
//             return Err(Error::NotFound);
//         }
//         return delete_count;
//     }
// }

pub fn get_all_users() -> Vec<User> {
    dotenv().ok().expect("error in env file");
    let database_url = env::var("DATABASE_URL").expect("No variable with this name");
    let conn = MysqlConnection::establish(&database_url).unwrap();
    return all_users
        .order(users::id.desc())
        .load::<User>(&conn)
        .expect("Error in finding Users");
}

#[post("/all")]
pub async fn all() -> impl Responder {
    let data_array = get_all_users();
    web::Json(data_array)
}

#[get["/ping"]]
pub async fn ping() -> impl Responder {
    web::Json("")
}
