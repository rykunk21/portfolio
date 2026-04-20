use yew::prelude::*;
use crate::components::scroll_helper::{ScrollHelperDots, ScrollProgressBar};

#[function_component(ScrollHelperDemo)]
pub fn scroll_helper_demo() -> Html {
    let active_section = use_state(|| 2_usize);
    
    html! {
        <div class="min-h-screen bg-neutral-900 text-white">
            // Progress bar at top
            <ScrollProgressBar />
            
            // Dots on right
            <ScrollHelperDots section_count={5} active_section={*active_section} />
            
            // Demo sections
            <div class="space-y-0">
                { for (0..5).map(|i| {
                    let bg_color = match i % 5 {
                        0 => "bg-gradient-to-br from-neutral-800 to-neutral-900",
                        1 => "bg-gradient-to-br from-slate-800 to-slate-900",
                        2 => "bg-gradient-to-br from-zinc-800 to-zinc-900",
                        3 => "bg-gradient-to-br from-stone-800 to-stone-900",
                        _ => "bg-gradient-to-br from-gray-800 to-gray-900",
                    };
                    html! {
                        <section 
                            id={format!("section-{}", i)}
                            class={classes!("min-h-screen", "flex", "items-center", "justify-center", bg_color)}
                        >
                            <button
                                onclick={{
                                    let active = active_section.clone();
                                    Callback::from(move |_| active.set(i))
                                }}
                                class="text-6xl font-light hover:scale-110 transition-transform cursor-pointer"
                                style={if *active_section == i { 
                                    "color: #fa8805; text-shadow: 0 0 20px rgba(250,136,5,0.5)" 
                                } else { 
                                    "color: rgba(255,255,255,0.3)" 
                                }}
                            >
                                { format!("Section {}", i + 1) }
                            </button>
                        </section>
                    }
                })}
            </div>
        </div>
    }
}
