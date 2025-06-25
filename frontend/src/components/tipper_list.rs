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
use kelpie_models::tipper::Tipper;
use log::warn;
use serde_json::json;
// frontend/src/components/tipper_list.rs
use yew::prelude::*;

#[function_component(TipperList)]
pub fn tipper_list() -> Html {
    let tippers = use_state(|| vec![]);
    let name_input = use_state(|| String::new());
    let email_input = use_state(|| String::new());

       // New state for editing
    let editing_id = use_state(|| None as Option<i32>);
    let edit_name = use_state(|| String::new());
    let edit_email = use_state(|| String::new());

    // Load tippers on mount
    {
        let tippers = tippers.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(resp) = Request::get("/api/tippers").send().await {
                    if let Ok(data) = resp.json::<Vec<Tipper>>().await {
                        tippers.set(data);
                    }
                }
            });
            || ()
        });
    }

    // Handle form submission
    let add_tipper = {
        let name_input = name_input.clone();
        let email_input = email_input.clone();
        let tippers = tippers.clone();

        Callback::from(move |_e: MouseEvent| {
            let name = name_input.clone();
            let email = email_input.clone();
            let tippers = tippers.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let payload = json!({
                    "name": (*name).clone(),
                    "email": (*email).clone(),
                });

                if let Ok(req) = Request::post("/api/tippers")
                    .header("Content-Type", "application/json")
                    .body(payload.to_string())
                {
                    if let Ok(resp) = req.send().await
                    {
                        if let Ok(new_tipper) = resp.json::<Tipper>().await {
                            let mut new_list = (*tippers).clone();
                            new_list.push(new_tipper);
                            tippers.set(new_list);
                            name.set(String::new());
                            email.set(String::new());
                        }
                    }
                }
            });
        })
    };

    // Handle deletion
    let start_edit = {
        let editing_id = editing_id.clone();
        let edit_name = edit_name.clone();
        let edit_email = edit_email.clone();
        Callback::from(move |tipper: Tipper| {
            editing_id.set(tipper.id);
            edit_name.set(tipper.name.clone());
            edit_email.set(tipper.email.clone());
        })
    };

    // Start editing

    // Cancel editing
    let cancel_edit = {
        let editing_id = editing_id.clone();
        Callback::from(move |_| editing_id.set(None))
    };

    // Save edit
    let save_edit = {
        let editing_id = editing_id.clone();
        let edit_name = edit_name.clone();
        let edit_email = edit_email.clone();
        let tippers = tippers.clone();
        Callback::from(move |_| {
            let id = editing_id.clone();
            let name = edit_name.clone();
            let email = edit_email.clone();
            let tippers = tippers.clone();
            let editing_id = editing_id.clone();
            wasm_bindgen_futures::spawn_local(async move {
                if let Some(id) = *id {
                    let payload = json!({"id": id, "name": (*name).clone(), "email": (*email).clone()});
                    let url = "/api/tippers";
                    if let Ok(req) = Request::put(&url)
                        .header("Content-Type", "application/json")
                        .body(payload.to_string()) {
                        if let Ok(resp) = req.send().await {
                            if let Ok(updated) = resp.json::<Tipper>().await {
                                let new_list: Vec<Tipper> = (*tippers)
                                    .iter()
                                    .map(|t| if t.id.is_some_and(|x| x == id) { updated.clone() } else { t.clone() })
                                    .collect();
                                tippers.set(new_list);
                                editing_id.set(None);
                            }
                        }
                    }
                }
            });
        })
    };

    let delete_tipper = {
        let tippers = tippers.clone();
        Callback::from(move |id: i32| {
            let tippers = tippers.clone();
            // Show confirm dialog before proceeding
            if web_sys::window()
                .and_then(|w| w.confirm_with_message("Are you sure you want to delete this tipper?\nThis will remove all tipping history for that user.").ok())
                .unwrap_or(false)
            {
                wasm_bindgen_futures::spawn_local(async move {
                    let url = format!("/api/tippers/{}", id);
                    let resp = Request::delete(&url).send().await;
                    if resp.is_ok() {
                        let updated: Vec<Tipper> = (*tippers).clone().into_iter().filter(|t| t.id.is_some_and(|x| x != id)).collect();
                        tippers.set(updated);
                    }
                });
            }
        })
    };

    html! {
        <div class="content">
            <h2>{ "Tippers" }</h2>
            <div class="scrollable-table" style="border-right: 1px solid #ccc;">
            <table class="scrollable-list">
                <thead>
                    <tr>
                        <th>{ "Name" }</th>
                        <th>{ "Email" }</th>
                        <th>{ "Actions" }</th>
                    </tr>
                </thead>
                <tbody>
                    // Add tipper form as the first row
                    <tr>
                            <td>
                                <input
                                    type="text"
                                    placeholder="Name"
                                    value={(*name_input).clone()}
                                    oninput={Callback::from(move |e: InputEvent| {
                                        name_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    })}
                                />
                            </td>
                            <td>
                                <input
                                    type="email"
                                    placeholder="Email"
                                    value={(*email_input).clone()}
                                    oninput={Callback::from(move |e: InputEvent| {
                                        email_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                                    })}
                                />
                            </td>
                            <td class="actions">
                            <IconButton onclick={add_tipper}>
                                { add_icon() }
                            </IconButton>
                            </td>
                    </tr>
                    // Existing tippers

                    { for (*tippers).iter().map(|tipper| {
                        let is_editing = tipper.id == *editing_id;
                        if is_editing {
                            html! {
                                <tr key={tipper.id.unwrap_or(-1)}>
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
                                            value={(*edit_email).clone()}
                                            oninput={{
                                                let edit_email = edit_email.clone();
                                                Callback::from(move |e: InputEvent| {
                                                    edit_email.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
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
                                let delete_tipper = delete_tipper.clone();
                                if let Some(id) = tipper.id{
                                    Callback::from(move |_| delete_tipper.emit(id))
                                } else {
                                    warn!("Tipper ID is None, cannot delete");
                                    Callback::from(|_| ())
                                }
                            };
                            let start_edit = {
                                let start_edit = start_edit.clone();
                                let tipper = tipper.clone();
                                Callback::from(move |_| start_edit.emit(tipper.clone()))
                            };
                            html! {
                                <tr key={tipper.id.unwrap_or(-1)}>
                                    <td>{ &tipper.name }</td>
                                    <td>{ &tipper.email }</td>
                                    <td class="actions">
                                        <div class="button-row">
                                            <IconButton onclick={start_edit}>
                                                { edit_icon() }
                                            </IconButton>
                                            <IconButton onclick={delete} disabled=false>
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
