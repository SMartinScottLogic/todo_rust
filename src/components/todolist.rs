use crate::components::closebutton::CloseButton;
use crate::components::todoitem::TodoItem;
use gloo_storage::{LocalStorage, Storage};
use todo_rust::TodoEntry;
use yew::{function_component, html, use_state, Callback};

#[function_component(TodoList)]
pub fn todolist() -> Html {
    let todos = use_state(|| LocalStorage::get("todo").unwrap_or_else(|_| Vec::<TodoEntry>::new()));
    let add_todo = {
        let todos = todos.clone();
        Callback::from(move |s: String| {
            let mut inner = (*todos).to_vec();
            log::debug!("Add todo: {s}");
            inner.push(TodoEntry::new(&s));
            LocalStorage::set("todo", inner.clone()).unwrap();
            todos.set(inner);
        })
    };

    let todo_display = todos.iter().enumerate().map(|(id, todo)| {
        let delete_todo = {
            let todos = todos.clone();
            Callback::from(move |_| {
                let mut inner = (*todos).to_vec();
                let removed = inner.remove(id);
                log::debug!("Removed todo: {removed:?}");
                LocalStorage::set("todo", inner.clone()).unwrap();
                todos.set(inner);
            })
        };
        html! {
            <div class={"todo container"} style={if id%2==0 {"background: #bbb;"}else{"background: #ccc;"}}>
                <span><TodoItem message = { todo.message.clone() } /><CloseButton onclick={delete_todo}/></span>
            </div>
        }
    });
    html! {
        <>
            <crate::components::editor::Editor update={add_todo} />
            { for todo_display }
        </>
    }
}
