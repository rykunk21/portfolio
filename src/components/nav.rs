use yew::prelude::*;

#[function_component(Nav)]
pub fn nav() -> Html{
    
    let show = false;

    html! {{ 
        if show {
            html! { 
                <ul>
                    <li> {"1"} </li>
                </ul> 
            }
        } else {
            html! {}
        }
    }}
}