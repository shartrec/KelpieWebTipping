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

pub(crate) mod logging;

use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::Serialize;
use rocket_db_pools::sqlx;

#[derive(Debug)]
pub enum ApiError {
    Db(sqlx::Error),
    NotFound(&'static str),
}

impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        ApiError::Db(err)
    }
}

#[derive(Debug, Serialize)]
pub struct ApiErrorMessage {
    pub error: String,
}

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, msg) = match self {
            ApiError::NotFound(msg) => (Status::NotFound, msg.to_string()),
            ApiError::Db(e) => (Status::Conflict, e.to_string()),
            // Add other variants as needed
        };
        let body = Json(ApiErrorMessage { error: msg });
        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(body.0.error.len(), std::io::Cursor::new(body.0.error.to_string()))
            .ok()
    }
}
