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
use crate::components::buttons::{IconButton, TipSelector};
use crate::components::icons::{reset_icon, save_icon};
use gloo_net::http::Request;
use kelpie_models::game::Game;
use kelpie_models::round::Round;
use kelpie_models::team::Team;
use kelpie_models::tip::Tip;
use kelpie_models::tipper::Tipper;
use serde::Deserialize;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct TipsProps {
    pub tipper_id: i32,
    pub round_id: i32, // This is now round_id
}

#[derive(Deserialize, PartialEq, Clone, Debug)]
pub struct RoundWithGames {
    pub round: Round,
    pub games: Vec<Game>,
}

// Helper function to load tips for a tipper and round
fn load_tips(
    tipper_id: i32,
    round_id: i32,
    round: &Option<RoundWithGames>,
    game_tips: UseStateHandle<HashMap<i32, Option<i32>>>,
) {
    let round = round.clone();
    wasm_bindgen_futures::spawn_local(async move {
        let url = format!("/api/tips/{}/{}", tipper_id, round_id);
        let tips = match Request::get(&url).send().await {
            Ok(resp) => resp.json::<Vec<Tip>>().await.unwrap_or_else(|_| vec![]),
            Err(_) => vec![],
        };

        let mut tips_map = HashMap::new();
        if tips.is_empty() {
            if let Some(r) = round {
                for g in &r.games {
                    tips_map.insert(g.game_id.unwrap_or(-1), None);
                }
            }
        } else {
            for tip in tips {
                tips_map.insert(tip.game_id, tip.team_id);
            }
        }
        game_tips.set(tips_map);
    });
}

#[function_component(Tips)]
pub fn record_tips(props: &TipsProps) -> Html {
    let round = use_state(|| None::<RoundWithGames>);
    let teams = use_state(|| Vec::<Team>::new());
    let game_tips = use_state(|| HashMap::<i32, Option<i32>>::new());
    let tipper = use_state(|| None::<Tipper>);
    let save_status = use_state(|| None::<String>);

    // Fetch teams once
    {
        let teams = teams.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let data = match Request::get("/api/teams").send().await {
                    Ok(resp) => resp.json::<Vec<Team>>().await.ok().unwrap_or_default(),
                    Err(_) => vec![],
                }; // Handle request error
                teams.set(data);
            });
            || ()
        });
    }

    // Fetch round with games and tips when round_id changes
    {
        let round = round.clone();
        let round_id = props.round_id;
        let tipper_id = props.tipper_id;
        let game_tips = game_tips.clone();
        use_effect_with(round_id, move |&round_id| {
            let round = round.clone();
            let game_tips = game_tips.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let new_round = match Request::get(&format!("api/rounds/{}", round_id)).send().await {
                    Ok(resp) => resp.json::<RoundWithGames>().await.ok(),
                    Err(_) => Some(RoundWithGames { round: Round::default(), games: vec![] }),
                };
                load_tips(tipper_id, round_id, &new_round, game_tips.clone());
                round.set(new_round.clone());
            });
            || ()
        });
    }

    // Fetch tipper details and tips when tipper_id changes
    {
        let tipper = tipper.clone();
        let tipper_id = props.tipper_id;
        let round_id = props.round_id;
        let round = round.clone();
        let game_tips = game_tips.clone();
        use_effect_with(tipper_id, move |&tipper_id| {
            let tipper = tipper.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let data = match Request::get(&format!("/api/tippers/{}", tipper_id)).send().await {
                    Ok(resp) => resp.json::<Tipper>().await.ok(),
                    Err(_) => None,
                };
                tipper.set(data);
                load_tips(tipper_id, round_id, &round, game_tips.clone());
            });
            || ()
        });
    }

    let update_tip = {
        let game_tips = game_tips.clone();
        Callback::from(move |(game_id, team_id)| {
            let mut updated = (*game_tips).clone();
            updated.insert(game_id, Some(team_id));
            game_tips.set(updated);
        })
    };

    let reset_tips = {
        let tipper_id = props.tipper_id;
        let round_id = props.round_id;
        let round = round.clone();
        let game_tips = game_tips.clone();
        Callback::from(move |_| {
            load_tips(tipper_id, round_id, &round, game_tips.clone());
        })
    };

    let save_tips = {
        let game_tips = game_tips.clone();
        let round = round.clone();
        let tipper_id = props.tipper_id;
        let round_id = props.round_id;
        let save_status = save_status.clone();
        Callback::from(move |_| {
            let all_tipped = if let Some(r) = &*round {
                r.games.iter().all(|g| {
                    let game_id = g.game_id.unwrap_or(-1);
                    game_tips.get(&game_id).and_then(|t| *t).is_some()
                })
            } else {
                false
            };
            if !all_tipped {
                web_sys::window()
                    .and_then(|w| w.alert_with_message("Please enter a tip for every game.").ok());
                return;
            }
            let tips: Vec<Tip> = if let Some(r) = &*round {
                r.games.iter().map(|g| {
                    Tip {
                        tipper_id,
                        game_id: g.game_id.unwrap_or(-1),
                        team_id: game_tips.get(&g.game_id.unwrap_or(-1)).and_then(|t| *t),
                    }
                }).collect()
            } else {
                vec![]
            };
            let save_status = save_status.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/tips/{}/{}", tipper_id, round_id);
                let resp = Request::post(&url)
                    .json(&tips)
                    .unwrap()
                    .send()
                    .await;
                match resp {
                    Ok(r) if r.ok() => {
                        save_status.set(Some("Tips saved!".to_string()));
                        let save_status = save_status.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            save_status.set(None);
                        }).forget();
                    }
                    _ => {
                        save_status.set(Some("Failed to save tips.".to_string()));
                        let save_status = save_status.clone();
                        gloo_timers::callback::Timeout::new(2000, move || {
                            save_status.set(None);
                        }).forget();
                    }
                }
            });
        })
    };

    html! {
        <div>
            <div style="display: flex; align-items: center; gap: 1rem;">
                <h4 >
                    {
                        match &*tipper {
                            Some(t) => {
                                let r_name = match &*round {
                                    Some(r) => r.round.round_number.to_string(),
                                    _ => "??".to_string(),
                                };
                                format!("Record tips for {} in Round {}", t.name, r_name)
                            }
                            None => format!("Record tips for Tipper ID {} in Round {}", props.tipper_id, props.round_id)
                        }
                    }
                </h4>
               <div style="margin-left: 7rem; display: flex; gap: 0.5rem;">
                    <IconButton label="Save" onclick={save_tips.clone()}>{ save_icon() }</IconButton>
                    <IconButton label="Reset" onclick={reset_tips.clone()}>{ reset_icon() }</IconButton>
                </div>
                if let Some(msg) = &*save_status {
                    <div style="margin: 0.5rem 0; color: #388e3c; font-weight: bold;">
                        { msg }
                    </div>
                }
            </div>
            {
                match &*round {
                    None => html! { <p>{ "Loading games..." }</p> },
                    Some(r) if r.games.is_empty() => html! { <p>{ "No games found for this round." }</p> },
                    Some(r) => html! {
                        <ul style="list-style: none; padding: 0;">
                            { for r.games.iter().map(|game| {
                                let home = teams.iter().find(|t| t.id == Some(game.home_team_id)).cloned();
                                let away = teams.iter().find(|t| t.id == Some(game.away_team_id)).cloned();
                                let selected = game_tips.get(&game.game_id.unwrap_or(-1)).and_then(|t| *t);
                                let radio_name = format!("tip-game-{}", game.game_id.unwrap_or(-1));
                                html! {
                                    <li style="margin-bottom: 1rem;">
                                        <TipSelector
                                            name={radio_name}
                                            home_team={home.clone().unwrap()}
                                            away_team={away.clone().unwrap()}
                                            selected_team_id={selected}
                                            on_change={Callback::from({
                                                let update_tip = update_tip.clone();
                                                let game_id = game.game_id.unwrap();
                                                move |team_id| update_tip.emit((game_id, team_id))
                                            })}
                                        />
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
