use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::{function_component, html, use_node_ref, Callback, Properties};

#[derive(Properties, PartialEq)]
pub struct EditorProps {
    pub update: Callback<String>,
}

#[function_component(Editor)]
pub fn editor(EditorProps { update }: &EditorProps) -> Html {
    let name_ref = use_node_ref();

    let onclick = {
        let name_ref = name_ref.clone();
        let update = update.clone();
        move |_| {
            if let Some(input) = name_ref.cast::<HtmlInputElement>() {
                let name = input.value();
                update.emit(name)
            }
        }
    };
    let onkeyup = {
        let name_ref = name_ref.clone();
        let update = update.clone();
        move |e: KeyboardEvent| {
            if e.key_code() == 13 {
                if let Some(input) = name_ref.cast::<HtmlInputElement>() {
                    let name = input.value();
                    update.emit(name);
                    input.set_value("");
                }
            }
        }
    };
    html! {
        <div style="display: flex; flex-flow: row nowrap;">
            <input type="text" ref={name_ref} onkeyup={onkeyup}
                name="editor" data-test-selector="nav-search-input"
                autocapitalize="none" spellcheck="false" autocomplete="off" class={"editor"} />
            <button onclick={onclick} style="flex: 0; font-size: 1.2em; ">{ "Ok" }</button>
        </div>
    }
}
