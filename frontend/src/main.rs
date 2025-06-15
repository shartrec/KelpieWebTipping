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

use yew::prelude::*;
use gloo_net::http::Request;

// frontend/src/main.rs
mod models;
mod components;

use yew::prelude::*;
use components::tipper_list::TipperList;
use crate::components::icon_button::IconButton;
use crate::components::icons::{teams_icon, tippers_icon};
use crate::components::team_list::TeamList;

#[derive(PartialEq, Clone)]
enum View {
    Teams,
    Tippers,
}

#[function_component(App)]
fn app() -> Html {
    let view = use_state(|| View::Teams);

    let set_teams = {
        let view = view.clone();
        Callback::from(move |_| view.set(View::Teams))
    };
    let set_tippers = {
        let view = view.clone();
        Callback::from(move |_| view.set(View::Tippers))
    };

    html! {
        <div style="display: flex;">
            <nav style="width: 80px; background: #f0f0f0; padding: 16px 0;">
                <IconButton label="Teams" onclick={set_teams}>
                    { teams_icon() }
                </IconButton>
                <IconButton label="Tippers" onclick={set_tippers}>
                    { tippers_icon() }
                </IconButton>
            </nav>
            <main style="flex: 1; padding: 24px;">
                <h1 style="display: flex; align-items: center;">
                    <img src="images/kelpiedog_120x120_transparent.png" alt="Kelpie Logo" style="margin-right: 12px;"/>
                    { "Football Tipping" }
                </h1>
                {
                    match *view {
                        View::Teams => html! { <TeamList /> },
                        View::Tippers => html! { <TipperList /> },
                    }
                }
            </main>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

