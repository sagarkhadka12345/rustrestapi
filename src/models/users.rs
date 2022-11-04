use crate::schema::schema::users;
use serde::Serialize;

#[derive(Serialize, Debug, Queryable)]
pub struct User {
    pub id: i32,
    pub name: Option<String>,
    pub rollno: i32,
    pub sec: String,
}

#[derive(Insertable)]
#[table_name = "users"]

pub struct NewUser {
    pub name: Option<String>,
    pub rollno: i32,
    pub sec: String,
}
