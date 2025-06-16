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
use crate::components::icon_button::IconButton;
use crate::components::icons::{cancel_icon, delete_icon, games_icon, save_icon, submit_icon};
use crate::models::team::Team;
use gloo_net::http::Request;
use log::debug;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use crate::{View, ViewContext};

#[derive(Serialize, Deserialize, Clone, Default)]
struct NewGame {
    home_team_id: i32,
    away_team_id: i32,
    game_date: NaiveDate,
    home_score: Option<i32>,
    away_score: Option<i32>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct NewRound {
    round_number: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    games: Vec<NewGame>,
}

#[function_component(AddRound)]
pub fn add_round() -> Html {
    let view_context = use_context::<ViewContext>().expect("ViewContext not found");

    let round = use_state(NewRound::default);
    let games = use_state(|| vec![]);
    let teams = use_state(|| Vec::<Team>::new());

    // Fetch the teams when the component mounts
    {
        let teams = teams.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/admin/api/teams")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(data) = response.json::<Vec<Team>>().await {
                            teams.set(data);
                        } else {
                            debug!("Failed to parse teams response");
                        }
                    }
                    Err(e) => {
                        debug!("Error fetching teams: {}", e);
                    }
                }
            });
            || ()
        });

        let round = round.clone();
        let games = games.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/admin/api/template_round")
                    .send()
                    .await
                {
                    Ok(response) => {
                        if let Ok(data) = response.json::<NewRound>().await {
                            games.set(data.games.clone());
                            round.set(data);
                        } else {
                            debug!("Failed to parse template round response");
                        }
                    }
                    Err(e) => {
                        debug!("Error fetching template round: {}", e);
                    }
                }
            });
            || ()
        });
    }

    let on_add_game = {
        let games = games.clone();
        let round = round.clone();
        Callback::from(move |_| {
            let mut g = (*games).clone();
            let game = NewGame {
                game_date: round.start_date, // Default to start date
                ..Default::default()
            };
            g.push(game);
            games.set(g);
        })
    };

    let on_delete_game = {
        let games = games.clone();
        Callback::from(move |index: usize| {
            let mut g = (*games).clone();
            g.remove(index);
            games.set(g);
        })
    };

    let on_submit = {
        let round = round.clone();
        let games = games.clone();
        Callback::from(move |_| {
            let mut round_data = (*round).clone();
            round_data.games = (*games).clone();
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::post("/admin/api/rounds")
                    .header("Content-Type", "application/json")
                    .json(&round_data)
                    .unwrap()
                    .send()
                    .await;
                if let Ok(r) = resp {
                    if r.ok() {
                        // gloo_dialogs::alert("Round added!");
                    } else {
                        // gloo_dialogs::alert("Error adding round.");
                    }
                }
            });
        })
    };

    let on_cancel = {
        let view = view_context.view.clone();
        Callback::from(move |_| view.set(View::Rounds))
    };

    html! {
        <div>
            <h2>{ "Add a round" }</h2>
            <div style="display: flex; gap: 1rem;">
                <input type="number" placeholder="Round Number"
                    value={round.round_number.to_string()}
                    oninput={Callback::from({
                        let round = round.clone();
                        move |e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            let mut r = (*round).clone();
                            r.round_number = value.parse().unwrap_or(0);
                            round.set(r);
                        }
                    })}
                />
                <input type="date" placeholder="Start Date"
                    value={round.start_date.format("%Y-%m-%d").to_string()}
                    oninput={Callback::from({
                        let round = round.clone();
                        move |e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            let mut r = (*round).clone();
                            r.start_date = NaiveDate::parse_from_str(value.as_str(), "YYYYMMDD").expect("Invalid date format");
                            round.set(r);
                        }
                    })}
                />
                <input type="date" placeholder="End Date"
                    value={round.end_date.format("%Y-%m-%d").to_string()}
                    oninput={Callback::from({
                        let round = round.clone();
                        move |e: InputEvent| {
                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                            let mut r = (*round).clone();
                            r.end_date = NaiveDate::parse_from_str(value.as_str(), "YYYYMMDD").expect("Invalid date format");
                            round.set(r);
                        }
                    })}
                />
            </div>
            <h3>{"Games"}</h3>
            <table>
                <thead>
                    <tr>
                        <th>{ "Home team" }</th>
                        <th>{ "score" }</th>
                        <th>{ "Away team" }</th>
                        <th>{ "score" }</th>
                        <th>{ "Date" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for games.iter().enumerate().map(|(i, game)|
                        html! {
                        <tr>
                            <td>
                                <select onchange={Callback::from({
                                    let games = games.clone();
                                    move |e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                        let mut g = (*games).clone();
                                        g[i].home_team_id = value.parse().unwrap_or(0);
                                        games.set(g);
                                    }
                                })}>
                                    <option value="" selected={game.home_team_id < 1} disabled=true>{ "Select Home Team" }</option>
                                    { for teams.iter().map(|team| {
                                        let selected = team.id == game.home_team_id;
                                        html! {
                                           <option value={team.id.to_string()} selected={selected}>{ &team.nickname.clone() }</option>
                                        }
                                    })}
                                </select>
                            </td>
                            <td>
                                <input type="number" size="1"
                                    value={game.home_score.map_or("".to_string(), |s| s.to_string())}
                                    oninput={Callback::from({
                                    let games = games.clone();
                                        move |e: InputEvent| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let mut g = (*games).clone();
                                            g[i].home_score = if value.is_empty() {
                                                None // Allow clearing the value
                                            } else {
                                                Some(value.parse().unwrap_or(0))
                                            };
                                            games.set(g);
                                        }
                                      })}
                                />
                            </td>
                            <td>
                                <select onchange={Callback::from({
                                    let games = games.clone();
                                    move |e: Event| {
                                        let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                        let mut g = (*games).clone();
                                        g[i].away_team_id = value.parse().unwrap_or(0);
                                        games.set(g);
                                    }
                                })}>
                                    <option value="" selected={game.away_team_id < 1} disabled=true>{ "Select Away Team" }</option>
                                    { for teams.iter().map(|team| {
                                        let selected = team.id == game.away_team_id;
                                        html! {
                                            <option value={team.id.to_string()} selected={selected}>{ &team.nickname.clone() }</option>
                                        }
                                    })}
                                </select>
                            </td>
                            <td>
                                <input type="number" size="1"
                                    value={game.away_score.map_or("".to_string(), |s| s.to_string())}
                                    oninput={Callback::from({
                                    let games = games.clone();
                                        move |e: InputEvent| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let mut g = (*games).clone();
                                            g[i].home_score = if value.is_empty() {
                                                None // Allow clearing the value
                                            } else {
                                                Some(value.parse().unwrap_or(0))
                                            };
                                            games.set(g);
                                        }
                                    })}
                                />
                            </td>
                            <td>
                                <input type="date" placeholder="Game Date"
                                    value={game.game_date.format("%Y-%m-%d").to_string()}
                                    min={round.start_date.format("%Y-%m-%d").to_string()}
                                    max={round.end_date.format("%Y-%m-%d").to_string()}
                                    oninput={Callback::from({
                                        let games = games.clone();
                                        move |e: InputEvent| {
                                            let value = e.target_unchecked_into::<web_sys::HtmlInputElement>().value();
                                            let mut g = (*games).clone();
                                            g[i].game_date = NaiveDate::parse_from_str(value.as_str(), "YYYYMMDD").expect("Invalid date format");
                                            games.set(g);
                                        }
                                    })}
                                />
                            </td>
                            <td>
                                <div class="button-row">
                                    <IconButton onclick={Callback::from({
                                        let on_delete_game = on_delete_game.clone();
                                        move |_| on_delete_game.emit(i)
                                    })} disabled=false>
                                        { delete_icon() }
                                    </IconButton>
                                </div>
                            </td>
                        </tr>
                    })}
                </tbody>
            </table>
                <div style="display: flex; width: 100%; gap: 1rem;">
                    <div class="button-row">
                        <IconButton label="Add Game" onclick={on_add_game}>
                            { games_icon() }
                        </IconButton>
                        <IconButton label="Save" onclick={on_submit}>
                            { save_icon() }
                        </IconButton>
                    </div>
                    <div style="margin-left: auto;">
                        <IconButton label="Cancel" onclick={on_cancel}>
                            { cancel_icon() }
                        </IconButton>
                    </div>
                </div>
        </div>
    }

}