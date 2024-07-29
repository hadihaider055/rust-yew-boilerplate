use yew::prelude::*;

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        <div class="bg-slate-800 flex items-center justify-center flex-col h-screen w-screen gap-10 text-white">
          <h1 class="text-7xl">{"404"}</h1>
          <h3 class="text-6xl">{"Oops, screen not found!"}</h3>
        </div>
    }
}
