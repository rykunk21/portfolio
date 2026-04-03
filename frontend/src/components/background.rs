use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// Fixed parallax background with beach/ocean scene
/// Layers extend beyond viewport to prevent hard edges
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
            closure.forget();
        });
    }
    
    // Calculate parallax offsets - extended range for more depth
    let nebula_offset = *scroll_y * 0.02;      // Z-5: very deep
    let star_far_offset = *scroll_y * 0.05;      // Z-4: far stars
    let star_near_offset = *scroll_y * 0.08;     // Z-3: near stars
    let moon_offset = *scroll_y * 0.10;         // Z-2: moon
    let ocean_offset = *scroll_y * 0.15;         // Z-1: ocean
    let beach_offset = *scroll_y * 0.25;        // Z-0: beach
    
    // Fire intensity for lighting effect (pulsing)
    let time = use_state(|| 0.0_f64);
    {
        let time = time.clone();
        use_effect_with((), move |_| {
            let mut i = 0.0;
            spawn_local(async move {
                loop {
                    sleep_ms(50).await;
                    i += 0.05;
                    time.set(i);
                }
            });
            || {}
        });
    }
    
    // Calculate fire glow intensity (0.3 to 0.7 range)
    let fire_intensity = 0.5 + 0.2 * (*time).sin();
    let fire_warmth = (0.3 + 0.15 * ((*time * 1.3).sin())) as f32;
    
    html! {
        <div class="relative min-h-screen overflow-x-hidden">
            // Base gradient - extends infinitely
            <div class="fixed inset-0 -top-[100vh] -bottom-[50vh]" 
                style="background: linear-gradient(180deg, #050510 0%, #0a0a1a 20%, #0f1419 60%, #1a1510 100%);"
            />
            
            // Layer Z-5: Deep nebula/mist (very subtle)
            <div class="fixed -top-[100vh] -left-[20vw] -right-[20vw] -bottom-[50vh] pointer-events-none opacity-40"
                style={format!("transform: translateY({}px);", nebula_offset)}>
                <div class="absolute inset-0" 
                    style="background: radial-gradient(ellipse 80% 50% at 30% 20%, rgba(40,30,60,0.4) 0%, transparent 60%);">
                </div>
                <div class="absolute inset-0"
                    style="background: radial-gradient(ellipse 60% 40% at 70% 30%, rgba(30,40,50,0.3) 0%, transparent 50%);">
                </div>
            </div>
            
            // Layer Z-4: Far stars
            <div class="fixed -top-[100vh] inset-x-0 -bottom-[50vh] pointer-events-none"
                style={format!("transform: translateY({}px);", star_far_offset)}>
                { for (0..80).map(|i| {
                    let top = ((i * 137) % 150) as f32; // 0-150vh spread
                    let left = ((i * 73) % 100) as f32;
                    let size = if i % 7 == 0 { 3 } else if i % 3 == 0 { 2 } else { 1 };
                    let opacity = 0.3 + ((i % 5) as f32 * 0.15);
                    html! {
                        <div class="star-far"
                            style={format!(
                                "top: {}vh; left: {}%; width: {}px; height: {}px; opacity: {};",
                                top, left, size, size, opacity
                            )}>
                        </div>
                    }
                })}
            </div>
            
            // Layer Z-3: Near stars (brighter, larger)
            <div class="fixed -top-[100vh] inset-x-0 -bottom-[50vh] pointer-events-none"
                style={format!("transform: translateY({}px);", star_near_offset)}>
                { for (0..40).map(|i| {
                    let top = ((i * 217) % 140) as f32;
                    let left = ((i * 137) % 100) as f32;
                    let size = if i % 5 == 0 { 4 } else { 2 };
                    let opacity = 0.5 + ((i % 4) as f32 * 0.15);
                    let blur = if i % 10 == 0 { 2 } else { 0 };
                    html! {
                        <div class="star-near"
                            style={format!(
                                "top: {}vh; left: {}%; width: {}px; height: {}px; opacity: {}; filter: blur({}px);",
                                top, left, size, size, opacity, blur
                            )}>
                        </div>
                    }
                })}
            </div>
            
            // Layer Z-2: Moon
            <div class="fixed -top-[50vh] inset-x-0 -bottom-[50vh] pointer-events-none"
                style={format!("transform: translateY({}px);", moon_offset)}>
                <div class="absolute" 
                    style="top: 15vh; right: 15%; width: 80px; height: 80px; border-radius: 50%;
                           background: radial-gradient(circle at 30% 30%, #f5f0e6, #d4cfc4, #b0aba0);
                           box-shadow: 0 0 60px rgba(245, 240, 230, 0.3), 0 0 120px rgba(245, 240, 230, 0.1);">
                </div>
                // Moon craters (subtle)
                <div class="absolute" style="top: calc(15vh + 20px); right: calc(15% + 15px); width: 15px; height: 15px; 
                       border-radius: 50%; background: rgba(180,175,170,0.3); box-shadow: inset 1px 1px 2px rgba(0,0,0,0.1);"></div>
                <div class="absolute" style="top: calc(15vh + 45px); right: calc(15% + 35px); width: 10px; height: 10px; 
                       border-radius: 50%; background: rgba(180,175,170,0.3); box-shadow: inset 1px 1px 2px rgba(0,0,0,0.1);"></div>
            </div>
            
            // Layer Z-1: Ocean (distant)
            <div class="fixed bottom-0 left-0 right-0 h-[40vh] pointer-events-none"
                style={format!("transform: translateY({}px); 
                               background: linear-gradient(to top, #0a1929 0%, #0d1b2a 30%, #1b263b 70%, transparent 100%);",
                               ocean_offset)}>
                // Wave lines
                <div class="absolute top-[10%] left-0 right-0 h-px" 
                     style="background: linear-gradient(90deg, transparent 0%, rgba(100,130,160,0.3) 20%, rgba(100,130,160,0.3) 80%, transparent 100%);"></div>
                <div class="absolute top-[25%] left-0 right-0 h-px"
                     style="background: linear-gradient(90deg, transparent 10%, rgba(100,130,160,0.2) 30%, rgba(100,130,160,0.2) 70%, transparent 100%);"></div>
                <div class="absolute top-[45%] left-0 right-0 h-px"
                     style="background: linear-gradient(90deg, transparent 0%, rgba(100,130,160,0.15) 25%, rgba(100,130,160,0.15) 75%, transparent 100%);"></div>
                // Moon reflection
                <div class="absolute top-[5%] right-[calc(15%-20px)] w-[100px] h-[30px]"
                     style="background: linear-gradient(180deg, rgba(245,240,230,0.15) 0%, transparent 100%); 
                            filter: blur(8px); transform: scaleY(0.3);"></div>
            </div>
            
            // Layer Z-0: Beach (foreground)
            <div class="fixed bottom-0 left-0 right-0 h-[25vh] pointer-events-none"
                style={format!("transform: translateY({}px);", beach_offset)}>
                // Sand base
                <div class="absolute bottom-0 inset-x-0 h-full"
                     style="background: linear-gradient(to top, #0f1410 0%, #1a1814 40%, #1e1a14 70%, transparent 100%);">
                </div>
                // Dune shapes
                <div class="absolute bottom-0 left-[-10%] w-[50%] h-[80%] rounded-full opacity-60"
                     style="background: radial-gradient(ellipse 100% 80% at 50% 100%, #161411 0%, transparent 70%);"></div>
                <div class="absolute bottom-0 left-[20%] w-[60%] h-[100%] rounded-full opacity-50"
                     style="background: radial-gradient(ellipse 100% 90% at 50% 100%, #1a1612 0%, transparent 70%);"></div>
                <div class="absolute bottom-0 right-[-10%] w-[50%] h-[70%] rounded-full opacity-70"
                     style="background: radial-gradient(ellipse 100% 70% at 50% 100%, #141210 0%, transparent 70%);"></div>
                // Small rocks/details
                <div class="absolute bottom-[10%] right-[25%] w-4 h-3 rounded-[40%]"
                     style="background: #0c0a08; box-shadow: 1px 1px 2px rgba(0,0,0,0.5);"></div>
                <div class="absolute bottom-[15%] right-[28%] w-2 h-2 rounded-[40%]"
                     style="background: #0d0b09; box-shadow: 1px 1px 2px rgba(0,0,0,0.5);"></div>
                <div class="absolute bottom-[8%] right-[22%] w-3 h-2 rounded-[40%]"
                     style="background: #0a0806; box-shadow: 1px 1px 2px rgba(0,0,0,0.5);"></div>
            </div>
            
            // Fire glow overlay (lighting effect only, no visible campfire)
            <div class="fixed bottom-0 right-0 w-[40vw] h-[50vh] pointer-events-none"
                 style={format!("background: radial-gradient(ellipse 80% 60% at 70% 100%, rgba(250,136,5,{:.2}) 0%, rgba(200,80,0,{:.2}) 30%, rgba(100,40,0,0.1) 55%, transparent 70%);",
                        fire_intensity, fire_intensity * 0.6)}>
            </div>
            
            // Warm tint overlay (subtle color grading)
            <div class="fixed inset-0 pointer-events-none"
                 style={format!("background: radial-gradient(ellipse 50% 40% at 80% 100%, rgba(250,150,50,{:.3}) 0%, transparent 60%); mix-blend-mode: overlay;",
                        fire_warmth)}>
            </div>
            
            // Content layer
            <div class="relative z-10">
                { props.children.clone() }
            </div>
            
            // CSS styles
            <style>
                {".star-far, .star-near {
                    position: absolute;
                    border-radius: 50%;
                    background: white;
                }
                .star-near {
                    box-shadow: 0 0 4px rgba(255,255,255,0.5);
                }"}
            </style>
        </div>
    }
}

async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(
        &mut |resolve, _| {
            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, ms as i32
            );
        }
    );
    wasm_bindgen_futures::JsFuture::from(promise).await.ok();
}
