use std::{thread::LocalKey, ops::Deref};

use gloo_storage::{LocalStorage, Storage};
use serde::{Serialize, Deserialize};
use yew::{Callback, function_component, html, use_state};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoEntry {
    message: String,
    active: bool,
}

impl TodoEntry {
    pub fn new(message: &str) -> Self {
        Self { message: message.to_owned(), active: true }
    }
}

#[function_component(HelloWorld)]
fn hello_world() -> Html {
    let todo = use_state(|| {
        LocalStorage::get("todo").unwrap_or_else(|_| Vec::<TodoEntry>::new())
    });
    let add_todo = {
        let todo = todo.clone();
        move |_| {
            let mut inner = (*todo).to_vec();
            inner.push(TodoEntry::new(&format!("t{}", (*todo).len())));
            LocalStorage::set("todo", inner.clone()).unwrap();
            todo.set(inner);
        }
    };

    let stages = todo.iter().enumerate().map(|(id, stage)| {
        html! {
            <div style={if id%2==0 {"background: #bbb;"}else{"background: #ccc;"}}>
                <p class={"name"} style={"padding-left: 2.5rem;"}>{ &stage.message }</p>
            </div>
        }
    });
    let msg: String = LocalStorage::get("message").unwrap();
    html! {
        <>
        <div onclick = { add_todo }>
            { msg }
        </div>
        { for stages }
        </>
     }
}


fn main() {
    yew::start_app::<HelloWorld>();
}
