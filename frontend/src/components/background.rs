use yew::prelude::*;
use wasm_bindgen::prelude::*;

/// Immersive background with starfield and ember particles
/// Creates depth perception and draws eye downward toward content
#[function_component(ImmersiveBackground)]
pub fn immersive_background() -> Html {
    let scroll_y = use_state(|| 0.0);
    
    // Track scroll position for parallax
    {
        let scroll_y = scroll_y.clone();
        use_effect_with((), move |_| {
            let window = web_sys::window().expect("window not available");
            
            let scroll_handler = Closure::wrap(Box::new(move || {
                let y = window.scroll_y().unwrap_or(0.0);
                scroll_y.set(y);
            }) as Box<dyn Fn()>);
            
            window
                .add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref())
                .unwrap();
            
            // Initial position
            scroll_y.set(window.scroll_y().unwrap_or(0.0));
            
            // Leak the closure to keep it alive (simpler than full cleanup for this effect)
            scroll_handler.forget();
            
            || {}
        });
    }
    
    let stars_html: Html = (0..150).map(|i| {
        let size = if i % 10 == 0 { 3 } else if i % 5 == 0 { 2 } else { 1 };
        let left = (i * 137) % 100; // Pseudo-random distribution
        let top = (i * 89) % 100;
        let delay = (i as f64 * 0.1) % 5.0;
        let duration = 2.0 + (i as f64 * 0.05) % 3.0;
        
        let style = format!(
            "position: absolute; left: {}%; top: {}%; width: {}px; height: {}px; background: rgba(255, 255, 255, {}); border-radius: 50%; animation: twinkle {}s ease-in-out infinite; animation-delay: {}s;",
            left, top, size, size,
            if size == 3 { "0.9" } else if size == 2 { "0.7" } else { "0.5" },
            duration, delay
        );
        
        html! {
            <div style={style}></div>
        }
    }).collect();
    
    // Ember particles rising from bottom-center
    let embers_html: Html = (0..20).map(|i| {
        let left = 40 + (i * 37) % 20; // Cluster around center bottom (40-60%)
        let delay = (i as f64 * 0.3) % 6.0;
        let duration = 4.0 + (i as f64 * 0.2) % 4.0;
        let size = 2 + (i % 3);
        
        let style = format!(
            "position: absolute; left: {}%; bottom: -10px; width: {}px; height: {}px; background: radial-gradient(circle, rgba(250, 136, 5, 0.9) 0%, rgba(250, 136, 5, 0) 70%); border-radius: 50%; animation: floatUp {}s ease-out infinite; animation-delay: {}s; opacity: 0;",
            left, size, size,
            duration, delay
        );
        
        html! {
            <div style={style}></div>
        }
    }).collect();
    
    // Parallax transforms
    let parallax_slow = format!("translateY({}px)", (*scroll_y * 0.1));
    let parallax_mid = format!("translateY({}px)", (*scroll_y * 0.3));
    let parallax_fast = format!("translateY({}px)", (*scroll_y * 0.5));
    
    html! {
        <div style="position: fixed; top: 0; left: 0; width: 100%; height: 100vh; z-index: -1; overflow: hidden; pointer-events: none;">
            
            // Deep space gradient background
            <div style="position: absolute; inset: 0; background: linear-gradient(to bottom, #0a0a0f 0%, #0f0f14 40%, #1a1510 80%, #231810 100%);"></div>
            
            // Starfield layer (slow parallax)
            <div style={format!("position: absolute; inset: 0; will-change: transform; transform: {};", parallax_slow)}>
                {stars_html}
            </div>
            
            // Subtle desert-sand glow from bottom (mid parallax)
            <div style={format!("position: absolute; bottom: -20vh; left: 30%; width: 40%; height: 60vh; background: radial-gradient(ellipse at center, rgba(176, 121, 79, 0.15) 0%, transparent 70%); filter: blur(40px); will-change: transform; transform: {};", parallax_mid)}></div>
            
            // Ember particles layer (fast parallax)
            <div style={format!("position: absolute; bottom: 0; left: 0; width: 100%; height: 40vh; overflow: hidden; will-change: transform; transform: {};", parallax_fast)}>
                {embers_html}
                
                
                // Warm glow from below viewport center
                <div style="position: absolute; bottom: -50px; left: 50%; transform: translateX(-50%); width: 300px; height: 100px; background: radial-gradient(ellipse at center, rgba(250, 136, 5, 0.3) 0%, transparent 70%); filter: blur(20px);"></div>
            </div>
            
            // CSS animations
            <style>
                {"@keyframes twinkle {
                    0%, 100% { opacity: 0.3; transform: scale(0.8); }
                    50% { opacity: 1; transform: scale(1.2); }
                }
                @keyframes floatUp {
                    0% { transform: translateY(0) translateX(0); opacity: 0; }
                    10% { opacity: 1; }
                    50% { transform: translateY(-20vh) translateX(20px); }
                    100% { transform: translateY(-40vh) translateX(-10px); opacity: 0; }
                }"}
            </style>
        </div>
    }
}
