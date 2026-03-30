use yew::prelude::*;

mod components;
mod theme;

use components::{
    about::About, audience::Audience, background::ImmersiveBackground, contact::Contact, header::Header, hero::Hero,
    loading::LoadingScreen, process::Process, services::Services,
};

#[function_component(App)]
fn app() -> Html {
    theme::apply_palette();

    html! {
        <LoadingScreen>
            <ImmersiveBackground />
            <Header />
            <Hero />
            <Services />
            <Process />
            <Audience />
            <About />
            <Contact />
        </LoadingScreen>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
