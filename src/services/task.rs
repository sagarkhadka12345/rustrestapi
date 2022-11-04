use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::database::connection::Conn;
use crate::models::task::Task;
use crate::schema::schema::task;
use crate::schema::schema::task::dsl::task as all_tasks;
use actix_web::{get, web, Responder};
use diesel::QueryDsl;
pub fn get_all_tasks() -> Vec<Task> {
    let conn = Conn::establish_connection();
    return all_tasks
        .order(task::id.desc())
        .load::<Task>(&conn)
        .expect("Error in finding Users");
}
#[get("/all")]
pub async fn all() -> impl Responder {
    let data_array = get_all_tasks();
    web::Json(data_array)
}
