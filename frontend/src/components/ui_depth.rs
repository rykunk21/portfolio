use yew::prelude::*;
/// UIDepth: Adds parallax depth to floating UI elements
/// Elements move at different speeds relative to scroll
/// Works with global page scroll, no containers needed
#[derive(Properties, PartialEq, Clone)]
pub struct UIDepthProps {
    #[prop_or_default]
    pub children: Children,
    /// Parallax speed (0.0 = fixed, 1.0 = normal scroll speed)
    /// Lower = appears farther away, moves slower
    #[prop_or(1.0)]
    pub depth: f64,
    /// Shadow intensity (0-1)
    #[prop_or(0.3)]
    pub shadow: f64,
    /// Z-index for layering
    #[prop_or(0)]
    pub z_offset: i32,
    /// Optional: vertical offset from natural position in pixels
    #[prop_or(0)]
    pub y_offset: i32,
}

#[function_component(UIDepth)]
pub fn ui_depth(props: &UIDepthProps) -> Html {
    let scroll_y = use_state(|| 0.0_f64);
    
    {
        let scroll_y = scroll_y.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Ok(y) = w.scroll_y() {
                        scroll_y.set(y);
                    }
                }
            }) as Box<dyn FnMut()>);
            
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
            closure.forget();
        });
    }
    
    // Parallax offset: element moves at (depth) * scroll_speed
    // depth < 1.0 makes it lag behind (appears distant)
    // depth > 1.0 makes it move faster (appears close)
    let parallax_offset = -(*scroll_y * (1.0 - props.depth)) + (props.y_offset as f64);
    
    // Calculate shadow
    let shadow_blur = 8.0 + (props.shadow * 16.0);
    let shadow_opacity = props.shadow * 0.35;
    let shadow_y = 2.0 + (props.shadow * 6.0);
    let shadow_x = (1.0 - props.depth) * 2.0; // slight tilt based on depth
    
    let z_index = 10 + props.z_offset;
    
    html! {
        <div 
            style={format!(
                "transform: translate3d({:.2}px, {:.2}px, 0); z-index: {}; filter: drop-shadow({:.1}px {:.1}px {:.1}px rgba(0,0,0,{:.2}));",
                shadow_x, parallax_offset, z_index, shadow_x * -1.0, shadow_y, shadow_blur, shadow_opacity
            )}
        >
            { props.children.clone() }
        </div>
    }
}

/// FloatingElement: Decorative elements that float around content
#[derive(Properties, PartialEq, Clone)]
pub struct FloatingElementProps {
    #[prop_or_default]
    pub children: Children,
    /// Depth (0.2 = far background, 0.8 = near foreground, 1.0 = content layer)
    #[prop_or(0.5)]
    pub depth: f64,
    /// Base position from section top (%) - use percentage for responsive
    #[prop_or(50)]
    pub top_pct: i32,
    /// Base position from left (%) - use percentage for responsive
    #[prop_or(50)]
    pub left_pct: i32,
    /// Width in pixels
    #[prop_or(100)]
    pub width: i32,
    /// Height in pixels
    #[prop_or(100)]
    pub height: i32,
}

#[function_component(FloatingElement)]
pub fn floating_element(props: &FloatingElementProps) -> Html {
    let scroll_y = use_state(|| 0.0_f64);
    
    {
        let scroll_y = scroll_y.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Ok(y) = w.scroll_y() {
                        scroll_y.set(y);
                    }
                }
            }) as Box<dyn FnMut()>);
            
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
            closure.forget();
        });
    }
    
    // Parallax: distance from content layer
    let depth_offset = *scroll_y * (1.0 - props.depth) * -1.0;
    
    // Calculate opacity based on depth (farther = more transparent)
    let opacity = 0.3 + (props.depth * 0.5);
    // Scale: farther = smaller
    let scale = 0.7 + (props.depth * 0.3);
    
    html! {
        <div
            class="absolute pointer-events-none"
            style={format!(
                "top: {}%; left: {}%; width: {}px; height: {}px; opacity: {:.2}; transform: translateY({:.1}px) scale({:.2}); z-index: {};",
                props.top_pct, props.left_pct, props.width, props.height, opacity, depth_offset, scale, (props.depth * 10.0) as i32
            )}
        >
            { props.children.clone() }
        </div>
    }
}

/// Section wrapper that provides context for floating elements
#[derive(Properties, PartialEq)]
pub struct ParallaxLayerProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(ParallaxLayer)]
pub fn parallax_layer(props: &ParallaxLayerProps) -> Html {
    html! {
        <div class="relative overflow-visible">
            { props.children.clone() }
        </div>
    }
}
