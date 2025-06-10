/*
 * Copyright (c) 2025. Trevor Campbell and others.
 *
 * This file is part of KelpieRustWeb.
 *
 * KelpieRustWeb is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieRustWeb is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieRustWeb; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */

mod routes;
mod models;
mod util;

#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::serde::json::Json;
use rocket::response::content::RawHtml;
use crate::models::tipper::Tipper;
use crate::routes::tippers::*;
use crate::util::logging::setup_logging;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[derive(Database)]
#[database("kelpie_db")]
pub struct DbTips(sqlx::PgPool);

#[rocket::launch]
fn rocket() -> _ {
    tracing::info!("Starting server...");

    let rocket = rocket::build()
        .attach(DbTips::init())
        .mount("/", FileServer::from(relative!("../frontend/dist")))
        .mount("/", routes![list, add, update, delete]);
    rocket
}

