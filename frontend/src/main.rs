use yew::prelude::*;
use gloo_net::http::Request;

// frontend/src/main.rs
mod models;
mod components;

use yew::prelude::*;
use components::tipper_list::TipperList;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{ "Football Tipping" }</h1>
            <TipperList />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

