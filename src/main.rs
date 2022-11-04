#[macro_use]
extern crate diesel;
extern crate dotenv;

use std::{process, thread};
mod database;
mod models;
mod schema;
mod services;
use actix_web::{
    web::{self},
    App, HttpServer,
};

use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use services::{
    country::get_country_list,
    task::all as all_task,
    user::{all, ping},
};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations/");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/api").service(
                web::scope("/v1")
                    .service(web::scope("/country").service(get_country_list))
                    .service(web::scope("/user").service(all))
                    .service(web::scope("/task").service(all_task).service(ping)),
            ),
        )
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await?;

    Ok(())
}
