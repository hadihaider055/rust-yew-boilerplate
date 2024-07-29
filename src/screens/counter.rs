use crate::components::button::Button;
use yew::prelude::*;

#[function_component(Counter)]
pub fn counter() -> Html {
    let counter = use_state(|| 0);
    let handle_increase = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };

    let handle_decrease = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };

    html! {
        <div class="bg-slate-800 flex items-center justify-center h-screen w-screen gap-10">
            <Button onclick={handle_increase} class="bg-blue-400 p-4 rounded-md">{ "+1" }</Button>
            <p class="text-white text-7xl">{ *counter }</p>
            <Button onclick={handle_decrease} class="bg-blue-400 p-4 rounded-md">{ "-1" }</Button>
        </div>
    }
}
