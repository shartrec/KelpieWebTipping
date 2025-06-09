// frontend/src/components/tipper_list.rs
use yew::prelude::*;
use gloo_net::http::Request;
use serde_json::json;

use crate::models::tipper::Tipper;

#[function_component(TipperList)]
pub fn tipper_list() -> Html {
    let tippers = use_state(|| vec![]);
    let name_input = use_state(|| String::new());
    let email_input = use_state(|| String::new());

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
    let onsubmit = {
        let name_input = name_input.clone();
        let email_input = email_input.clone();
        let tippers = tippers.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let name = name_input.clone();
            let email = email_input.clone();
            let tippers = tippers.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let payload = json!({
                    "name": (*name).clone(),
                    "email": (*email).clone(),
                });

                if let Ok(resp) = Request::post("/api/tippers")
                    .header("Content-Type", "application/json")
                    .body(payload.to_string())
                    .send()
                    .await
                {
                    if let Ok(new_tipper) = resp.json::<Tipper>().await {
                        let mut new_list = (*tippers).clone();
                        new_list.push(new_tipper);
                        tippers.set(new_list);
                        name.set(String::new());
                        email.set(String::new());
                    }
                }
            });
        })
    };

    // Handle deletion
    let delete_tipper = {
        let tippers = tippers.clone();
        Callback::from(move |id: i32| {
            let tippers = tippers.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let url = format!("/api/tippers/{}", id);
                let resp = Request::delete(&url).send().await;
                if resp.is_ok() {
                    let updated: Vec<Tipper> = (*tippers).clone().into_iter().filter(|t| t.id != id).collect();
                    tippers.set(updated);
                }
            });
        })
    };

    html! {
        <div>
            <h2>{ "Tippers" }</h2>
            <form onsubmit={onsubmit}>
                <input
                    type="text"
                    placeholder="Name"
                    value={(*name_input).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        name_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                    })}
                />
                <input
                    type="email"
                    placeholder="Email"
                    value={(*email_input).clone()}
                    oninput={Callback::from(move |e: InputEvent| {
                        email_input.set(e.target_unchecked_into::<web_sys::HtmlInputElement>().value());
                    })}
                />
                <button type="submit">{ "Add Tipper" }</button>
            </form>

            <ul>
                { for (*tippers).iter().map(|tipper| {
                    let delete = {
                        let delete_tipper = delete_tipper.clone();
                        let id = tipper.id;
                        Callback::from(move |_| delete_tipper.emit(id))
                    };

                    html! {
                        <li key={tipper.id}>
                            { format!("{} ({})", tipper.name, tipper.email) }
                            <button onclick={delete}>{ "Delete" }</button>
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
