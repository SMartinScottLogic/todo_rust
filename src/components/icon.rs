use yew::{function_component, html, Properties};

#[derive(Clone, Properties, PartialEq)]
pub struct IconProps {
    pub width: usize,
    pub height: usize,
}

#[function_component(Alarm)]
pub fn alarm(IconProps { width, height }: &IconProps) -> Html {
    html! {
        <svg style="color: darkblue; padding-right: 0.5rem" xmlns={"http://www.w3.org/2000/svg"} width={ width.to_string() } height={height.to_string()} fill={"currentColor"} class={"bi bi-alarm"} viewBox={"0 0 16 16"}>
            <path d={"M8.5 5.5a.5.5 0 0 0-1 0v3.362l-1.429 2.38a.5.5 0 1 0 .858.515l1.5-2.5A.5.5 0 0 0 8.5 9V5.5z"}/>
            <path d={"M6.5 0a.5.5 0 0 0 0 1H7v1.07a7.001 7.001 0 0 0-3.273 12.474l-.602.602a.5.5 0 0 0 .707.708l.746-.746A6.97 6.97 0 0 0 8 16a6.97 6.97 0 0 0 3.422-.892l.746.746a.5.5 0 0 0 .707-.708l-.601-.602A7.001 7.001 0 0 0 9 2.07V1h.5a.5.5 0 0 0 0-1h-3zm1.038 3.018a6.093 6.093 0 0 1 .924 0 6 6 0 1 1-.924 0zM0 3.5c0 .753.333 1.429.86 1.887A8.035 8.035 0 0 1 4.387 1.86 2.5 2.5 0 0 0 0 3.5zM13.5 1c-.753 0-1.429.333-1.887.86a8.035 8.035 0 0 1 3.527 3.527A2.5 2.5 0 0 0 13.5 1z"}/>
        </svg>
    }
}

#[function_component(Plus)]
pub fn plus(IconProps { width, height }: &IconProps) -> Html {
    html! {
        <svg style="padding-right: 0.5rem" xmlns="http://www.w3.org/2000/svg" width={ width.to_string()} height={ height.to_string() } fill="currentColor" class="bi bi-plus-circle" viewBox="0 0 16 16">
          <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
          <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
        </svg>
    }
}

#[function_component(Cross)]
pub fn cross(IconProps { width, height }: &IconProps) -> Html {
    html! {
        <svg style="padding-right: 0.5rem" xmlns="http://www.w3.org/2000/svg" width={ width.to_string()} height={ height.to_string() } fill="currentColor" class="bi bi-cross" viewBox="0 0 16 16">
            <path d="M4.646 4.646a.5.5 0 0 1 .708 0L8 7.293l2.646-2.647a.5.5 0 0 1 .708.708L8.707 8l2.647 2.646a.5.5 0 0 1-.708.708L8 8.707l-2.646 2.647a.5.5 0 0 1-.708-.708L7.293 8 4.646 5.354a.5.5 0 0 1 0-.708z"/>
        </svg>
    }
}
