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

use crate::components::tips::Tips;
use gloo_net::http::Request;
use kelpie_models::round::Round;
use kelpie_models::tipper::Tipper;
use yew::prelude::*;

#[function_component(TipView)]
pub fn tip_view() -> Html {
    // State for tippers fetched from backend
    let tippers = use_state(|| Option::<Vec<Tipper>>::None);
    let selected_tipper = use_state(|| None::<usize>);

    // State for rounds and selected round
    let rounds = use_state(|| Option::<Vec<Round>>::None);
    let selected_round = use_state(|| None::<i32>);

    // Fetch tippers from backend on mount
    {
        let tippers = tippers.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("admin/api/tippers").send().await;
                match resp {
                    Ok(response) => {
                        if let Ok(json) = response.json::<Vec<Tipper>>().await {
                            tippers.set(Some(json));
                        } else {
                            tippers.set(Some(vec![]));
                        }
                    }
                    Err(_) => {
                        tippers.set(Some(vec![]));
                    }
                }
            });
            || ()
        });
    }

    // Fetch rounds from backend on mount
    {
        let rounds = rounds.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let resp = Request::get("admin/api/rounds").send().await;
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

    // Define the tipper selection callback
    let on_tipper_select = {
        let selected_tipper = selected_tipper.clone();
        Callback::from(move |idx: usize| selected_tipper.set(Some(idx)))
    };

    html! {
        <div class="content">
            <div style="display: flex; flex-direction:row; border-bottom: 1px solid #ccc;">
                <h3 style="padding-right: 3rem;">{ "Enter tips" }</h3>
                {
                    match &*rounds {
                        None => html! { <span>{ "Loading..." }</span> },
                        Some(list) if list.is_empty() => html! { <span>{ "No rounds found" }</span> },
                        Some(list) => html! {
                            <select id="round-select" onchange={on_round_select.clone()} style="width: 15rem;">
                                <option value="" selected={selected_round.is_none()} disabled=true>{ "Select Round" }</option>
                                { for list.iter().map(|round| {
                                    let selected = Some(round.round_number) == *selected_round;
                                    html! {
                                        <option value={round.round_number.to_string()} selected={selected}>
                                            { format!("Round {}", round.round_number) }
                                        </option>
                                    }
                                })}
                            </select>
                        }
                    }
                }
            </div>
            <div style="display: flex; flex: 1; min-height: 0;">
                // Sidebar for tipper selection
                <div class="scrollable-list" style="border-right: 1px solid #ccc;">
                    <ul>
                    {
                        match &*tippers {
                            None => html! { <li>{ "Loading..." }</li> },
                            Some(list) if list.is_empty() => html! { <li>{ "No tippers found" }</li> },
                            Some(list) => html! {
                                <>
                                    { for list.iter().enumerate().map(|(idx, tipper)| {
                                    let is_selected = *selected_tipper == Some(idx);
                                    let on_click = {
                                        let on_tipper_select = on_tipper_select.clone();
                                        Callback::from(move |_| on_tipper_select.emit(idx))
                                    };
                                    html! {
                                        <li style={ if is_selected { "font-weight: bold;" } else { "" } }>
                                            <button onclick={on_click.clone()} class="key-button" style="width: 90%; background: none; border: none; text-align: left;">
                                                { &tipper.name }
                                            </button>
                                        </li>
                                    }
                                })}
                                </>
                            }
                        }
                    }
                    </ul>
                </div>
                <div style="flex: 1; display: flex; flex-direction: column;">
                    // Main body for tips recording
                    <div style="flex: 1; padding: 1em;">
                        {
                            if let (Some(tipper_list), Some(round_list)) = (&*tippers, &*rounds) {
                                if !tipper_list.is_empty() && selected_tipper.unwrap_or(0) < tipper_list.len() {
                                    if let Some(selected_round_number) = *selected_round {
                                        let tipper = &tipper_list[selected_tipper.unwrap_or(0)];
                                        // Find the round object by round_number
                                        if let Some(round) = round_list.iter().find(|r| r.round_number == selected_round_number) {
                                            let round_id = round.round_id.unwrap_or(0);
                                            let tipper_id = tipper.id.unwrap_or(0);
                                            html! {
                                                <Tips tipper_id={tipper_id} round={round_id} />
                                            }
                                        } else {
                                            html! { <p>{ "Selected round not found." }</p> }
                                        }
                                    } else {
                                        html! { <p>{ "Please select a round." }</p> }
                                    }
                                } else {
                                    html! { <></> }
                                }
                            } else {
                                html! { <></> }
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}