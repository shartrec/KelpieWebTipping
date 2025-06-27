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
use rocket::serde::Serialize;
use sqlx::PgConnection;

pub async fn get_leaderboard(pool: &mut PgConnection) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
    let query = r#"
        SELECT tippers.name AS tipper_name, COUNT(*) AS total_score
        FROM tips
        JOIN tippers ON tips.tipper_id = tippers.tipper_id
        JOIN games ON tips.game_id = games.game_id
        WHERE (
            (tips.team_id = games.home_team_id AND games.home_team_score >= games.away_team_score)
            OR
            (tips.team_id = games.away_team_id AND games.away_team_score >= games.home_team_score)
        )
        GROUP BY tippers.name
        ORDER BY total_score DESC
    "#;

    let leaderboard = sqlx::query_as::<_, LeaderboardEntry>(query)
        .fetch_all(pool)
        .await?;

    Ok(leaderboard)
}

pub async fn get_score_by_round(pool: &mut PgConnection, round_id: i32) -> Result<Vec<LeaderboardEntry>, sqlx::Error> {
    let query = r#"
        SELECT tippers.name AS tipper_name, COUNT(*) AS total_score
        FROM tips
        JOIN tippers ON tips.tipper_id = tippers.tipper_id
        JOIN games ON tips.game_id = games.game_id
        WHERE (
            games.round_id = $1 AND
            (
                (tips.team_id = games.home_team_id AND games.home_team_score >= games.away_team_score)
                OR
                (tips.team_id = games.away_team_id AND games.away_team_score >= games.home_team_score)
            )
        )
        GROUP BY tippers.name
        ORDER BY total_score DESC
    "#;

    let leaderboard = sqlx::query_as::<_, LeaderboardEntry>(query)
        .bind(round_id)
        .fetch_all(pool)
        .await?;

    Ok(leaderboard)
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct LeaderboardEntry {
    pub tipper_name: String,
    pub total_score: i64,
}