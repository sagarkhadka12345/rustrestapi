use crate::models::country::Country;
use actix_web::{get, web, Responder};

#[get("/show")]
pub async fn get_country_list() -> impl Responder {
    let mut vec: Vec<Country> = Vec::new();

    vec.push(Country {
        country_code: "PH".to_string(),
        country_name: "Philippines".to_string(),
    });
    vec.push(Country {
        country_code: "MY".to_string(),
        country_name: "Malaysia".to_string(),
    });
    vec.push(Country {
        country_code: "ID".to_string(),
        country_name: "Indonesia".to_string(),
    });

    return web::Json(vec);
}
