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
use chrono::NaiveDate;
use gloo_net::http::Request;
use log::debug;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TipsProps {
    pub tipper_id: i32,
    pub round: i32, // This is now round_id
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct Game {
    game_id: i32,
    home_team_id: i32,
    away_team_id: i32,
    game_date: Option<NaiveDate>,
    home_score: Option<i32>,
    away_score: Option<i32>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
pub struct RoundWithGames {
    pub round_id: Option<i32>,
    pub round_number: i32,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub games: Vec<Game>,
    // ...other fields if needed...
}

#[function_component(Tips)]
pub fn record_tips(props: &TipsProps) -> Html {
    let round = use_state(|| Option::<RoundWithGames>::None);
    let tips = use_state(|| std::collections::HashMap::<i32, String>::new());

    // Fetch round with games for the round_id
    {
        let round = round.clone();
        let round_id = props.round;
        use_effect_with(round_id, move |_| {
            let round = round.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("admin/api/rounds/{}", round_id);
                let resp = Request::get(&url).send().await;
                match resp {
                    Ok(response) => {
                        if let Ok(json) = response.json::<RoundWithGames>().await {
                            round.set(Some(json));
                        } else {
                            debug!("Failed to parse round data");
                            round.set(Some(RoundWithGames {
                                round_id: Some(round_id),
                                round_number: 0,
                                start_date: None,
                                end_date: None,
                                games: vec![],
                            }));
                        }
                    }
                    Err(_) => {
                        round.set(Some(RoundWithGames {
                            round_id: Some(round_id),
                            round_number: 0,
                            start_date: None,
                            end_date: None,
                            games: vec![],
                        }));
                    }
                }
            });
            || ()
        });
    }

    // Handler for selecting a tip
    let on_tip_select = {
        let tips = tips.clone();
        Callback::from(move |(game_id, team): (i32, String)| {
            let mut new_tips = (*tips).clone();
            new_tips.insert(game_id, team);
            tips.set(new_tips);
        })
    };

    html! {
        <div>
            <h4>{ format!("Record tips for Tipper ID {} in Round {}", props.tipper_id, props.round) }</h4>
            {
                match &*round {
                    None => html! { <p>{ "Loading games..." }</p> },
                    Some(r) if r.games.is_empty() => html! { <p>{ "No games found for this round." }</p> },
                    Some(r) => html! {
                        <ul style="list-style: none; padding: 0;">
                            { for r.games.iter().map(|game| {
                                let selected = tips.get(&game.game_id).cloned();
                                html! {
                                    <li style="margin-bottom: 1em;">
                                        <span style="margin-right: 1em;">{ format!("{} vs {}", game.home_team_id, game.away_team_id) }</span>
                                        <label style="margin-right: 1em;">
                                            <input
                                                type="radio"
                                                name={format!("game-{}", game.game_id)}
                                                checked={selected.as_deref() == Some(&game.home_team_id.to_string())}
                                                onchange={{
                                                    let on_tip_select = on_tip_select.clone();
                                                    let home = game.home_team_id.clone();
                                                    let id = game.game_id;
                                                    Callback::from(move |_| on_tip_select.emit((id, home.to_string())))
                                                }}
                                            />
                                            { &game.home_team_id }
                                        </label>
                                        <label>
                                            <input
                                                type="radio"
                                                name={format!("game-{}", game.game_id)}
                                                checked={selected.as_deref() == Some(&game.away_team_id.to_string())}
                                                onchange={{
                                                    let on_tip_select = on_tip_select.clone();
                                                    let away = game.away_team_id.clone();
                                                    let id = game.game_id;
                                                    Callback::from(move |_| on_tip_select.emit((id, away.to_string())))
                                                }}
                                            />
                                            { &game.away_team_id }
                                        </label>
                                    </li>
                                }
                            })}
                        </ul>
                    }
                }
            }
        </div>
    }
}
