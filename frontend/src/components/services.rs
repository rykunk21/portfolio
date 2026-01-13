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
    let flipped = use_state(|| false);

    let toggle_flip = {
        let flipped = flipped.clone();
        Callback::from(move |_| flipped.set(!*flipped))
    };

    html! {
        <section class={classes!("services", "py-12", "px-4") }>
            <div class={classes!("max-w-4xl", "mx-auto") }>
                <h2 class={classes!("text-2xl", "font-semibold") }>{ "Services" }</h2>

                // Services grid
                <div class={classes!("mt-6", "grid", "gap-4", "sm:grid-cols-2") }>
                    { for services.into_iter().map(|s| html! {
                        <div class={classes!("p-4","border","rounded") }>{ s }</div>
                    }) }
                </div>

                // Toggle switch
                <div class={classes!("mt-8", "flex", "items-center", "gap-2") }>
                    <label>
                        <input type="checkbox" onchange={toggle_flip.clone()} />
                        { " Flip card" }
                    </label>
                </div>

                // Flip card
                <div class={classes!("mt-6", "w-64", "h-32", "perspective")}>
                    <div class={classes!("relative", "w-full", "h-full", "transition-transform", "duration-500", if *flipped {"rotate-y-180"} else {""})}>
                        // Front
                        <div class={classes!("absolute", "w-full", "h-full", "backface-hidden", "bg-blue-500", "flex", "items-center", "justify-center", "text-white", "rounded")}>
                            { "Front" }
                        </div>
                        // Back
                        <div class={classes!("absolute", "w-full", "h-full", "backface-hidden", "bg-green-500", "flex", "items-center", "justify-center", "text-white", "rounded", "rotate-y-180")}>
                            { "Back" }
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
