use crate::components::todoitem::TodoItem;
use gloo_storage::{LocalStorage, Storage};
use todo_rust::TodoEntry;
use yew::{function_component, html, use_state};

#[function_component(TodoList)]
pub fn todolist() -> Html {
    let todo = use_state(|| LocalStorage::get("todo").unwrap_or_else(|_| Vec::<TodoEntry>::new()));
    let add_todo = {
        let todo = todo.clone();
        move |_| {
            let mut inner = (*todo).to_vec();
            let new_todo = format!("t{}", inner.len());
            log::debug!("Add todo: {new_todo:?}");
            inner.push(TodoEntry::new(&new_todo));
            LocalStorage::set("todo", inner.clone()).unwrap();
            todo.set(inner);
        }
    };

    let todos = todo.iter().enumerate().map(|(id, todo)| {
        html! {
            <div style={if id%2==0 {"background: #bbb;"}else{"background: #ccc;"}}>
                <TodoItem message = { todo.message.clone() } />
            </div>
        }
    });
    let msg: String = LocalStorage::get("message").unwrap_or_else(|_| "Hello World".to_string());
    html! {
        <>
            <div onclick = { add_todo }>
                { msg }
            </div>
            { for todos }
        </>
    }
}
