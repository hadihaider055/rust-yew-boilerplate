use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod containers;
mod screens;

use crate::screens::counter::Counter;
use crate::screens::not_found::NotFound;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Counter,
    #[at("/*path")]
    NotFound,
}

#[function_component]
fn App() -> Html {
    fn switch(routes: Route) -> Html {
        match routes {
            Route::Counter => html! { <Counter /> },
            Route::NotFound => html! { <NotFound /> },
        }
    }

    html! {
        <BrowserRouter>
             <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
