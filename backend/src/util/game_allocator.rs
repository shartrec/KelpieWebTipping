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

use kelpie_models::game::Game;
use kelpie_models::team::Team;
use chrono::NaiveDate;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::BTreeMap;

pub(crate) fn allocate_games(round_id: i32, teams: &Vec<Team>, start: NaiveDate, end: NaiveDate) -> Vec<Game> {
    // Create a list of days between start and end dates
    let mut days = Vec::new();
    let mut current_date = start;
    while current_date <= end {
        days.push(current_date); // Store day names
        current_date = current_date.succ_opt().unwrap(); // Move to the next day
    }

    let mut rng = thread_rng();
    let mut shuffled_teams = teams.clone();
    shuffled_teams.shuffle(&mut rng);

    // Pair teams into games
    let mut games = Vec::new();
    for pair in shuffled_teams.chunks(2) {
        if let [a, b] = pair {
            games.push((a.clone(), b.clone()));
        }
    }

    let num_games = games.len();
    let num_days = days.len();
    let base_games_per_day = num_games / num_days;
    let extra_games = num_games % num_days;

    // Determine day order: prioritize middle days for extra games
    let mut day_indices: Vec<usize> = (0..num_days).collect();
    day_indices.sort_by_key(|&i| (i as isize - (num_days as isize / 2)).abs());

    let mut games_per_day = vec![base_games_per_day; num_days];
    for &i in day_indices.iter().take(extra_games) {
        games_per_day[i] += 1;
    }

    // Assign games to days
    let mut schedule: BTreeMap<NaiveDate, Vec<(Team, Team)>> = BTreeMap::new();
    let mut game_iter = games.into_iter();

    for (i, &day) in days.iter().enumerate() {
        let count = games_per_day[i];
        let day_games: Vec<_> = game_iter.by_ref().take(count).collect();
        schedule.insert(day.clone(), day_games);
    }

    // convert to Game objects
    let mut game_objects = Vec::new();
    for (day, day_games) in schedule {
        for (team1, team2) in day_games {
            game_objects.push(Game {
                game_id: None,
                round_id: Some(round_id),
                home_team_id: team1.id.unwrap(),
                away_team_id: team2.id.unwrap(),
                game_date: day.clone(),
                home_team_score: None,
                away_team_score: None,
            });
        }
    }
    game_objects
}
