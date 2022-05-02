use gloo_storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yew::{function_component, html, use_state};

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TodoEntry {
    message: String,
    active: bool,
}

impl TodoEntry {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            active: true,
        }
    }
}

#[function_component(App)]
fn app() -> Html {
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
                <p class={"name"} style={"padding-left: 2.5rem;"}>{ &todo.message }</p>
            </div>
        }
    });
    let msg: String = LocalStorage::get("message").unwrap();
    html! {
       <>
       <div onclick = { add_todo }>
           { msg }
       </div>
       { for todos }
       </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
