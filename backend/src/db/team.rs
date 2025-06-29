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
use kelpie_models::team::Team;
use log::error;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::PgConnection;
use rocket_db_pools::sqlx::Row;

pub(crate) async fn insert(pool: &mut PgConnection, name: String, nickname: String) -> Result<Team, sqlx::Error> {
    let result =
        sqlx::query("INSERT INTO teams (name, nickname) VALUES ($1, $2) RETURNING team_id")
            .bind(name.clone())
            .bind(nickname.clone())
            .fetch_one(pool)
            .await;
    match result {
        Ok(row) => {
            let id = row.get::<i32, _>(0);
            Ok(Team { id: Some(id), name, nickname, can_delete: Some(true) })
        }
        Err(e) => {
            error!("Error inserting team: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn update(pool: &mut PgConnection, id: i32, name: String, nickname: String) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("UPDATE teams SET name=$1, nickname=$2 WHERE team_id = $3")
        .bind(name.clone())
        .bind(nickname.clone())
        .bind(id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error updating team: {}", e);
            Err( e)
        }
    }
}

pub(crate) async fn delete(pool: &mut PgConnection, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM teams WHERE team_id = $1")
        .bind(id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error deleting team: {}", e);
            Err(e)
        }
    }
}

fn from_row(row: &sqlx::postgres::PgRow) -> Team {
    let id = row.get::<i32, _>(0);
    let name = row.get::<String, _>(1);
    let nickname = row.get::<String, _>(2);
    // can_delete is optional, only present in some queries
    let can_delete = if row.len() > 3 {
        Some(!row.get::<bool, _>(3))
    } else {
        None
    };
    Team { id: Some(id), name, nickname, can_delete }
}

pub(crate) async fn get(pool: &mut PgConnection, id: i32) -> Result<Option<Team>, sqlx::Error> {
    let result = sqlx::query(
        "SELECT team_id, name, nickname FROM teams WHERE team_id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await;
    match result {
        Ok(row) => match row {
            Some(row) => Ok(Some(from_row(&row))),
            None => Ok(None),
        },
        Err(e) => {
            error!("Error getting team: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get_all(pool: &mut PgConnection) -> Result<Vec<Team>, sqlx::Error> {
    let result = sqlx::query(
        "SELECT teams.*, exists(SELECT 1 FROM games WHERE teams.team_id = games.away_team_id
                  OR teams.team_id = games.home_team_id) AS prohibit_delete FROM teams
                  ORDER BY teams.name")
        .fetch_all(pool)
        .await;
    match result {
        Ok(rows) => {
            let teams = rows.into_iter().map(|row| from_row(&row)).collect();
            Ok(teams)
        }
        Err(e) => {
            error!("Error getting all teams: {}", e);
            Err(e)
        }
    }
}
