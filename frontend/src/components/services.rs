use yew::prelude::*;

/// Services section and list
#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub services: Vec<String>,
}

#[function_component(Services)]
pub fn services(props: &Props) -> Html {
    let services = props.services.clone();

    html! {
        <section class={classes!("services", "py-12", "px-4") }>
            <div class={classes!("max-w-4xl", "mx-auto") }>
                <h2 class={classes!("text-2xl", "font-semibold") }>{ "Services" }</h2>
                <div class={classes!("mt-6", "grid", "gap-4", "sm:grid-cols-2") }>
                    { for services.into_iter().map(|s| html! { <div class={classes!("p-4","border","rounded") }>{ s }</div> }) }
                </div>
            </div>
        </section>
    }
}
