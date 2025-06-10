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

// backend/src/routes/tippers.rs
use rocket::serde::json::Json;
use rocket::State;
use sqlx::PgPool;
use crate::models::tipper::Tipper;
use tokio::sync::Mutex;
use crate::{models, DbTips};
use crate::models::tipper;
use crate::routes::tippers;
use crate::util::ApiError;

use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx::{self, Row};

#[get("/api/tippers")]
pub async fn list(mut pool: Connection<DbTips>) ->  Result<Json<Vec<Tipper>>, ApiError> {

    let tippers = tipper::get_all(&mut **pool).await?;
    Ok(Json(tippers))
}

#[post("/api/tippers", data = "<tipper>")]
pub async fn add(tipper: Json<Tipper>, mut pool: Connection<DbTips>) -> Result<Json<Tipper>, ApiError> {
    let new = tipper::insert(&mut **pool, tipper.name.clone(), tipper.email.clone()).await?;
    Ok(Json(new))
}

#[put("/api/tippers/<id>", data = "<tipper>")]
pub async fn update(id: i32, tipper: Json<Tipper>, mut pool: Connection<DbTips>) -> Result<Option<Json<Tipper>>, ApiError> {
    if let Some(id) = tipper.id {
        let count = tipper::update(&mut **pool, id, tipper.name.clone(), tipper.email.clone()).await?;
        match count {
            0 => {
                Err(ApiError::NotFound("Row not found"))
            }
            1 => {
                let new = tipper::insert(&mut **pool, tipper.name.clone(), tipper.email.clone()).await?;
                Ok(Some(Json(new)))
            },
            _ => Err(ApiError::NotFound("Ow"))
        }
    } else {
        Ok(None)
    }
}

#[delete("/api/tippers/<id>")]
pub async fn delete(id: i32, mut pool: Connection<DbTips>) -> Result<&'static str, ApiError> {
    tipper::delete(&mut **pool, id).await?;
    Ok("OK")
}
