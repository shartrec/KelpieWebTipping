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
use crate::db::reporting::{get_leaderboard, get_score_by_round, LeaderboardEntry};
use crate::DbTips;
use rocket::serde::json::Json;
use rocket::Route;
use rocket_db_pools::Connection;

pub(crate) fn routes() -> Vec<Route> {
    routes![leaderboard, round]
}

#[get("/leaderboard")]
pub async fn leaderboard(mut pool: Connection<DbTips>) -> Json<Vec<LeaderboardEntry>> {
    match get_leaderboard(&mut **pool).await {
        Ok(entries) => Json(entries),
        Err(e) => {
            error!("Error fetching leaderboard: {}", e);
            Json(vec![])
        }, // Handle errors gracefully
    }
}

#[get("/round/<round_id>")]
pub async fn round(mut pool: Connection<DbTips>, round_id: i32) -> Json<Vec<LeaderboardEntry>> {
    match get_score_by_round(&mut **pool, round_id).await {
        Ok(entries) => Json(entries),
        Err(e) => {
            error!("Error fetching leaderboard: {}", e);
            Json(vec![])
        }, // Handle errors gracefully
    }
}
