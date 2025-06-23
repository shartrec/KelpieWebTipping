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
use chrono::NaiveDate;
use log::error;
use rocket::serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgConnection, Row};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Round {
    pub(crate) round_id: Option<i32>,
    pub(crate) round_number: i32,
    pub(crate) start_date: NaiveDate,
    pub(crate) end_date: NaiveDate,
}

pub(crate) async fn insert(pool: &mut PgConnection, round_number: i32, start_date: NaiveDate, end_date: NaiveDate) -> Result<Round, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO rounds (round_number, start_date, end_date) VALUES ($1, $2, $3) RETURNING round_id",
    )
        .bind(round_number)
        .bind(start_date)
        .bind(end_date)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => {
            let id = row.get::<i32, _>(0);
            Ok(Round{round_id: Some(id), round_number, start_date, end_date})
        }
        Err(e) => {
            error!("Error inserting round: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn update(pool: &mut PgConnection, id: i32, round_number: i32, start_date: NaiveDate, end_date: NaiveDate) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE rounds SET round_number=$1, start_date=$2, end_date=$3 WHERE round_id=$4",
    )
        .bind(round_number)
        .bind(start_date)
        .bind(end_date)
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error updating round: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn delete(pool: &mut PgConnection, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM rounds WHERE round_id=$1")
        .bind(id)
        .execute(pool)
        .await;

    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error deleting round: {}", e);
            Err(e)
        }
    }
}

fn build_round(row: PgRow) -> Round {
    let round_id = row.get::<i32, _>(0);
    let round_number = row.get::<i32, _>(1);
    let start_date = row.get::<NaiveDate, _>(2);
    let end_date = row.get::<NaiveDate, _>(3);
    Round{round_id: Some(round_id), round_number, start_date, end_date}
}

pub(crate) async fn get(pool: &mut PgConnection, id: i32) -> Result<Option<Round>, sqlx::Error> {
    let result = sqlx::query("SELECT round_id, round_number, start_date, end_date FROM rounds WHERE round_id=$1")
        .bind(id)
        .fetch_optional(pool)
        .await;

    match result {
        Ok(row) => match row {
            Some(row) => {
                Ok(Some(build_round(row)))
            }
            None => Ok(None),
        },
        Err(e) => {
            error!("Error getting round: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get_last_round (pool: &mut PgConnection) -> Result<Option<Round>, sqlx::Error> {
    let result = sqlx::query("SELECT round_id, round_number, start_date, end_date FROM rounds ORDER BY round_number DESC LIMIT 1")
        .fetch_optional(pool)
        .await;

    match result {
        Ok(row) => match row {
            Some(row) => {
                Ok(Some(build_round(row)))
            }
            None => Ok(None),
        },
        Err(e) => {
            error!("Error getting round: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn round_with_number_exists (pool: &mut PgConnection, round_number: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("SELECT count(*) FROM rounds WHERE round_number = $1 LIMIT 1")
        .bind(round_number)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => {
            let count: i64 = row.get(0);
            Ok(count > 0)
        },
        Err(e) => {
            error!("Error getting round: {}", e);
            Err(e)
        }
    }
}
pub(crate) async fn round_with_number_used (pool: &mut PgConnection, round_id: i32, round_number: i32) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "SELECT count(*) FROM rounds WHERE round_id != $1 AND round_number = $2 LIMIT 1")
        .bind(round_id)
        .bind(round_number)
        .fetch_one(pool)
        .await;

    match result {
        Ok(row) => {
            let count: i64 = row.get(0);
            Ok(count > 0)
        },
        Err(e) => {
            error!("Error getting round: {}", e);
            Err(e)
        }
    }
}

pub(crate) async fn get_all(pool: &mut PgConnection) -> Result<Vec<Round>, sqlx::Error> {
    let result = sqlx::query("SELECT round_id, round_number, start_date, end_date FROM rounds ORDER BY round_number")
        .fetch_all(pool)
        .await;

    match result {
        Ok(rows) => {
            let mut rounds = Vec::new();
            for row in rows {
                rounds.push(build_round(row));
            }
            Ok(rounds)
        }
        Err(e) => {
            error!("Error getting all rounds: {}", e);
            Err(e)
        }
    }
}
