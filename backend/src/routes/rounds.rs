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
use crate::models::round::Round;
use crate::models::team::Team;
use crate::models::{game, round, team, tip};
use crate::util::{game_allocator, ApiError};
use crate::DbTips;
use chrono::NaiveDate;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Route;
use rocket_db_pools::Connection;
use sqlx::{Acquire, PgConnection};
use std::ops::Add;

pub(crate) fn routes() -> Vec<Route> {
    routes![add_round, list, delete_round, get_round, update_round, template_round]
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NewGame {
    pub(crate) home_team_id: i32,
    pub(crate) away_team_id: i32,
    pub(crate) game_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct NewRound {
    pub(crate) round_id: Option<i32>,
    pub(crate) round_number: i32,
    pub(crate) start_date: NaiveDate,
    pub(crate) end_date: NaiveDate,
    pub(crate) games: Vec<NewGame>,
}

#[get("/api/rounds")]
pub(crate) async fn list(mut pool: Connection<DbTips>) -> Result<Json<Vec<Round>>, ApiError> {
    let rounds = round::get_all(&mut **pool).await?;
    Ok(Json(rounds))
}

#[post("/api/rounds", data = "<new_round>")]
pub(crate) async fn add_round(mut pool: Connection<DbTips>, new_round: Json<NewRound>,
) -> Result<Json<Round>, ApiError> {
    let mut tx = pool.begin().await?;

    validate_new(&mut tx, &new_round).await?;

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

#[put("/api/rounds", data = "<new_round>")]
pub(crate) async fn update_round(mut pool: Connection<DbTips>, new_round: Json<NewRound>,
) -> Result<&'static str, ApiError> {
    let mut tx = pool.begin().await?;

    validate_existing(&mut tx, &new_round).await?;
    let id = new_round.round_id.unwrap_or(-1);

    // Update round
    let _rows = round::update(
        &mut tx,
        new_round.round_id.unwrap_or(-1),
        new_round.round_number,
        new_round.start_date,
        new_round.end_date,
    ).await?;

    // Replace all games
    game::delete_by_round(&mut tx, id).await?;
    for g in &new_round.games {
        game::insert(
            &mut tx,
            id,
            g.home_team_id,
            g.away_team_id,
            g.game_date,
            None,
            None,
        ).await?;
    }

    tx.commit().await?;
    Ok("OK")
}

#[delete("/api/rounds/<id>")]
pub(crate) async fn delete_round(id: i32, mut pool: Connection<DbTips>) -> Result<&'static str, ApiError> {
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

#[get("/api/rounds/<id>")]
pub(crate) async fn get_round(id: i32, mut pool: Connection<DbTips>) -> Result<Json<NewRound>, ApiError> {
    // Get the last defined round and set it as the current round to one week later
    let round = round::get(&mut **pool, id).await?;
    if let Some(round) = round {

        let games = game::get_for_round(&mut **pool, id).await?
            .into_iter().map(|g| NewGame {
            home_team_id: g.home_team_id,
            away_team_id: g.away_team_id,
            game_date: g.game_date,
        }).collect();

        let round = NewRound{
            round_id: round.round_id,
            round_number: round.round_number,
            start_date: round.start_date,
            end_date: round.end_date,
            games: games, // No games defined yet
        };
        Ok(Json(round))
    } else {
        // No rounds defined, set default values
        Err(ApiError::NotFound(format!("Round with ID {} not found", id)))
    }

}

#[get("/api/template_round")]
pub(crate) async fn template_round(mut pool: Connection<DbTips>) -> Result<Json<NewRound>, ApiError> {
    // Get the last defined round and set it as the current round to one week later
    let lr = round::get_last_round(&mut **pool).await?;
    let round =
        if let Some(last_round) = lr {
            let round_number = &last_round.round_number + 1;
            let start = last_round.start_date.add(chrono::Duration::days(7));
            let end = last_round.end_date.add(chrono::Duration::days(7));

            let teams  = team::get_all(&mut **pool)
                .await?
                .into_iter()
                .collect::<Vec<Team>>();
            let game_list = game_allocator::allocate_games(
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
                round_id: None, // No ID yet
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

async fn validate_existing(pool: &mut PgConnection, round: &Json<NewRound>) -> Result<(), ApiError> {
    if let Some(round_id) = round.round_id {
        // Round number must be > 0 and be unique, i.e. not in database
        if round.round_number <= 0 {
            return Err(ApiError::Invalid("Round number must be greater than 0".to_string()));
        }

        if round::round_with_number_used(&mut *pool, round_id, round.round_number).await? {
            return Err(ApiError::Invalid("Round number already exists".to_string()));
        }

        validate_common(pool, round).await?;
    } else {
        // If no round_id is set, we are creating a new round
        return Err(ApiError::Error("No round id, can't update".to_string()));
    }
    Ok(())
}

async fn validate_new(pool: &mut PgConnection, round: &Json<NewRound>) -> Result<(), ApiError> {
    // Round number must be > 0 and be unique, i.e. not in database
    if round.round_number <= 0{
        return Err(ApiError::Invalid("Round number must be greater than 0".to_string()));
    }

    if round::round_with_number_exists(&mut *pool, round.round_number).await? {
        return Err(ApiError::Invalid("Round number already exists".to_string()));
    }

    validate_common(pool, round).await?;
    Ok(())
}

async fn validate_common(pool: &mut PgConnection, round: &Json<NewRound>)  -> Result<(), ApiError>{
    if round.start_date > round.end_date {
        return Err(ApiError::Invalid("Round date must not be greater than end date".to_string()));
    }

    // Validate the games are set up correctly
    // Check each team is used only once per round
    let mut game_teams = std::collections::HashSet::new();
    for game in round.games.iter() {
        for id in [game.home_team_id, game.away_team_id] {
            if !game_teams.insert(id) {
                let t = team::get(pool, id).await?;
                let team_name = if let Some(t) = t {
                    t.name
                } else {
                    id.to_string()
                };
                return Err(ApiError::Invalid(format!("Team {} is used more than once in this round", team_name)));
            }
        }
        // also check the game date is between the round start and end dates
        if game.game_date < round.start_date || game.game_date > round.end_date {
            let date = game.game_date.format("%Y-%m-%d").to_string();
            let start_date = round.start_date.format("%Y-%m-%d").to_string();
            let end_date = round.end_date.format("%Y-%m-%d").to_string();
            return Err(ApiError::Invalid(
                format!("Game date {} is not between the round start date {} and end date {}",
                        date, start_date, end_date)));
        }
    }
    Ok(())
}
