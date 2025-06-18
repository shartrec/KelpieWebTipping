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

#[macro_use] extern crate rocket;
mod routes;
mod models;
mod util;

use crate::routes::{rounds, teams};
use crate::routes::tippers;
use rocket::fs::{relative, FileServer};

use rocket_db_pools::{sqlx, Database};
use crate::util::logging::setup_logging;

#[derive(Database)]
#[database("kelpie_db")]
pub(crate) struct DbTips(sqlx::PgPool);

#[rocket::launch]
fn rocket() -> _ {
    setup_logging();
    tracing::info!("Starting server...");

    let rocket = rocket::build()
        .attach(DbTips::init())
        .mount("/", FileServer::from(relative!("./static")))
        .mount("/admin", tippers::routes())
        .mount("/admin", teams::routes())
        .mount("/admin", rounds::routes());
    rocket
}

