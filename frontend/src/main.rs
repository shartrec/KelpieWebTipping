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

// frontend/src/main.rs
mod models;
mod components;

use crate::components::edit_round::EditRound;
use crate::components::icon_button::IconButton;
use crate::components::icons::{rounds_icon, teams_icon, tippers_icon};
use crate::components::round_list::RoundList;
use crate::components::team_list::TeamList;
use components::tipper_list::TipperList;

#[derive(PartialEq, Clone)]
enum View {
    Teams,
    Tippers,
    Rounds,
    RoundEdit{round_id: Option<i32>},
}

#[derive(PartialEq, Clone)]
pub(crate) struct ViewContext {
    view: UseStateHandle<View>,
    error_msg: UseStateHandle<Option<String>>,
}
impl ViewContext {
    pub(crate) fn set_view(&self, view: View) {
        self.error_msg.set(None); // Clear error message when changing view
        self.view.set(view);
    }
}

#[function_component(App)]
fn app() -> Html {
    let view = use_state(|| View::Teams);
    let error_msg = use_state(|| None::<String>);

    let view_context = ViewContext {
        view: view.clone(),
        error_msg: error_msg.clone(),
    };

    let set_teams = {
        let view_context = view_context.clone();
        Callback::from(move |_| view_context.set_view(View::Teams))
    };
    let set_tippers = {
        let view_context = view_context.clone();
        Callback::from(move |_| view_context.set_view(View::Tippers))
    };
    let set_rounds = {
        let view_context = view_context.clone();
        Callback::from(move |_| view_context.set_view(View::Rounds))
    };

    let set_error_msg = {
        let error_msg = error_msg.clone();
        Callback::from(move |msg: Option<String>| error_msg.set(msg))
    };

    html! {
        <ContextProvider<ViewContext> context={view_context}>

            <div style="display: flex;">
                <nav style="width: 80px; background: #f0f0f0; padding: 16px 0;">
                    <IconButton label="Teams" onclick={set_teams}>
                        { teams_icon() }
                    </IconButton>
                    <IconButton label="Tippers" onclick={set_tippers}>
                        { tippers_icon() }
                    </IconButton>
                    <IconButton label="Rounds" onclick={set_rounds}>
                        { rounds_icon() }
                    </IconButton>
                </nav>
                <main style="flex: 1; padding: 24px;">
                    <h1 style="display: flex; align-items: center;">
                        <img src="images/kelpiedog_120x120_transparent.png" alt="Kelpie Logo" style="margin-right: 12px;"/>
                        <span>{ "Kelpie Footy Tipping" }<span style="font-size:1rem;"><br/>{"by Shartrec"}</span></span>

                    </h1>
                    {
                        if let Some(msg) = &*error_msg {
                            html! { <div style="color: red;">{ msg }</div> }
                        } else {
                            html! {}
                        }
                    }

                    {
                        match *view {
                            View::Teams => html! { <TeamList set_error_msg={set_error_msg.clone()}/> },
                            View::Tippers => html! { <TipperList /> },
                            View::Rounds => html! { <RoundList /> },
                            View::RoundEdit{round_id} => html! { <EditRound set_error_msg={set_error_msg.clone()} round_id={round_id}/> },
                        }
                    }
                </main>
            </div>
        </ContextProvider<ViewContext>>

    }
}

fn main() {
    console_log::init_with_level(log::Level::Debug).expect("error initializing logger");
    yew::Renderer::<App>::new().render();
}

