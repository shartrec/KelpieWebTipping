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
use std::cell::RefCell;
use log::error;
use rocket_db_pools::sqlx;
use rocket_db_pools::sqlx::PgConnection;
use rocket_db_pools::sqlx::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Tipper {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

impl Tipper {

}

pub async fn insert(pool: &mut PgConnection, name: String, email: String) -> Result<Tipper, sqlx::Error> {
    let result = sqlx::query("INSERT INTO tippers (name, email) VALUES ($1, $2) RETURNING tipper_id")
        .bind(name.clone())
        .bind(email.clone())
        .fetch_one(pool)
        .await;
    match result {
        Ok(row) => {
            let id = row.get::<i32, _>(0);
            Ok(crate::models::tipper::Tipper { id: Some(id), name, email})
        },
        Err(e) => {
            error!("Error inserting tipper: {}", e);
            Err(e)
        },
    }
}

pub async fn update(pool: &mut PgConnection, id: i32, name: String, email: String) -> Result<u64, sqlx::Error> {
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

pub async fn delete(pool: &mut PgConnection, id: i32) -> Result<u64, sqlx::Error> {
    let result = sqlx::query("DELETE FROM tippers WHERE tipper_id = $1")
        .bind(id)
        .execute(pool)
        .await;
    match result {
        Ok(result) => {
            Ok(result.rows_affected())
        },
        Err(e) => {
            error!("Error deleting tipper: {}", e);
            Err(e)
        },
    }
}

pub async fn get(pool: &mut PgConnection, id: i32) -> Result<Option<crate::models::tipper::Tipper>, sqlx::Error> {
    let result = sqlx::query("SELECT tipper_id, name, email FROM tippers WHERE tipper_id = $1")
        .bind(id)
        .fetch_optional(pool)
        .await;
    match result {
        Ok(row) => {
            match row {
                Some(row) => {
                    let tipper_id = row.get::<i32, _>(0);
                    let name = row.get::<String, _>(1);
                    let email = row.get::<String, _>(2);
                    Ok(Some(Tipper { id: Some(tipper_id), name, email}))
                },
                None => {
                    Ok(None)
                }
            }
        },
        Err(e) => {
            error!("Error getting tipper: {}", e);
            Err(e)
        },
    }
}

pub async fn get_all(pool: &mut PgConnection) -> Result<Vec<crate::models::tipper::Tipper>, sqlx::Error> {
    let result =
        sqlx::query("SELECT tipper_id, name, email FROM tippers ORDER BY name")
            .fetch_all(pool)
            .await;
    match result {
        Ok(rows) => {
            let mut tippers = Vec::new();
            for row in rows {
                let tipper_id = row.get::<i32, _>(0);
                let name = row.get::<String, _>(1);
                let email = row.get::<String, _>(2);
                tippers.push(Tipper { id: Some(tipper_id), name, email});
            }
            Ok(tippers)
        },
        Err(e) => {
            error!("Error getting all tippers: {}", e);
            Err(e)
        },
    }
}
