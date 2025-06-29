/*
 * Copyright (c) 2025. Trevor Campbell and others.
 *
 * This file is part of KelpieTipping.
 *
 * KelpieTipping is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License,or
 * (at your option) any later version.
 *
 * KelpieTipping is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
 * See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with KelpieTipping; if not, write to the Free Software
 * Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA  02111-1307  USA
 *
 * Contributors:
 *      Trevor Campbell
 *
 */
#![allow(unused)]
use chrono::NaiveDate;
use kelpie_models::game::Game;
use log::error;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::PgConnection;
use rocket_db_pools::sqlx::Row;

pub(crate) async fn insert(
    pool: &mut PgConnection,
    round_id: i32,
    home_team_id: i32,
    away_team_id: i32,
    game_date: NaiveDate,
    home_team_score: Option<i32>,
    away_team_score: Option<i32>,
) -> Result<Game, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO games (round_id, home_team_id, away_team_id, game_date, home_team_score, away_team_score) \
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING game_id",
    )
        .bind(round_id)
        .bind(home_team_id)
        .bind(away_team_id)
        .bind(game_date)
        .bind(home_team_score)
        .bind(away_team_score)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => {
            let id = row.get::<i32, _>(0);
            Ok(Game {game_id: Some(id), round_id: Some(round_id), home_team_id, away_team_id, game_date, home_team_score, away_team_score })
        }
        Err(e) => {
            error!("Error inserting game: {}", e);
            Err(e)
        }
    }
}


pub(crate) async fn update(
    pool: &mut PgConnection,
    game_id: i32,
    home_team_id: i32,
    away_team_id: i32,
    game_date: NaiveDate,
    home_team_score: Option<i32>,
    away_team_score: Option<i32>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE games SET home_team_id=$1, away_team_id=$2, game_date=$3, \
         home_team_score=$4, away_team_score=$5 WHERE game_id=$6",
    )
        .bind(home_team_id)
        .bind(away_team_id)
        .bind(game_date)
        .bind(home_team_score)
        .bind(away_team_score)
        .bind(game_id)
        .execute(pool)
        .await;

    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error updating game: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn delete(pool: &mut PgConnection, game_id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM games WHERE game_id=$1")
        .bind(game_id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error deleting game: {}", e);
            Err(e)
        }
    }
}

fn from_row(row: &sqlx::postgres::PgRow) -> Game {
    Game {
        game_id: Some(row.get::<i32, _>(0)),
        round_id: Some(row.get::<i32, _>(1)),
        home_team_id: row.get::<i32, _>(2),
        away_team_id: row.get::<i32, _>(3),
        game_date: row.get::<NaiveDate, _>(4),
        home_team_score: row.get::<Option<i32>, _>(5),
        away_team_score: row.get::<Option<i32>, _>(6),
    }
}

pub(crate) async fn get(pool: &mut PgConnection, game_id: i32) -> Result<Option<Game>, sqlx::Error> {
    let result = sqlx::query(
        "SELECT game_id, round_id, home_team_id, away_team_id, game_date, home_team_score, away_team_score \
         FROM games WHERE game_id=$1",
    )
        .bind(game_id)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(row) => match row {
            Some(row) => Ok(Some(from_row(&row))),
            None => Ok(None),
        },
        Err(e) => {
            error!("Error getting game: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get_for_round(pool: &mut PgConnection, round_id: i32) -> Result<Vec<Game>, sqlx::Error> {
    let result = sqlx::query(
        "SELECT game_id, round_id, home_team_id, away_team_id, game_date, home_team_score, away_team_score \
         FROM games WHERE round_id = $1 ORDER BY game_date",
    )
        .bind(round_id)
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) => {
            let games = rows.into_iter().map(|row| from_row(&row)).collect();
            Ok(games)
        }
        Err(e) => {
            error!("Error getting all games: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get_all(pool: &mut PgConnection) -> Result<Vec<Game>, sqlx::Error> {
    let result = sqlx::query(
        "SELECT game_id, round_id, home_team_id, away_team_id, game_date, home_team_score, away_team_score \
         FROM games ORDER BY game_date",
    )
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) => {
            let games = rows.into_iter().map(|row| from_row(&row)).collect();
            Ok(games)
        }
        Err(e) => {
            error!("Error getting all games: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn delete_by_round(pool: &mut PgConnection, round_id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM games WHERE round_id=$1")
        .bind(round_id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error deleting game: {}", e);
            Err(e)
        }
    }
}