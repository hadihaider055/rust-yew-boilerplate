use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub children: Children,
    pub onclick: Callback<MouseEvent>,
    pub class: String,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let classes = format!("{} bg-blue-400 p-4 rounded-md", props.class);

    html! {
        <button class={classes} onclick={onclick}>
            { for props.children.iter() }
        </button>
    }
}
