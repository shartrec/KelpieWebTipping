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
use sqlx::PgConnection;

#[derive(Debug)]
pub struct Tip {
    pub tip_id: Option<i32>,
    pub user_id: Option<i32>,
    pub match_id: Option<i32>,
    pub predicted_home_score: Option<i32>,
    pub predicted_away_score: Option<i32>,
    pub tip_date: Option<NaiveDateTime>,
}

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