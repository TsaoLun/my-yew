use gloo::console::log;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use stylist::{yew::styled_component, style};

mod components;
use components::atoms::main_title::{MainTitle, Color};
#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let main_title_load = Callback::from(|message: String| log!(message));
    html!{
        <div>
            <MainTitle title="Hi there!!!!!!!" color={Color::Normal} on_load={main_title_load}/>
            <p>{"more text"}</p>
        </div>
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
} */

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter().map(|task| html! {<li>{task}</li>}).collect()
}
