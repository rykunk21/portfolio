use yew::prelude::*;

mod portfolio;

// pub struct Model {
//     value: i32,
// }

#[function_component(App)]
pub fn app() -> Html {
    // let state = use_state(|| Model { value: 0 });

    // let onclick = {
    //     let state = state.clone();

    //     Callback::from(move |_| {
    //         state.set(Model {
    //             value: state.value + 1,
    //         })
    //     })
    // };

    html! {
        <div>
            <h1>{"This is my portfolio"}</h1>
            <portfolio::Portfolio/>
        </div>
    }
}
