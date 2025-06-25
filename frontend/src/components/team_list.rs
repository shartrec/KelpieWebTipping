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

use crate::components::buttons::IconButton;
use crate::components::icons::{add_icon, cancel_icon, delete_icon, edit_icon, save_icon};
use gloo_net::http::Request;
use kelpie_models::team::Team;
use log::warn;
use serde_json::json;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TeamListProps {
    pub set_error_msg: Callback<Option<String>>,
}

#[function_component(TeamList)]
pub fn team_list(props: &TeamListProps) -> Html {
    let teams = use_state(|| vec![]);
    let name_input = use_state(|| String::new());
    let nickname_input = use_state(|| String::new());

    // New state for editing
    let editing_id = use_state(|| None as Option<i32>);
    let edit_name = use_state(|| String::new());
    let edit_nickname = use_state(|| String::new());

    // Load teams
    {
        let teams = teams.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/teams").send().await {
                    if let Ok(data) = resp.json::<Vec<Team>>().await {
                        teams.set(data);
                    }
                }
            });
            || ()
        });
    }

    // Add team
    let add_team = {
        let name_input = name_input.clone();
        let nickname_input = nickname_input.clone();
        let teams = teams.clone();

        Callback::from(move |_e: MouseEvent| {
            let name = name_input.clone();
            let nickname = nickname_input.clone();
            let teams = teams.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let payload = json!({
                    "name": (*name).clone(),
                    "nickname": (*nickname).clone(),
                });

                if let Ok(req) = Request::post("/api/teams")
                    .header("Content-Type", "application/json")
                    .body(payload.to_string())
                {
                    if let Ok(resp) = req.send().await
                    {
                        if let Ok(new_team) = resp.json::<Team>().await {
                            let mut new_list = (*teams).clone();
                            new_list.push(new_team);
                            teams.set(new_list);
                            name.set(String::new());
                            nickname.set(String::new());
                        }
                    }
                }
            });
        })
    };

    // Start editing
    let start_edit = {
        let editing_id = editing_id.clone();
        let edit_name = edit_name.clone();
        let edit_nickname = edit_nickname.clone();
        Callback::from(move |team: Team| {
            editing_id.set(team.id);
            edit_name.set(team.name.clone());
            edit_nickname.set(team.nickname.clone());
        })
    };

    // Cancel editing
    let cancel_edit = {
        let editing_id = editing_id.clone();
        Callback::from(move |_| editing_id.set(None))
    };

    // Save edit
    let save_edit = {
        let editing_id = editing_id.clone();
        let edit_name = edit_name.clone();
        let edit_nickname = edit_nickname.clone();
        let teams = teams.clone();
        Callback::from(move |_| {
            let id = editing_id.clone();
            let name = edit_name.clone();
            let nickname = edit_nickname.clone();
            let teams = teams.clone();
            let editing_id = editing_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(id) = *id {
                    let payload = json!({ "id": id, "name": (*name).clone() , "nickname": (*nickname).clone() });
                    let url = "/api/teams";
                    if let Ok(req) = Request::put(&url)
                        .header("Content-Type", "application/json")
                        .body(payload.to_string()) {
                        if let Ok(resp) = req.send().await {
                            if let Ok(updated) = resp.json::<Team>().await {
                                let new_list: Vec<Team> = (*teams)
                                    .iter()
                                    .map(|t| if t.id.is_some_and(|x| x == id) { updated.clone() } else { t.clone() })
                                    .collect();
                                teams.set(new_list);
                                editing_id.set(None);
                            }
                        }
                    }
                }
            });
        })
    };

    let delete_team = {
        let teams = teams.clone();
        let set_error_msg = props.set_error_msg.clone();
        Callback::from(move |id: i32| {
            let teams = teams.clone();
            let set_error_msg = set_error_msg.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/teams/{}", id);
                match Request::delete(&url).send().await {
                    Ok(resp) => {
                        if resp.ok() {
                            set_error_msg.emit(None);
                            let new_list: Vec<Team> = (*teams).clone().into_iter().filter(|t| t.id.is_some_and(|x| x != id)).collect();
                            teams.set(new_list);
                        } else {
                            let status = resp.status();
                            match resp.text().await {
                                Ok(text) => {
                                    set_error_msg.emit(Some(format!("Delete failed ({}): {}", status, text)));
                                }
                                Err(e) => {
                                    set_error_msg.emit(Some(format!("Delete failed ({}): {}", status, e)));
                                }
                            }
                        }
                    },
                    Err(e) => {
                        println!("I got an error");
                        set_error_msg.emit(Some(format!("Error: {}", e)));
                    }
                }
            });
        })
    };


    // Delete team
html! {
        <div class="content">
            <h2>{ "Teams" }</h2>
            <div class="scrollable-table" style="border-right: 1px solid #ccc;">
            <table>
                <thead>
                    <tr>
                        <th>{ "Name" }</th>
                        <th>{ "Nick Name" }</th>
                        <th>{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    <tr>
                        <td>
                            <input
                                value={(*name_input).clone()}
                                oninput={{
                                    let name_input = name_input.clone();
                                    Callback::from(move |e: InputEvent| {
                                        name_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    })
                                }}
                            />
                        </td>
                        <td>
                            <input
                                value={(*nickname_input).clone()}
                                oninput={{
                                    let nickname_input = nickname_input.clone();
                                    Callback::from(move |e: InputEvent| {
                                        nickname_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    })
                                }}
                            />
                        </td>
                        <td class="actions">
                            <IconButton onclick={add_team}>
                                { add_icon() }
                            </IconButton>
                        </td>
                    </tr>
                    { for (*teams).iter().map(|team| {
                        let is_editing = team.id == *editing_id;
                        if is_editing {
                            html! {
                                <tr key={team.id.unwrap_or(-1)}>
                                    <td>
                                        <input
                                            value={(*edit_name).clone()}
                                            oninput={{
                                                let edit_name = edit_name.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    edit_name.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                                })
                                            }}
                                        />
                                    </td>
                                    <td>
                                        <input
                                            value={(*edit_nickname).clone()}
                                            oninput={{
                                                let edit_nickname = edit_nickname.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    edit_nickname.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                                })
                                            }}
                                        />
                                    </td>
                                    <td class="actions">
                                        <div class="button-row">
                                            <IconButton onclick={save_edit.clone()}>
                                                { save_icon() }
                                            </IconButton>
                                            <IconButton onclick={cancel_edit.clone()}>
                                                { cancel_icon() }
                                            </IconButton>
                                        </div>
                                    </td>
                                </tr>
                            }
                        } else {
                            let delete = {
                                let delete_team = delete_team.clone();
                                if let Some(id) = team.id{
                                    Callback::from(move |_| delete_team.emit(id))
                                } else {
                                    warn!("Tipper ID is None, cannot delete");
                                    Callback::from(|_| ())
                                }
                            };
                            let start_edit = {
                                let start_edit = start_edit.clone();
                                let team = team.clone();
                                Callback::from(move |_| start_edit.emit(team.clone()))
                            };
                            let disabled = !(team.can_delete.unwrap_or(false));
                            html! {
                                <tr key={team.id.unwrap_or(-1)}>
                                    <td>{ &team.name }</td>
                                    <td>{ &team.nickname }</td>
                                    <td class="actions">
                                        <div class="button-row">
                                            <IconButton onclick={start_edit}>
                                                { edit_icon() }
                                            </IconButton>
                                            <IconButton onclick={delete} disabled={disabled}>
                                                { delete_icon() }
                                            </IconButton>
                                        </div>
                                    </td>
                                </tr>
                            }
                        }
                    })}
                </tbody>
            </table>
            </div>
        </div>
    }
}
