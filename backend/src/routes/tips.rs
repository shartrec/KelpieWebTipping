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
use crate::db::tip;
use crate::util::ApiError;
use crate::DbTips;
use kelpie_models::tip::Tip;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;
use sqlx::Acquire;

pub(crate) fn routes() -> Vec<Route> {
    routes![get_tips_for_round, save_tips_for_round, tips_exist]
}

#[get("/api/tips/exists/round/<round_id>")]
pub(crate) async fn tips_exist(round_id: i32, mut pool: Connection<DbTips>,
) -> Result<Json<bool>, ApiError> {
    let tips = tip::exist_for_round(&mut **pool,round_id).await?;
    Ok(Json(tips))
}

#[get("/api/tips/<tipper_id>/<round_id>")]
pub(crate) async fn get_tips_for_round(tipper_id: i32, round_id: i32, mut pool: Connection<DbTips>,
) -> Result<Json<Vec<Tip>>, ApiError> {
    let tips = tip::get_by_tipper_and_round(&mut **pool, tipper_id, round_id).await?;
    Ok(Json(tips))
}

#[post("/api/tips/<_tipper_id>/<_round_id>", data = "<tips>")]
pub(crate) async fn save_tips_for_round(
    _tipper_id: i32,
    _round_id: i32,
    mut pool: Connection<DbTips>,
    tips: Json<Vec<Tip>>,
) -> Result<&'static str, ApiError> {
    let mut tx = pool.begin().await?;

    // Insert games
    for t in &tips.0 {
        // Try update
        let rows_affected = tip::update(&mut tx, t).await?;
        if rows_affected == 0 {
            // If no rows were affected, insert
            tip::insert(&mut tx, t).await?;
        }
    }
    tx.commit().await?;
    Ok("OK")
}
