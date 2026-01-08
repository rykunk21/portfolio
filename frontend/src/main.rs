use yew::prelude::*;

mod components;

use components::{
    about::About, audience::Audience, contact::Contact, header::Header, hero::Hero,
    process::Process,
};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <Hero />
            <Process />
            <Audience />
            <About />
            <Contact />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
