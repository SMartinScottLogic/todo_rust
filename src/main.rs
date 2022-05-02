use bounce::BounceRoot;
use yew::{function_component, html};

mod components;
use crate::components::todolist::TodoList;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[function_component(App)]
fn app() -> Html {
    html! {
       <BounceRoot>
           <TodoList />
       </BounceRoot>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
