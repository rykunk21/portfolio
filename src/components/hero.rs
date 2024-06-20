
use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {

    html! {
        <div class="hero">
            <div class="content">
                <img src="./img/cube.svg"/>
            </div>
            <p> {"this is some content"} </p>
        </div>
    }
}