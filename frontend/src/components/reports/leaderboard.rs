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
use gloo_net::http::Request;
use log::debug;
use yew::prelude::*;
use serde::Deserialize;
use kelpie_models::round::Round;
use web_sys::{js_sys, Blob, Url};
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use gloo_utils::document;
use crate::components::buttons::IconButton;
use crate::components::icons::{csv_icon, save_icon};

#[derive(Deserialize, Debug, Clone)]
struct LeaderboardEntry {
    tipper_name: String,
    tip_score: i64,
    bonus_score: i64,
    total_score: i64,
}

#[function_component(Leaderboard)]
pub(crate) fn leaderboard() -> Html {
    let leaderboard = use_state(|| vec![]);
    let rounds = use_state(|| Option::<Vec<Round>>::None);
    let selected_round = use_state(|| None::<i32>);

    // Fetch rounds on mount
    {
        let rounds = rounds.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("api/rounds").send().await;
                match resp {
                    Ok(response) => {
                        if let Ok(json) = response.json::<Vec<Round>>().await {
                            rounds.set(Some(json));
                        } else {
                            rounds.set(Some(vec![]));
                        }
                    }
                    Err(_) => {
                        rounds.set(Some(vec![]));
                    }
                }
            });
            || ()
        });
    }

    // Fetch leaderboard when selected_round changes
    {
        let leaderboard = leaderboard.clone();
        let selected_round = selected_round.clone();
        use_effect_with(selected_round, move |selected_round| {
            let leaderboard = leaderboard.clone();
            let selected_round = selected_round.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = if let Some(round_id) = *selected_round {
                    format!("/reports/round/{}", round_id)
                } else {
                    "/reports/leaderboard".to_string()
                };
                match Request::get(&url).send().await {
                    Ok(response) => {
                        if let Ok(entries) = response.json::<Vec<LeaderboardEntry>>().await {
                            leaderboard.set(entries);
                        } else {
                            leaderboard.set(vec![]);
                        }
                    }
                    Err(e) => {
                        debug!("Error fetching leaderboard report: {}", e);
                        leaderboard.set(vec![]);
                    }
                }
            });
            || ()
        });
    }

    // Handle round selection
    let on_round_select = {
        let selected_round = selected_round.clone();
        Callback::from(move |e: Event| {
            let input = e.target_dyn_into::<web_sys::HtmlSelectElement>();
            if let Some(select) = input {
                let value = select.value();
                let round_num = value.parse::<i32>().ok();
                selected_round.set(round_num);
            }
        })
    };

    // Export leaderboard to CSV (Excel-compatible)
    let export_to_excel = {
        let leaderboard = leaderboard.clone();
        Callback::from(move |_| {
            let mut csv = String::from("Tipper,Game Score,Bonus Score,Total Score\n");
            for entry in leaderboard.iter() {
                csv.push_str(&format!(
                    "\"{}\",{},{},{}\n",
                    entry.tipper_name.replace('"', "\"\""),
                    entry.tip_score,
                    entry.bonus_score,
                    entry.total_score
                ));
            }

            // Create a Blob and trigger download
            let window = web_sys::window().unwrap();
            let array = js_sys::Array::new();
            array.push(&JsValue::from_str(&csv));
            let blob = Blob::new_with_str_sequence(&array).unwrap();
            let url = Url::create_object_url_with_blob(&blob).unwrap();

            let document = document();
            let a = document.create_element("a").unwrap();
            a.set_attribute("href", &url).unwrap();
            a.set_attribute("download", "leaderboard.csv").unwrap();
            a.set_attribute("style", "display: none;").unwrap();
            document.body().unwrap().append_child(&a).unwrap();
            let a_html = a.dyn_ref::<web_sys::HtmlElement>().unwrap();
            a_html.click();
            document.body().unwrap().remove_child(&a).unwrap();
            Url::revoke_object_url(&url).unwrap();
        })
    };

    html! {
        <div>
            <h1>{ "Competition Leaderboard" }</h1>
            <div style="display: flex; flex-direction:row; padding: 10px; border-bottom: 1px solid #ccc; align-items: center;">
                <span style="padding-right: 2rem;">{ "Show results for:" }</span>
                {
                    match &*rounds {
                        None => html! { <span>{ "Loading..." }</span> },
                        Some(list) if list.is_empty() => html! { <span>{ "No rounds found" }</span> },
                        Some(list) => html! {
                            <select id="round-select" onchange={on_round_select.clone()} style="width: 15rem;">
                                <option value="" selected={selected_round.is_none()}>{ "Overall" }</option>
                                { for list.iter().map(|round| {
                                    let selected = Some(round.round_id.unwrap_or(0)) == *selected_round;
                                    html! {
                                        <option value={round.round_id.unwrap_or(0).to_string()} selected={selected}>
                                            { format!("Round {}", round.round_number) }
                                        </option>
                                    }
                                })}
                            </select>
                        }
                    }
                }
                <IconButton
                    label={Some("Export".to_string())}
                    onclick={export_to_excel}
                    disabled={false}
                >
                    { csv_icon() }
                </IconButton>
            </div>
            <a href="/" style="display: inline-block; margin-bottom: 1rem;">{ "Back to Main Page" }</a>
            <table>
                <thead>
                    <tr>
                        <th>{ "Tipper" }</th>
                        <th>{ "Game Score" }</th>
                        <th>{ "Bonus Score" }</th>
                        <th>{ "Total Score" }</th>
                    </tr>
                </thead>
                <tbody>
                    { for leaderboard.iter().map(|entry| html! {
                        <tr>
                            <td>{ &entry.tipper_name }</td>
                            <td>{ &entry.tip_score }</td>
                            <td>{ &entry.bonus_score }</td>
                            <td>{ &entry.total_score}</td>
                        </tr>
                    }) }
                </tbody>
            </table>
        </div>
    }
}