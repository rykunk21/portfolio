use yew::prelude::*;

mod portfolio;
mod hero;
mod nav;
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div>
            <nav::Nav/>
            <hero::Hero/>
            <portfolio::Portfolio/>
        </div>
    }
}
