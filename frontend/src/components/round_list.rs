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

use crate::components::icons::{delete_icon, edit_icon, rounds_icon};
use crate::{View, ViewContext};
use gloo_net::http::Request;
use kelpie_models::round::Round;
use yew::prelude::*;
use crate::components::buttons::IconButton;

#[function_component(RoundList)]
pub fn round_list() -> Html {
    let view_context = use_context::<ViewContext>().expect("ViewContext not found");

    let error_msg = use_state(|| None::<String>);
    let rounds = use_state(|| vec![]);

    // Load rounds
    {
        let rounds = rounds.clone();
        let error_msg = error_msg.clone();
        use_effect_with((), move |_| {
            // Clear error on load
            error_msg.set(None);
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/api/rounds").send().await {
                    Ok(resp) => {
                        if resp.ok() {
                            match resp.json::<Vec<Round>>().await {
                                Ok(data) => {
                                    rounds.set(data);
                                    error_msg.set(None); // Clear error on success
                                }
                                Err(e) => {
                                    error_msg.set(Some(format!("Failed to parse rounds: {}", e)));
                                }
                            }
                        } else {
                            let status = resp.status();
                            let text = resp.text().await.unwrap_or_default();
                            error_msg.set(Some(format!("Failed to load rounds ({}): {}", status, text)));
                        }
                    }
                    Err(e) => {
                        error_msg.set(Some(format!("Error loading rounds: {}", e)));
                    }
                }
            });
            || ()
        });
    }

    let edit_round= {
        let view = view_context.view.clone();
        Callback::from(move |id: i32| view.set(View::RoundEdit{ round_id: Some(id) }))
    };

    let delete_round = {
        // Add your delete logic here
        let rounds = rounds.clone();
        let error_msg = error_msg.clone();
        Callback::from(move |id: i32| {
            let rounds = rounds.clone();
            let error_msg = error_msg.clone();
            // Show confirm dialog before proceeding
            if web_sys::window()
                .and_then(|w| w.confirm_with_message("Are you sure you want to delete this round?\nThis will remove all games and tips for the round.").ok())
                .unwrap_or(false)
            {
                // Clear error before delete
                error_msg.set(None);
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/rounds/{}", id);
                    match Request::delete(&url).send().await {
                        Ok(resp) => {
                            if resp.ok() {
                                let updated: Vec<Round> = (*rounds).clone().into_iter().filter(|t| t.round_id.unwrap() != id).collect();
                                rounds.set(updated);
                                error_msg.set(None); // Clear error on success
                            } else {
                                let status = resp.status();
                                let text = resp.text().await.unwrap_or_default();
                                error_msg.set(Some(format!("Delete failed ({}): {}", status, text)));
                            }
                        }
                        Err(e) => {
                            error_msg.set(Some(format!("Error deleting round: {}", e)));
                        }
                    }
                });
            }
        })
    };

    let add_round = {
        let view = view_context.view.clone();
        Callback::from(move |_| view.set(View::RoundEdit{round_id: None}))
    };

    html! {
        <div class="content">
            if let Some(msg) = &*error_msg {
                <div class="alert">{ msg }</div>
            }
            <h2>{ "Rounds" }</h2>
            <div class="scrollable-table" style="border-right: 1px solid #ccc;">
            <table>
                <thead>
                    <tr>
                        <th>{ "Round" }</th>
                        <th>{ "From" }</th>
                        <th>{ "To" }</th>
                        <th>{ "Bonus" }</th>
                        <th></th>
                    </tr>
                </thead>
                <tbody>
                    { for (*rounds).iter().map(|round| {
                        let round = round.clone();
                        let start_date = round.start_date.format("%Y-%m-%d").to_string();
                        let end_date = round.end_date.format("%Y-%m-%d").to_string();
                        let delete = {
                            let delete_round = delete_round.clone();
                            // Use Callback::from to create a callback that captures the round ID
                            Callback::from(move |_| {
                                if let Some(id) = round.round_id {
                                    delete_round.emit(id);
                                }
                            })
                        };
                        let do_edit = {
                            let edit_round = edit_round.clone();
                            // Use Callback::from to create a callback that captures the round ID
                            Callback::from(move |_| {
                                if let Some(id) = round.round_id {
                                    edit_round.emit(id);
                                }
                            })
                        };
                            html! {
                                <tr key={round.round_id.unwrap_or(-1)}>
                                    <td>{ &round.round_number }</td>
                                    <td>{ &start_date }</td>
                                    <td>{ &end_date }</td>
                                    <td>{ &round.bonus_points }</td>
                                    <td  class="actions">
                                        <div class="button-row">
                                            <IconButton onclick={do_edit}>
                                                { edit_icon() }
                                            </IconButton>
                                            <IconButton onclick={delete}>
                                                { delete_icon() }
                                            </IconButton>
                                        </div>
                                    </td>
                                </tr>
                            }
                    })}
                </tbody>
            </table>
        </div>
            <div class="button-row">
                <IconButton label="Add" onclick={add_round}>
                    { rounds_icon() }
                </IconButton>
            </div>
        </div>
    }
}