use diesel::mysql::MysqlConnection;

use diesel::result::Error;
use diesel::sql_types::Integer;

use crate::Database::connection::connection;
use actix_web::get;
use actix_web::{web, Responder};

use diesel::prelude::*;
use diesel::{self};
use dotenv::dotenv;
use serde::Serialize;
use std::env;
