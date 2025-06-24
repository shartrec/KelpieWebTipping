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
use log::debug;
use yew::prelude::*;
use kelpie_models::team::Team;

/// Props for IconButton
#[derive(Properties, PartialEq)]
pub struct IconButtonProps {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    pub onclick: Callback<MouseEvent>,
    pub children: Children,
}

#[function_component(IconButton)]
pub fn icon_button(props: &IconButtonProps) -> Html {
    let IconButtonProps { label, disabled, onclick, children } = props;

    html! {
        <button disabled={*disabled} class="icon-button" {onclick}>
            <div class="icon">{ for children.iter() }</div>
        if let Some(lbl) = label {
            <div class="label">{ lbl }</div>
        }
        </button>
    }
}

#[derive(Properties, PartialEq)]
pub struct TipSelectorProps {
    pub home_team: Team,
    pub away_team: Team,
    pub selected_team_id: Option<i32>,
    pub on_change: Callback<i32>,
    pub name: String, // Add a name prop for radio group
}

#[function_component(TipSelector)]
pub fn tip_selector(props: &TipSelectorProps) -> Html {
    let on_change = props.on_change.clone();

    let home_id = props.home_team.id.unwrap();
    let away_id = props.away_team.id.unwrap();

    let handle_select = move |team_id: i32| {
        debug!("Emitting");
        on_change.emit(team_id);
    };

    html! {
        <div class="button-group">
            <input
                type="radio"
                id={format!("home-{}-{}", home_id, props.name)}
                name={props.name.clone()}
                checked={props.selected_team_id == Some(home_id)}
                oninput={Callback::from({
                    let handle_select = handle_select.clone();
                    move |_| handle_select(home_id)
                })}
            />
            <label
                for={format!("home-{}-{}", home_id, props.name)}
            >
                { &props.home_team.nickname }
            </label>

            <input
                type="radio"
                id={format!("away-{}-{}", away_id, props.name)}
                name={props.name.clone()}
                checked={props.selected_team_id == Some(away_id)}
                oninput={Callback::from({
                    let handle_select = handle_select.clone();
                    move |_| handle_select(away_id)
                })}
            />
            <label
                for={format!("away-{}-{}", away_id, props.name)}
            >
                { &props.away_team.nickname }
            </label>
        </div>
    }
}