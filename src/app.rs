use std::ops::Deref;

use crate::components::atoms::main_title::{Color, MainTitle};
use crate::components::atoms::struct_hello::StructHello;
use crate::components::molecules::custom_form::{CustomForm, Data};
use crate::router::{switch, Route};
use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::ContextProvider;
use yew_router::prelude::*;
#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}
#[derive(Debug, PartialEq, Clone, Default)]
pub struct User {
    pub username: String,
    pub favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let user_state = use_state(User::default);
    let main_title_load = Callback::from(|message: String| log!(message));
    let first_load = use_state(|| {
        log!("Renew use state");
        true
    });

    use_effect(move || {
        log!("Running after rendered.");
        if *first_load {
            first_load.set(false);
            log!("First load set false");
        }
        || {}
    });

    log!("Rendering...");

    let custom_form_submit = {
        let user_state = user_state.clone();
        Callback::from(move |data: Data| {
            let mut user = user_state.deref().clone();
            user.username = data.username;
            user.favorite_language = data.favorite_language;
            user_state.set(user);
        })
    };
    html! {
        <>
        <div><StructHello /></div>
        <ContextProvider<User> context={user_state.deref().clone()}>
            <MainTitle title="Hi there!!!!!!!" color={Color::Normal} on_load={main_title_load}/>
            <CustomForm onsubmit={custom_form_submit}/>
            <BrowserRouter>
                <Switch<Route> render={switch} />
            </BrowserRouter>
        </ContextProvider<User>>
        </>
    }
}

/* #[function_component(App)]
pub fn app() -> Html {
    let name = "TsaoLun";
    let name_obj = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };
    log!("Hello! My name is", name);
    log!(serde_json::to_string_pretty(&name_obj).unwrap());
    let class = "my_title";
    // let message = Some("I am a message");
    let message: Option<&str> = None;

    let tasks = vec!["record video", "grocery shopping", "pet Xilbe"];

    html! {
        <>
            <h1 class={class}>{"Hello World!!!"}</h1>
            if class == "my_titles" {
                <p>{"Hi there."}</p>
            } else {
                <p>{"I'm not a title."}</p>
            }

            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"no messages here."}</p>
            }

            <ul>
                {list_to_html(tasks)}
            </ul>
        </>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|task| html! {<li>{task}</li>}).collect()
} */
