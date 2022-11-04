use serde::Serialize;

#[derive(Serialize, Queryable)]
pub struct Country {
    pub country_code: String,
    pub country_name: String,
}
