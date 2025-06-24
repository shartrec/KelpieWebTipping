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
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Round {
    pub round_id: Option<i32>,
    pub round_number: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl PartialEq for Round {
    fn eq(&self, other: &Self) -> bool {
        self.round_id == other.round_id
    }
}