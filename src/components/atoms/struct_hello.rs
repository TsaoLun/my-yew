use stylist::{style, Style};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
}

pub struct StructHello {
    pub message: String,
    pub stylesheet: Style,
}

impl StructHello {
    fn style() -> Style {
        style!(
            r#"
            color: green;
        "#
        )
        .unwrap()
    }
}

impl Component for StructHello {
    type Message = ();
    type Properties = Props;
    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            message: "Hello World from a Struct!".to_owned(),
            stylesheet: Self::style(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <h1 class={self.stylesheet.clone()}>{&ctx.props().message}</h1>
        }
    }
}
