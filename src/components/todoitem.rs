use yew::{function_component, html, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct TodoItemProps {
    pub message: String,
}

#[function_component(TodoItem)]
pub fn todoitem(TodoItemProps { message }: &TodoItemProps) -> Html {
    html! {
        <>
            <span class={"name"} style={"padding-left: 2.5rem;"}>{ message }</span>
        </>
    }
}
