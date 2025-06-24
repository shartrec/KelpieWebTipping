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
use chrono::NaiveDateTime;
use kelpie_models::tip::Tip;
use sqlx::{PgConnection, Row};


pub(crate) async fn delete_by_round(pool: &mut PgConnection, round_id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tips WHERE game_id in (select game_id from games where round_id=$1)")
        .bind(round_id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            log::error!("Error deleting game: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn insert(pool: &mut PgConnection, tip: &Tip) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("INSERT INTO tips (tipper_id, game_id, team_id) VALUES ($1, $2, $3)")
        .bind(tip.tipper_id)
        .bind(tip.game_id)
        .bind(tip.team_id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            log::error!("Error inserting tip: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn update(pool: &mut PgConnection, tip: &Tip,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("UPDATE tips SET team_id = $1 WHERE tipper_id = $2 AND game_id = $3")
        .bind(tip.team_id)
        .bind(tip.tipper_id)
        .bind(tip.game_id)
        .execute(pool)
        .await;

    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            log::error!("Error updating tip: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get_by_tipper_and_round(
    pool: &mut PgConnection,
    tipper_id: i32,
    round_id: i32,
) -> Result<Vec<Tip>, sqlx::Error> {
    let result = sqlx::query("SELECT tipper_id, game_id, team_id FROM tips WHERE tipper_id = $1 AND game_id IN (SELECT game_id FROM games WHERE round_id = $2)")
        .bind(tipper_id)
        .bind(round_id)
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) => {
            let tips: Vec<Tip> = rows.into_iter()
                .map(|row| Tip {
                    tipper_id: row.get::<i32, _>(0),
                    game_id: row.get::<i32, _>(1),
                    team_id: Some(row.get::<i32, _>(2)),
                })
                .collect();
            Ok(tips)
        }
        Err(e) => {
            log::error!("Error fetching tips: {}", e);
            Err(e)
        }
    }
}