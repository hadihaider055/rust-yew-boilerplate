use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub children: Children,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let onclick = props.onclick.clone();
    let classes = format!(
        "{} bg-blue-400 p-4 rounded-md disabled:cursor-not-allowed disabled:opacity-80",
        props.class
    );
    let disabled = props.disabled.then(|| false);

    html! {
        <button class={classes} {onclick} disabled={disabled.is_some()}>
            { for props.children.iter() }
        </button>
    }
}
