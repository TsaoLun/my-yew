use std::ops::Deref;

use gloo::console::log;
use yew::prelude::*;

use crate::app::User;
use crate::components::atoms::custom_button::CustomButton;
use crate::components::atoms::text_input::TextInput;

#[derive(Default, Clone)]
pub struct Data {
    pub username: String,
    pub favorite_language: String,
    // pub count: u32,
}
#[derive(Properties, PartialEq)]
pub struct Props {
    pub onsubmit: Callback<Data>,
}

#[function_component(CustomForm)]
pub fn custom_form(props: &Props) -> Html {
    let state = use_state(|| Data::default());
    let user_context = use_context::<User>();
    let cloned_state = state.clone();
    let username_changed = Callback::from(move |username: String| {
        log!("Received", username.to_owned());
        // let mut data = (*cloned_state).clone();
        // data.username = username;
        // cloned_state.set(data);
        cloned_state.set(Data {
            username,
            ..cloned_state.deref().clone()
        })
    });
    let cloned_state = state.clone();
    let language_changed = Callback::from(move |language| {
        cloned_state.set(Data {
            favorite_language: language,
            ..cloned_state.deref().clone()
        })
    });
    // let button_clicked = Callback::from(move |_| {
    //     let mut data = cloned_state.deref().clone();
    //     data.count += 1;
    //     cloned_state.set(data);
    // });
    let form_onsubmit = props.onsubmit.clone();
    let cloned_state = state.clone();
    let onsubmit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let data = cloned_state.deref().clone();
        form_onsubmit.emit(data);
    });
    html! {
        <form onsubmit={onsubmit}>
            <TextInput name="username" handle_onchange={username_changed} />
            <TextInput name="favorite_language" handle_onchange={language_changed} />
            <CustomButton label="Submit"/>
            <p>{"Username: "}{user_context.clone().unwrap_or_default().username}</p>
            <p>{"Favorite Language:"}{user_context.unwrap_or_default().favorite_language}</p>
        </form>
    }
}
