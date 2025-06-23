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
use crate::db::team;
use kelpie_models::team::Team;
use crate::util::ApiError;
use crate::DbTips;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

pub(crate) fn routes() -> Vec<Route> {
    routes![list, add, update, delete]
}
#[get("/api/teams")]
pub(crate) async fn list(mut pool: Connection<DbTips>) -> Result<Json<Vec<Team>>, ApiError> {
    let teams = team::get_all(&mut **pool).await?;
    Ok(Json(teams))
}

#[post("/api/teams", data = "<team>")]
pub(crate) async fn add(team: Json<Team>, mut pool: Connection<DbTips>) -> Result<Json<Team>, ApiError> {
    let new = team::insert(&mut **pool, team.name.clone(), team.nickname.clone()).await?;
    Ok(Json(new))
}

#[put("/api/teams", data = "<team>")]
pub(crate) async fn update(team: Json<Team>, mut pool: Connection<DbTips>) -> Result<Json<Team>, ApiError> {
    if let Some(id) = team.id {
        let count = team::update(&mut **pool, id, team.name.clone(), team.nickname.clone()).await?;
        match count {
            0 => Err(ApiError::NotFound("Row not found".to_string())),
            1 => {
                if let Some(new) = team::get(&mut **pool, id).await? {
                    Ok(Json(new))
                } else {
                    Err(ApiError::NotFound("Row not found".to_string()))
                }
            },
            _ => Err(ApiError::Error("Unexpected row count".to_string()))
        }
    } else {
        Err(ApiError::NotFound("Row not found".to_string()))
    }
}

#[delete("/api/teams/<id>")]
pub(crate) async fn delete(id: i32, mut pool: Connection<DbTips>) -> Result<&'static str, ApiError> {
    team::delete(&mut **pool, id).await?;
    Ok("OK")
}