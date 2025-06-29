/*
 * Copyright (c) 2025-2025. Trevor Campbell and others.
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
#![allow(unused)]
use kelpie_models::tipper::Tipper;
use log::error;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::PgConnection;
use rocket_db_pools::sqlx::Row;
use sqlx::postgres::PgRow;

pub(crate) async fn insert(pool: &mut PgConnection, name: String, email: String) -> Result<Tipper, sqlx::Error> {
    let result = sqlx::query("INSERT INTO tippers (name, email) VALUES ($1, $2) RETURNING tipper_id")
        .bind(name.clone())
        .bind(email.clone())
        .fetch_one(pool)
        .await;
    match result {
        Ok(row) => {
            let id = row.get::<i32, _>(0);
            Ok(Tipper { id: Some(id), name, email})
        },
        Err(e) => {
            error!("Error inserting tipper: {}", e);
            Err(e)
        },
    }
}

pub(crate) async fn update(pool: &mut PgConnection, id: i32, name: String, email: String) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("UPDATE tippers SET name=$1, email=$2 WHERE tipper_id = $3")
        .bind(name.clone())
        .bind(email.clone())
        .bind(id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => {
            Ok(result.rows_affected())
        },
        Err(e) => {
            error!("Error updating tipper: {}", e);
            Err(e)
        },
    }
}

pub(crate) async fn delete(pool: &mut PgConnection, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tippers WHERE tipper_id = $1")
        .bind(id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => Ok(result.rows_affected()),
        Err(e) => {
            error!("Error deleting tipper: {}", e);
            Err(e)
        },
    }
}

pub(crate) async fn get(pool: &mut PgConnection, id: i32) -> Result<Option<Tipper>, sqlx::Error> {
    let result = sqlx::query("SELECT tipper_id, name, email FROM tippers WHERE tipper_id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await;
    match result {
        Ok(row) =>  match row {
                Some(row) => {
                    let tipper = from_row(row);
                    Ok(Some(tipper))
                }
                None => Ok(None),
            }
        Err(e) => {
            error!("Error getting tipper: {}", e);
            Err(e)
        },
    }
}

fn from_row(row: PgRow) -> Tipper {
    let tipper_id = row.get::<i32, _>(0);
    let name = row.get::<String, _>(1);
    let email = row.get::<String, _>(2);
    let tipper = Tipper { id: Some(tipper_id), name, email };
    tipper
}

pub(crate) async fn get_all(pool: &mut PgConnection) -> Result<Vec<Tipper>, sqlx::Error> {
    let result =
        sqlx::query("SELECT tipper_id, name, email FROM tippers ORDER BY name")
            .fetch_all(pool)
            .await;
    match result {
        Ok(rows) => {
            let mut tippers = Vec::new();
            for row in rows {
                tippers.push(from_row(row));
            }
            Ok(tippers)
        },
        Err(e) => {
            error!("Error getting all tippers: {}", e);
            Err(e)
        },
    }
}
