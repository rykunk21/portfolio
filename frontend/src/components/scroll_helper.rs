use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// ScrollHelper: Visual indicator showing scroll progress
/// Displays dots/progress for each section viewport
#[derive(Properties, PartialEq)]
pub struct ScrollHelperProps {
    #[prop_or(5)]
    pub section_count: usize,
    /// Active section index (0-based)
    #[prop_or(0)]
    pub active_section: usize,
    /// Color for inactive dots
    #[prop_or(String::from("rgba(255,255,255,0.3)"))]
    pub inactive_color: String,
    /// Color for active dot
    #[prop_or(String::from("#fa8805"))]
    pub active_color: String,
}

#[function_component(ScrollHelperDots)]
pub fn scroll_helper_dots(props: &ScrollHelperProps) -> Html {
    html! {
        <div class="fixed right-6 top-1/2 -translate-y-1/2 z-50 flex flex-col gap-3"
            style="mix-blend-mode: difference;"
        >
            { for (0..props.section_count).map(|i| {
                let is_active = i == props.active_section;
                let size = if is_active { "w-3 h-3" } else { "w-2 h-2" };
                let opacity = if is_active { "1" } else { "0.5" };
                let color = if is_active { &props.active_color } else { &props.inactive_color };
                
                html! {
                    <button
                        class={classes!(size, "rounded-full", "transition-all", "duration-300")}
                        style={format!(
                            "background-color: {}; opacity: {}; box-shadow: 0 0 10px {};",
                            color, opacity, if is_active { color } else { "transparent" }
                        )}
                        aria-label={format!("Go to section {}", i + 1)}
                    />
                }
            })}
        </div>
    }
}

/// AutoScrollHelper: Automatically tracks section positions and updates active dot
#[function_component(AutoScrollHelper)]
pub fn auto_scroll_helper() -> Html {
    let active_section = use_state(|| 0_usize);
    let section_count = use_state(|| 5_usize);

    {
        let active_section = active_section.clone();
        let section_count = section_count.clone();
        
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let Ok(scroll_y) = window.scroll_y() {
                        if let Ok(vh) = window.inner_height() {
                            let viewport_height = vh.as_f64().unwrap_or(800.0);
                            let total_height = window
                                .document()
                                .and_then(|d| d.body())
                                .map(|b| b.scroll_height() as f64)
                                .unwrap_or(0.0);
                            
                            // Calculate which viewport we're in
                            let raw_idx = (scroll_y / viewport_height) as usize;
                            let max_idx = (total_height / viewport_height) as usize;
                            let idx = raw_idx.min(max_idx);
                            
                            active_section.set(idx);
                            section_count.set(max_idx.max(5));
                        }
                    }
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
                // Initial call
                closure.as_ref().unchecked_ref::<js_sys::Function>()
                    .call0(&JsValue::NULL).ok();
            }
            closure.forget();
        });
    }

    html! {
        <ScrollHelperDots 
            section_count={*section_count}
            active_section={*active_section}
        />
    }
}

/// ScrollProgressBar: Horizontal progress indicator at top
#[function_component(ScrollProgressBar)]
pub fn scroll_progress_bar() -> Html {
    let progress = use_state(|| 0.0_f64);

    {
        let progress = progress.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let (Ok(scroll_y), Some(document)) = (window.scroll_y(), window.document()) {
                        if let Some(body) = document.body() {
                            let total_height = body.scroll_height() as f64;
                            let viewport_height = window.inner_height()
                                .unwrap_or_else(|_| wasm_bindgen::JsValue::from_f64(800.0))
                                .as_f64()
                                .unwrap_or(800.0);
                            let scrollable = total_height - viewport_height;
                            let pct = if scrollable > 0.0 {
                                (scroll_y / scrollable).clamp(0.0, 1.0)
                            } else {
                                0.0
                            };
                            progress.set(pct);
                        }
                    }
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
            closure.forget();
        });
    }

    let width_pct = (*progress * 100.0) as i32;

    html! {
        <div class="fixed top-0 left-0 right-0 h-1 z-[100] bg-neutral-900/50"
        >
            <div 
                class="h-full transition-all duration-150 ease-out"
                style={format!(
                    "width: {}%; background: linear-gradient(90deg, {}, {}); box-shadow: 0 0 10px {};",
                    width_pct, "#fa8805", "#ff6b35", "rgba(250,136,5,0.5)"
                )}
            />
        </div>
    }
}
