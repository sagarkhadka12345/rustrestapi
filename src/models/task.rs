use serde::Serialize;

#[derive(Serialize, Debug, Queryable)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub class: String,
}
