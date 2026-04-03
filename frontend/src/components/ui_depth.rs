use yew::prelude::*;
use wasm_bindgen::prelude::*;

/// UI Depth System: Adds shadow and parallax to content elements
/// Works with scroll snap to ensure each viewport shows clean composition
#[derive(Properties, PartialEq, Clone)]
pub struct UIDepthProps {
    #[prop_or_default]
    pub children: Children,
    /// Parallax speed relative to viewport (0.0 = fixed, 1.0 = normal scroll)
    #[prop_or(1.0)]
    pub depth: f64,
    /// Shadow intensity (0.0 = none, 1.0 = deep shadow)
    #[prop_or(0.3)]
    pub shadow: f64,
    /// Z-index offset for layering
    #[prop_or(0)]
    pub z_offset: i32,
}

/// Individual element with depth and shadow
#[function_component(UIDepth)]
pub fn ui_depth(props: &UIDepthProps) -> Html {
    let scroll_y = use_state(|| 0.0_f64);
    let viewport_index = use_state(|| 0_usize);
    
    {
        let scroll_y = scroll_y.clone();
        let viewport_index = viewport_index.clone();
        use_effect_with((), move |_| {
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            
            let closure = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Ok(y) = w.scroll_y() {
                        scroll_y.set(y);
                        // Calculate viewport index (100vh per section)
                        let vh_js = w.inner_height().unwrap_or_else(|_| wasm_bindgen::JsValue::from_f64(800.0));
                        let vh = vh_js.as_f64().unwrap_or(800.0);
                        let idx = (y / vh) as usize;
                        viewport_index.set(idx);
                    }
                }
            }) as Box<dyn FnMut()>);
            
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            closure.forget();
        });
    }
    
    // Calculate offset based on position within viewport
    let vh = 800.0;
    let position_in_viewport = *scroll_y % vh;
    let parallax_offset = position_in_viewport * (1.0 - props.depth);
    
    // Calculate shadow based on depth
    let shadow_blur = 10.0 + (props.shadow * 20.0);
    let shadow_opacity = props.shadow * 0.4;
    let shadow_y = 4.0 + (props.shadow * 8.0);
    
    // Warm lighting from fire (bottom right)
    let fire_distance = 100.0 - (*scroll_y % vh);
    let fire_glow = if fire_distance < 200.0 {
        (1.0 - fire_distance / 200.0) * 0.15 * props.shadow
    } else {
        0.0
    };
    
    let z_index = 10 + props.z_offset;
    
    html! {
        <div 
            class="ui-depth-element"
            style={format!(
                "transform: translateY({:.2}px); z-index: {}; --shadow-blur: {:.1}px; --shadow-y: {:.1}px; --shadow-opacity: {:.2}; --fire-glow: {:.3};",
                parallax_offset, z_index, shadow_blur, shadow_y, shadow_opacity, fire_glow
            )}
        >
            <div class="ui-depth-shadow"
                style={format!(
                    "box-shadow: 0 {:.1}px {:.1}px rgba(0,0,0,{:.2}), 0 0 {:.1}px rgba(250,136,5,{:.3});",
                    shadow_y, shadow_blur, shadow_opacity, shadow_blur * 1.5, fire_glow
                )}
            >
                { props.children.clone() }
            </div>
            
            <style>
                {".ui-depth-element {
                    position: relative;
                    will-change: transform;
                    transition: transform 0.1s linear;
                }
                
                .ui-depth-shadow {
                    position: relative;
                    border-radius: inherit;
                }"}
            </style>
        </div>
    }
}

/// Section wrapper that provides viewport context
#[derive(Properties, PartialEq)]
pub struct ParallaxSectionProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(0)]
    pub section_index: usize,
    /// Height of section (default 100vh for snap)
    #[prop_or(100)]
    pub height_vh: usize,
}

#[function_component(ParallaxSection)]
pub fn parallax_section(props: &ParallaxSectionProps) -> Html {
    html! {
        <section 
            class="relative min-h-screen flex flex-col items-center justify-center px-6"
            data-section={props.section_index.to_string()}
        >
            { props.children.clone() }
        </section>
    }
}

/// Container for layered elements within a section
#[derive(Properties, PartialEq)]
pub struct LayerGroupProps {
    #[prop_or_default]
    pub children: Children,
    /// Base depth for all children (they'll be offset relative to this)
    #[prop_or(0.5)]
    pub base_depth: f64,
}

#[function_component(LayerGroup)]
pub fn layer_group(props: &LayerGroupProps) -> Html {
    html! {
        <div class="relative w-full max-w-6xl mx-auto"
             style={format!("--base-depth: {}", props.base_depth)}
        >
            { props.children.clone() }
        </div>
    }
}
