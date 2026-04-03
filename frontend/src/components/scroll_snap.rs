use yew::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::ui_depth::{UIDepth, ParallaxSection, LayerGroup};

/// REQ-005: Scroll snap with UI parallax depth
/// Each viewport snaps to exact position, content appears at correct depth
#[function_component(ScrollSnap)]
pub fn scroll_snap() -> Html {
    let active_section = use_state(|| 0_usize);
    
    {
        let active_section = active_section.clone();
        use_effect_with((), move |_| {
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            
            let closure = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Ok(scroll_y) = w.scroll_y() {
                        let vh_js = w.inner_height().unwrap_or_else(|_| wasm_bindgen::JsValue::from_f64(800.0));
                        let vh = vh_js.as_f64().unwrap_or(800.0);
                        let idx = ((scroll_y + vh / 2.0) / vh) as usize;
                        active_section.set(idx);
                    }
                }
            }) as Box<dyn FnMut()>);
            
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }
    
    html! {
        <div class="relative"
             style="scroll-snap-type: y mandatory; overflow-y: scroll; height: 100vh;"
        >
            // Section 1: Introduction
            <div style="scroll-snap-align: start; scroll-snap-stop: always; min-height: 100vh;"
            >
                <ParallaxSection section_index={0} height_vh={100}>
                    <LayerGroup base_depth={0.7}>
                        // Title - shallow depth for "floating" feel
                        <UIDepth depth={0.85} shadow={0.4} z_offset={20}>
                            <h2 class="text-4xl md:text-6xl font-light mb-8 text-center"
                                style="color: var(--color-surface-50);"
                            >
                                { "Welcome to the Experience" }
                            </h2>
                        </UIDepth>
                        
                        // Subtitle - slightly deeper
                        <UIDepth depth={0.75} shadow={0.3} z_offset={15}>
                            <p class="text-xl text-center max-w-2xl mb-12"
                               style="color: var(--color-surface-400);"
                            >
                                { "Each viewport is carefully composed with depth" }
                            </p>
                        </UIDepth>
                        
                        // Arrow indicator
                        <UIDepth depth={0.9} shadow={0.2} z_offset={10}>
                            <div class="animate-bounce">
                                <svg class="w-8 h-8 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor"
                                     style="color: var(--color-highlight-500);"
                                >
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 14l-7 7m0 0l-7-7m7 7V3" />
                                </svg>
                            </div>
                        </UIDepth>
                    </LayerGroup>
                </ParallaxSection>
            </div>
            
            // Section 2: Showcase (image center)
            <div style="scroll-snap-align: start; scroll-snap-stop: always; min-height: 100vh;"
            >
                <ParallaxSection section_index={1} height_vh={100}>
                    <LayerGroup base_depth={0.5}>
                        // Image - deepest element in this section
                        <UIDepth depth={0.6} shadow={0.8} z_offset={5}>
                            <div class="w-full max-w-3xl mx-auto mb-8 rounded-xl overflow-hidden"
                                 style="transform: perspective(1000px) rotateY(-3deg);"
                            >
                                <div class="aspect-video bg-gradient-to-br from-slate-700 to-slate-900 flex items-center justify-center"
                                >
                                    <span class="text-surface-500 text-2xl">{ "Portfolio Image" }</span>
                                </div>
                            </div>
                        </UIDepth>
                        
                        // Caption - floats above image
                        <UIDepth depth={0.8} shadow={0.4} z_offset={15}>
                            <h3 class="text-3xl font-light text-center mb-4"
                                style="color: var(--color-surface-50);"
                            >
                                { "Featured Work" }
                            </h3>
                        </UIDepth>
                        
                        // Description - floats highest
                        <UIDepth depth={0.9} shadow={0.3} z_offset={18}>
                            <p class="text-lg text-center max-w-xl"
                               style="color: var(--color-surface-400);"
                            >
                                { "Each element positioned at precise depth for cinematic effect" }
                            </p>
                        </UIDepth>
                    </LayerGroup>
                </ParallaxSection>
            </div>
            
            // Section 3: Call to Action
            <div style="scroll-snap-align: start; scroll-snap-stop: always; min-height: 100vh;"
            >
                <ParallaxSection section_index={2} height_vh={100}>
                    <LayerGroup base_depth={0.6}>
                        // Heading
                        <UIDepth depth={0.8} shadow={0.5} z_offset={20}>
                            <h2 class="text-5xl font-light text-center mb-6"
                                style="color: var(--color-surface-50);"
                            >
                                { "Ready to Start?" }
                            </h2>
                        </UIDepth>
                        
                        // Button group
                        <div class="flex flex-wrap gap-4 justify-center"
                        >
                            <UIDepth depth={0.85} shadow={0.4} z_offset={25}>
                                <button class="px-8 py-4 rounded-full text-lg font-medium transition-transform hover:scale-105"
                                        style="background: var(--color-highlight-500); color: var(--color-neutral-950);"
                                >
                                    { "Get Started" }
                                </button>
                            </UIDepth>
                            
                            <UIDepth depth={0.9} shadow={0.3} z_offset={22}>
                                <button class="px-8 py-4 rounded-full text-lg font-medium border-2 transition-transform hover:scale-105"
                                        style="border-color: var(--color-surface-600); color: var(--color-surface-50);"
                                >
                                    { "Learn More" }
                                </button>
                            </UIDepth>
                        </div>
                    </LayerGroup>
                </ParallaxSection>
            </div>
            
            // Scroll Progress Indicator (optional)
            <div class="fixed right-6 top-1/2 -translate-y-1/2 z-50 flex flex-col gap-2"
            >
                { for (0..3).map(|i| {
                    let is_active = i == *active_section;
                    html! {
                        <div class={if is_active { "w-3 h-3 rounded-full" } else { "w-2 h-2 rounded-full opacity-50" }}
                             style={if is_active { 
                                 String::from("background: var(--color-highlight-500); box-shadow: 0 0 10px var(--color-highlight-500);")
                             } else {
                                 String::from("background: var(--color-surface-600);")
                             }}
                        />
                    }
                })}
            </div>
        </div>
    }
}
