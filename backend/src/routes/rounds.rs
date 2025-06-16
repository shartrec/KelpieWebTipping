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
use std::ops::Add;
use chrono::NaiveDate;
use rocket::Route;
use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::Acquire;
use crate::DbTips;
use crate::models::{game, round, team, tip, tipper};
use crate::models::round::Round;
use crate::models::team::Team;
use crate::util::{game_allocator, ApiError};

pub fn routes() -> Vec<Route> {
    routes![add_round, list, delete /*, update*/, template_round]
}

#[derive(Serialize, Deserialize)]
pub struct NewGame {
    pub home_team_id: i32,
    pub away_team_id: i32,
    pub game_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Default)]
pub struct NewRound {
    pub round_number: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub games: Vec<NewGame>,
}

#[get("/api/rounds")]
pub async fn list(mut pool: Connection<DbTips>) -> Result<Json<Vec<Round>>, ApiError> {
    let rounds = round::get_all(&mut **pool).await?;
    Ok(Json(rounds))
}

#[post("/api/rounds", data = "<new_round>")]
pub async fn add_round(mut pool: Connection<DbTips>, new_round: Json<NewRound>,
) -> Result<Json<Round>, ApiError> {
    let mut tx = pool.begin().await?;

    // Insert round
    let round = round::insert(
        &mut tx,
        new_round.round_number,
        new_round.start_date,
        new_round.end_date,
    ).await?;

    // Insert games
    for g in &new_round.games {
        game::insert(
            &mut tx,
            round.round_id.unwrap_or(-1),
            g.home_team_id,
            g.away_team_id,
            g.game_date,
            None,
            None,
        ).await?;
    }

    tx.commit().await?;
    Ok(Json(round))
}

#[delete("/api/rounds/<id>")]
pub async fn delete(id: i32, mut pool: Connection<DbTips>) -> Result<&'static str, ApiError> {
    let mut tx = pool.begin().await?;

    // Delete all tips for the round
    tip::delete_by_round(&mut tx, id).await?;
    // Delete all games associated with the round
    game::delete_by_round(&mut tx, id).await?;
    // Delete the round itself
    round::delete(&mut tx, id).await?;

    tx.commit().await?;
    Ok("OK")
}

#[get("/api/template_round")]
pub async fn template_round(mut pool: Connection<DbTips>) -> Result<Json<NewRound>, ApiError> {
    // Get the last defined round and set it as the current round to one week later
    let lr = round::get_last_round(&mut **pool).await?;
    let mut round =
        if let Some(last_round) = lr {
            let round_number = &last_round.round_number + 1;
            let start = last_round.start_date.add(chrono::Duration::days(7));
            let end = last_round.end_date.add(chrono::Duration::days(7));

            let teams  = team::get_all(&mut **pool)
                .await?
                .into_iter()
                .collect::<Vec<Team>>();
            let mut game_list = game_allocator::allocate_games(
                -1, // No round ID yet
                &teams,
                start,
                end,
            ).into_iter().map(|g| NewGame {
                home_team_id: g.home_team_id,
                away_team_id: g.away_team_id,
                game_date: g.game_date,
            }).collect();

            NewRound{
                round_number,
                start_date: start,
                end_date: end,
                games: game_list, // No games defined yet
            }
        } else {
            // No rounds defined, set default values
            NewRound::default()
        };

    Ok(Json(round))
}

