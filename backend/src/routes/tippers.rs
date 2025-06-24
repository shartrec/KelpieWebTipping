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

use crate::db::tipper;
use kelpie_models::tipper::Tipper;
use crate::util::ApiError;
use crate::DbTips;
use rocket::serde::json::Json;
use rocket::Route;

use rocket_db_pools::Connection;

pub(crate) fn routes() -> Vec<Route> {
    routes![list, add, update, delete, get]
}

#[get("/api/tippers")]
pub(crate) async fn list(mut pool: Connection<DbTips>) ->  Result<Json<Vec<Tipper>>, ApiError> {

    let tippers = tipper::get_all(&mut **pool).await?;
    Ok(Json(tippers))
}

#[post("/api/tippers", data = "<tipper>")]
pub(crate) async fn add(tipper: Json<Tipper>, mut pool: Connection<DbTips>) -> Result<Json<Tipper>, ApiError> {
    let new = tipper::insert(&mut **pool, tipper.name.clone(), tipper.email.clone()).await?;
    Ok(Json(new))
}

#[put("/api/tippers", data = "<tipper>")]
pub(crate) async fn update(tipper: Json<Tipper>, mut pool: Connection<DbTips>) -> Result<Json<Tipper>, ApiError> {
    if let Some(id) = tipper.id {
        let count = tipper::update(&mut **pool, id, tipper.name.clone(), tipper.email.clone()).await?;
        match count {
            0 => {
                Err(ApiError::NotFound("Row not found".to_string()))
            }
            1 => {
                if let Some(new) = tipper::get(&mut **pool, id).await? {
                    Ok(Json(new))
                } else {
                    Err(ApiError::NotFound("Row not found".to_string()))
                }
            },
            _ => Err(ApiError::Error("Multiple rows updated".to_string()))
        }
    } else {
        Err(ApiError::NotFound("Row not found".to_string()))
    }
}

#[delete("/api/tippers/<id>")]
pub(crate) async fn delete(id: i32, mut pool: Connection<DbTips>) -> Result<&'static str, ApiError> {
    tipper::delete(&mut **pool, id).await?;
    Ok("OK")
}

#[get("/api/tippers/<id>")]
pub(crate) async fn get(id: i32, mut pool: Connection<DbTips>) -> Result<Json<Tipper>, ApiError> {
    match tipper::get(&mut **pool, id).await? {
        Some(tipper) => Ok(Json(tipper)),
        None => Err(ApiError::NotFound("Tipper not found".to_string())),
    }
}
