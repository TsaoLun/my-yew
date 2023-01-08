use gloo::console::log;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::router::Route;

#[function_component(Hello)]
pub fn hello() -> Html {
    let history = use_navigator().unwrap();
    let onclick = Callback::from(move |_| {
        history.push(&Route::Home);
    });
    html! {
        <div>
            <h1>{"Hello"}</h1>
            <button onclick={onclick}>{"Go Home"}</button>
        </div>
    }
}
