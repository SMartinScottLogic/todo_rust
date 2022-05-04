use yew::{function_component, html, Callback, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct CloseButtonProps {
    pub onclick: Callback<yew::MouseEvent>,
}

#[function_component(CloseButton)]
pub fn todoitem(CloseButtonProps { onclick }: &CloseButtonProps) -> Html {
    html! {
        <span style="cursor: pointer; position: absolute; right: 0px;" onclick={onclick}><crate::components::icon::Cross width=32 height=32 /></span>
    }
}
