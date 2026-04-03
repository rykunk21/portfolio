use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

/// Phase 1: Static 4-layer parallax background
/// Beach at night scene with depth via scroll-speed differences
#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
    // Track scroll position for parallax
    let scroll_y = use_state(|| 0.0_f64);
    
    {
        let scroll_y = scroll_y.clone();
        use_effect_with((), move |_| {
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            
            let window_for_closure = window.clone();
            let closure = Closure::wrap(Box::new(move || {
                if let Ok(y) = window_for_closure.scroll_y() {
                    scroll_y.set(y);
                }
            }) as Box<dyn FnMut()>);
            
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            
            // Closure will be kept alive by JS event listener
            closure.forget();
        });
    }
    
    // Calculate parallax offsets for each layer
    let star_offset = *scroll_y * 0.05;      // Z-4: barely moves
    let ocean_offset = *scroll_y * 0.15;     // Z-3: slow
    let beach_offset = *scroll_y * 0.30;     // Z-2: medium
    let fire_offset = *scroll_y * 0.50;      // Z-1: fastest (but still slower than content)
    
    html! {
        <div class="relative min-h-screen bg-neutral-950 overflow-x-hidden">
            // Layer Z-4: Star field (deepest)
            <div 
                class="fixed inset-0 pointer-events-none"
                style={format!(
                    "transform: translateY({}px); background: {};",
                    star_offset,
                    "radial-gradient(ellipse at bottom, #1a1a2e 0%, #0f0f1a 40%, #000000 100%)"
                )}
            >
                // Static stars (CSS-generated)
                <div class="stars-layer">
                    // Top-left cluster
                    <div class="star" style="top: 10%; left: 15%; opacity: 0.8; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 12%; left: 18%; opacity: 0.5; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 8%; left: 22%; opacity: 0.9; width: 3px; height: 3px;"></div>
                    <div class="star" style="top: 15%; left: 12%; opacity: 0.6; width: 1px; height: 1px;"></div>
                    
                    // Top-right cluster
                    <div class="star" style="top: 5%; left: 75%; opacity: 0.7; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 8%; left: 80%; opacity: 0.4; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 3%; left: 85%; opacity: 0.8; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 12%; left: 78%; opacity: 0.5; width: 1px; height: 1px;"></div>
                    
                    // Mid-left
                    <div class="star" style="top: 25%; left: 5%; opacity: 0.6; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 28%; left: 8%; opacity: 0.8; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 22%; left: 11%; opacity: 0.4; width: 2px; height: 2px;"></div>
                    
                    // Mid-right
                    <div class="star" style="top: 20%; left: 90%; opacity: 0.9; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 23%; left: 93%; opacity: 0.5; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 18%; left: 88%; opacity: 0.7; width: 1px; height: 1px;"></div>
                    
                    // Scattered
                    <div class="star" style="top: 35%; left: 30%; opacity: 0.6; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 38%; left: 45%; opacity: 0.4; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 32%; left: 60%; opacity: 0.8; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 40%; left: 70%; opacity: 0.5; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 45%; left: 25%; opacity: 0.7; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 15%; left: 50%; opacity: 0.6; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 30%; left: 85%; opacity: 0.5; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 42%; left: 10%; opacity: 0.8; width: 2px; height: 2px;"></div>
                    <div class="star" style="top: 48%; left: 55%; opacity: 0.4; width: 1px; height: 1px;"></div>
                    <div class="star" style="top: 8%; left: 40%; opacity: 0.9; width: 2px; height: 2px;"></div>
                </div>
            </div>
            
            // Layer Z-3: Ocean (distant)
            <div 
                class="fixed bottom-0 left-0 right-0 h-96 pointer-events-none"
                style={format!(
                    "transform: translateY({}px); background: {};",
                    ocean_offset,
                    "linear-gradient(to top, #0d1b2a 0%, #1b263b 30%, transparent 100%)"
                )}
            >
                // Subtle wave line at horizon
                <div class="absolute top-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-slate-600 to-transparent opacity-30"></div>
            </div>
            
            // Layer Z-2: Beach silhouette
            <div 
                class="fixed bottom-0 left-0 right-0 h-64 pointer-events-none"
                style={format!(
                    "transform: translateY({}px); {}",
                    beach_offset,
                    "background: linear-gradient(to top, #0f1419 0%, #1a1f2e 40%, transparent 100%);"
                )}
            >
                // Dune silhouette (CSS shapes)
                <div class="beach-dunes absolute bottom-0 left-0 right-0 h-full"></div>
            </div>
            
            // Layer Z-1: Campfire base (foreground element)
            <div 
                class="fixed bottom-8 right-12 w-32 h-40 pointer-events-none"
                style={format!(
                    "transform: translateY({}px);",
                    fire_offset
                )}
            >
                // Static campfire logs (Phase 1 - no animation)
                <div class="absolute bottom-0 left-1/2 -translate-x-1/2 w-20 h-10">
                    // Log 1
                    <div class="absolute bottom-2 left-2 w-16 h-3 rounded-full transform rotate-12"
                         style="background: #3d2817; box-shadow: 0 2px 4px rgba(0,0,0,0.5);">
                    </div>
                    // Log 2
                    <div class="absolute bottom-2 right-2 w-14 h-3 rounded-full transform -rotate-12"
                         style="background: #4a301c; box-shadow: 0 2px 4px rgba(0,0,0,0.5);">
                    </div>
                    // Log 3 (crossing)
                    <div class="absolute bottom-4 left-4 w-12 h-3 rounded-full transform rotate-45"
                         style="background: #362312; box-shadow: 0 2px 4px rgba(0,0,0,0.5);">
                    </div>
                </div>
                
                // Static flame placeholder (red glow, no animation yet)
                <div class="absolute bottom-8 left-1/2 -translate-x-1/2 w-16 h-24 rounded-full opacity-40 blur-xl"
                     style="background: radial-gradient(circle, #fa8805 0%, #ff6b00 30%, transparent 70%);">
                </div>
            </div>
            
            // Content layer (Z-0, normal scroll)
            <div class="relative z-10">
                { props.children.clone() }
            </div>
            
            // CSS styles for stars and dunes
            <style>
                {".stars-layer {
                    position: absolute;
                    inset: 0;
                }
                
                .star {
                    position: absolute;
                    border-radius: 50%;
                    background: white;
                    box-shadow: 0 0 4px rgba(255, 255, 255, 0.4);
                }
                
                .star:nth-child(1), .star:nth-child(7), .star:nth-child(19) {
                    box-shadow: 0 0 6px rgba(255, 255, 255, 0.6), 0 0 12px rgba(255, 255, 255, 0.3);
                }
                
                .beach-dunes {
                    background: 
                        radial-gradient(ellipse 60% 40% at 20% 100%, #1a1f2e 0%, transparent 50%),
                        radial-gradient(ellipse 50% 35% at 45% 100%, #151922 0%, transparent 45%),
                        radial-gradient(ellipse 70% 45% at 75% 100%, #1e2330 0%, transparent 50%);
                }"}
            </style>
        </div>
    }
}
