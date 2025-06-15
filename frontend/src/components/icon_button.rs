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
            <div class="label">{ label }</div>
        }
        </button>
    }
}