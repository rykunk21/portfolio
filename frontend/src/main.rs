use yew::prelude::*;
use yew::use_effect_with;

mod components;
mod theme;

use components::{
    about::About, audience::Audience, background::ImmersiveBackground, contact::Contact, header::Header, hero::Hero,
    process::Process, scroll_snap, services::Services,
};

#[function_component(App)]
fn app() -> Html {
    theme::apply_palette();
    
    // Initialize scroll snap on mobile
    use_effect_with((), |_| {
        scroll_snap::init_scroll_snap();
        || {}
    });

    html! {
        <>
            <ImmersiveBackground />
            <section id="hero"><Header /><Hero /></section>
            <section id="services"><Services /></section>
            <section id="process"><Process /></section>
            <section id="audience"><Audience /></section>
            <section id="about"><About /></section>
            <section id="contact"><Contact /></section>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
