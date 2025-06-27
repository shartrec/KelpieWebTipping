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
use crate::db::{game, round, team, tip};
use crate::util::{game_allocator, ApiError};
use crate::DbTips;
use kelpie_models::game::Game;
use kelpie_models::round::Round;
use kelpie_models::team::Team;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::Route;
use rocket_db_pools::Connection;
use sqlx::{Acquire, PgConnection};
use std::ops::Add;

pub(crate) fn routes() -> Vec<Route> {
    routes![add_round, list, delete_round, get_round, update_round, template_round]
}

#[derive(Serialize, Deserialize, Default)]
pub(crate) struct NewRound {
    pub(crate) round: Round,
    pub(crate) games: Vec<Game>,
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
        new_round.round.round_number,
        new_round.round.start_date,
        new_round.round.end_date,
        new_round.round.bonus_points,
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

    let round = &new_round.round;
    let id = round.round_id.unwrap_or(-1);

    // Update round
    let _rows = round::update(
        &mut tx,
        round.round_id.unwrap_or(-1),
        round.round_number,
        round.start_date,
        round.end_date,
        round.bonus_points,
    ).await?;

    // Sophisticated game update logic
    use std::collections::{HashMap, HashSet};

    // Fetch existing games from DB
    let existing_games = game::get_for_round(&mut tx, id).await?;
    let mut existing_games_map: HashMap<Option<i32>, &kelpie_models::game::Game> =
        existing_games.iter().map(|g| (g.game_id, g)).collect();

    // Map input games by game_id (if present)
    let mut input_games_map: HashMap<Option<i32>, &Game> =
        new_round.games.iter().filter(|g| g.game_id.is_some()).map(|g| (g.game_id, g)).collect();

    // Update existing games and collect input game_ids
    let mut input_game_ids = HashSet::new();
    for game in &new_round.games {
        if let Some(game_id) = game.game_id {
            input_game_ids.insert(game_id);
            if let Some(_) = existing_games_map.get(&Some(game_id)) {
                // Update game
                game::update(
                    &mut tx,
                    game_id,
                    game.home_team_id,
                    game.away_team_id,
                    game.game_date,
                    game.home_team_score,
                    game.away_team_score,
                ).await?;
            }
        }
    }

    // Insert new games (those without a game_id)
    for game in &new_round.games {
        if game.game_id.is_none() {
            game::insert(
                &mut tx,
                id,
                game.home_team_id,
                game.away_team_id,
                game.game_date,
                game.home_team_score,
                game.away_team_score,
            ).await?;
        }
    }

    // Delete games that are in DB but not in input
    for (game_id, _) in existing_games_map.iter() {
        if let Some(game_id) = game_id {
            if !input_game_ids.contains(game_id) {
                game::delete(&mut tx, *game_id).await?;
            }
        }
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

        let games = game::get_for_round(&mut **pool, id).await?;

        let round = NewRound{
            round: round,
            games: games,
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
            ).into_iter().map(|g| Game {
                game_id: None, // No ID yet
                round_id: None,
                home_team_id: g.home_team_id,
                away_team_id: g.away_team_id,
                game_date: g.game_date,
                home_team_score: None,
                away_team_score: None,
            }).collect();

            let round = Round {
                round_id: None, // No ID yet
                round_number,
                start_date: start,
                end_date: end,
                bonus_points: last_round.bonus_points,
            };
            NewRound {
                round,
                games: game_list,
            }
        } else {
            // No rounds defined, set default values
            NewRound::default()
        };

    Ok(Json(round))
}

async fn validate_existing(pool: &mut PgConnection, round: &Json<NewRound>) -> Result<(), ApiError> {
    let r = &round.round;
    if let Some(round_id) = r.round_id {
        // Round number must be > 0 and be unique, i.e. not in database
        if r.round_number <= 0 {
            return Err(ApiError::Invalid("Round number must be greater than 0".to_string()));
        }

        if round::round_with_number_used(&mut *pool, round_id, r.round_number).await? {
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
    let r = &round.round;
    // Round number must be > 0 and be unique, i.e. not in database
    if r.round_number <= 0{
        return Err(ApiError::Invalid("Round number must be greater than 0".to_string()));
    }

    if round::round_with_number_exists(&mut *pool, r.round_number).await? {
        return Err(ApiError::Invalid("Round number already exists".to_string()));
    }

    validate_common(pool, round).await?;
    Ok(())
}

async fn validate_common(pool: &mut PgConnection, round: &Json<NewRound>)  -> Result<(), ApiError>{
    let r = &round.round;

    if r.start_date > r.end_date {
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
                    t.nickname
                } else {
                    id.to_string()
                };
                return Err(ApiError::Invalid(format!("Team {} is used more than once in this round", team_name)));
            }
        }
        // also check the game date is between the round start and end dates
        if game.game_date < r.start_date || game.game_date > r.end_date {
            let date = game.game_date.format("%Y-%m-%d").to_string();
            let start_date = r.start_date.format("%Y-%m-%d").to_string();
            let end_date = r.end_date.format("%Y-%m-%d").to_string();
            return Err(ApiError::Invalid(
                format!("Game date {} is not between the round start date {} and end date {}",
                        date, start_date, end_date)));
        }
    }
    Ok(())
}
