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
#![allow(unused)]
use rocket::serde::Serialize;
use sqlx::PgConnection;

fn leaderboard_sql(where_clause: &str) -> String {
    format!(r#"
        WITH tip_scores AS (
            SELECT
                tips.tipper_id,
                tippers.name AS tipper_name,
                games.round_id,
                tips.game_id,
                CASE
                    WHEN (tips.team_id = games.home_team_id AND games.home_team_score >= games.away_team_score)
                      OR (tips.team_id = games.away_team_id AND games.away_team_score >= games.home_team_score)
                    THEN 1 ELSE 0 END AS score
            FROM tips
            JOIN tippers ON tips.tipper_id = tippers.tipper_id
            JOIN games ON tips.game_id = games.game_id
            {where_clause}
        ),
        round_perfect AS (
            SELECT
                ts.tipper_id,
                ts.round_id,
                r.bonus_points,
                CASE WHEN COUNT(*) = SUM(ts.score) AND COUNT(*) > 0 THEN r.bonus_points ELSE 0 END AS bonus
            FROM tip_scores ts
            JOIN rounds r ON ts.round_id = r.round_id
            GROUP BY ts.tipper_id, ts.round_id, r.bonus_points
        ),
        tipper_scores AS (
            SELECT
                tippers.name AS tipper_name,
                COALESCE(SUM(ts.score),0) AS tip_score
            FROM tippers
            LEFT JOIN tip_scores ts ON tippers.tipper_id = ts.tipper_id
            GROUP BY tippers.name
        ),
        tipper_bonuses AS (
            SELECT
                tippers.name AS tipper_name,
                COALESCE(SUM(rp.bonus),0) AS bonus_score
            FROM tippers
            LEFT JOIN round_perfect rp ON tippers.tipper_id = rp.tipper_id
            GROUP BY tippers.name
        )
        SELECT
            ts.tipper_name,
            ts.tip_score,
            tb.bonus_score,
            (ts.tip_score + tb.bonus_score) AS total_score
        FROM tipper_scores ts
        LEFT JOIN tipper_bonuses tb ON ts.tipper_name = tb.tipper_name
        ORDER BY total_score DESC
    "#, where_clause = where_clause)
}

pub async fn get_leaderboard(pool: &mut PgConnection) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
    let query = leaderboard_sql("");
    let leaderboard = sqlx::query_as::<_, LeaderboardEntry>(&query)
        .fetch_all(pool)
        .await?;
    Ok(leaderboard)
}

pub async fn get_score_by_round(pool: &mut PgConnection, round_id: i32) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
    let query = leaderboard_sql("WHERE games.round_id = $1");
    let leaderboard = sqlx::query_as::<_, LeaderboardEntry>(&query)
        .bind(round_id)
        .fetch_all(pool)
        .await?;

    Ok(leaderboard)
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct LeaderboardEntry {
    pub tipper_name: String,
    pub tip_score: i64,
    pub bonus_score: i64,
    pub total_score: i64,
}