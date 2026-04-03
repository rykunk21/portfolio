use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// Fixed parallax background with beach scene
/// Multiple star layers + beach layers that interact with UI
#[derive(Properties, PartialEq)]
pub struct BackgroundProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Background)]
pub fn background(props: &BackgroundProps) -> Html {
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
    
    // Parallax offsets - beach moves faster than stars
    let star_layer1_offset = *scroll_y * 0.02;
    let star_layer2_offset = *scroll_y * 0.04;
    let star_layer3_offset = *scroll_y * 0.06;
    let star_layer4_offset = *scroll_y * 0.08;
    let moon_offset = *scroll_y * 0.10;
    let ocean_back_offset = *scroll_y * 0.15;
    let ocean_front_offset = *scroll_y * 0.22;
    let beach_dune_offset = *scroll_y * 0.30;
    let beach_sand_offset = *scroll_y * 0.38;
    
    // Fire animation
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
    
    let fire_intensity = 0.5 + 0.2 * (*time).sin();
    
    html! {
        <div class="relative min-h-screen overflow-x-hidden">
            // Base gradient
            <div class="fixed inset-0" 
                style="background: linear-gradient(180deg, #030308 0%, #080818 15%, #0c1220 40%, #101820 70%, #151410 100%);"
            />
            
            // Star Layer 1: Deepest (200 stars, smallest)
            <div class="fixed inset-0 pointer-events-none"
                style={format!("transform: translateY({}px);", star_layer1_offset)}>
                { generate_stars(200, 1, 0.25, 0.45, "star-dim".to_string()) }
            </div>
            
            // Star Layer 2: Far (150 stars)
            <div class="fixed inset-0 pointer-events-none"
                style={format!("transform: translateY({}px);", star_layer2_offset)}>
                { generate_stars(150, 2, 0.3, 0.55, "star-far".to_string()) }
            </div>
            
            // Star Layer 3: Mid (100 stars)
            <div class="fixed inset-0 pointer-events-none"
                style={format!("transform: translateY({}px);", star_layer3_offset)}>
                { generate_stars(100, 3, 0.35, 0.65, "star-mid".to_string()) }
            </div>
            
            // Star Layer 4: Near (50 stars, brightest)
            <div class="fixed inset-0 pointer-events-none"
                style={format!("transform: translateY({}px);", star_layer4_offset)}>
                { generate_stars(50, 4, 0.5, 0.9, "star-near".to_string()) }
            </div>
            
            // Moon
            <div class="fixed -top-[20vh] inset-x-0 -bottom-[50vh] pointer-events-none"
                style={format!("transform: translateY({}px);", moon_offset)}>
                <div class="absolute" 
                    style="top: 20vh; right: 20%; width: 60px; height: 60px; border-radius: 50%;
                           background: radial-gradient(circle at 35% 35%, #fff8f0 0%, #e8e0d8 30%, #c8c0b8 100%);
                           box-shadow: 0 0 40px rgba(255,248,240,0.25), 0 0 80px rgba(255,248,240,0.1);">
                </div>
                <div class="absolute" 
                    style="top: 75vh; right: calc(20% + 10px); width: 50px; height: 12px; 
                           border-radius: 50%; background: linear-gradient(90deg, transparent, rgba(255,248,240,0.15), transparent);
                           filter: blur(4px);">
                </div>
            </div>
            
            // Ocean Back Layer
            <div class="fixed bottom-0 left-0 right-0 h-[50vh] pointer-events-none"
                style={format!("transform: translateY({}px); background: linear-gradient(to top, #081018 0%, #0c1828 25%, #101d30 60%, transparent 100%);",
                       ocean_back_offset)}>
                <div class="absolute top-[20%] left-0 right-0 h-px opacity-20"
                     style="background: linear-gradient(90deg, transparent 10%, #4a6080 50%, transparent 90%);">
                </div>
                <div class="absolute top-[40%] left-0 right-0 h-px opacity-15"
                     style="background: linear-gradient(90deg, transparent 20%, #405670 50%, transparent 90%);">
                </div>
                <div class="absolute top-[65%] left-0 right-0 h-px opacity-10"
                     style="background: linear-gradient(90deg, transparent 30%, #354a60 50%, transparent 70%);">
                </div>
            </div>
            
            // Ocean Front Layer
            <div class="fixed -bottom-[10vh] left-0 right-0 h-[35vh] pointer-events-none"
                style={format!("transform: translateY({}px); background: linear-gradient(to top, #081218 0%, #0b1620 40%, transparent 100%);",
                       ocean_front_offset)}>
            </div>
            
            // Beach Dune Layer
            <div class="fixed -bottom-[5vh] left-0 right-0 h-[30vh] pointer-events-none"
                style={format!("transform: translateY({}px);", beach_dune_offset)}>
                <div class="absolute bottom-0 left-[-10%] w-[50%] h-[100%]"
                     style="background: radial-gradient(ellipse 100% 100% at 50% 100%, #0c100c 0%, transparent 70%); filter: blur(2px);">
                </div>
                <div class="absolute bottom-0 left-[30%] w-[50%] h-[85%]"
                     style="background: radial-gradient(ellipse 100% 100% at 50% 100%, #0e1210 0%, transparent 70%); filter: blur(1px);">
                </div>
                <div class="absolute bottom-0 right-[-5%] w-[45%] h-[90%]"
                     style="background: radial-gradient(ellipse 100% 100% at 50% 100%, #0a0e0a 0%, transparent 70%);">
                </div>
            </div>
            
            // Beach Sand Layer (obscures UI at bottom)
            <div class="fixed bottom-0 left-0 right-0 h-[25vh] pointer-events-none z-20"
                style={format!("transform: translateY({}px); background: linear-gradient(to top, #0c100a 0%, #101412 30%, transparent 100%);",
                       beach_sand_offset)}>
                <div class="absolute bottom-[15%] right-[22%] w-3 h-2 rounded-[30%]"
                     style="background: #080807; box-shadow: 1px 1px 2px rgba(0,0,0,0.5);">
                </div>
                <div class="absolute bottom-[20%] right-[25%] w-2 h-2 rounded-[40%]"
                     style="background: #0a0908; box-shadow: 1px 1px 2px rgba(0,0,0,0.5);">
                </div>
                <div class="absolute bottom-[12%] right-[28%] w-4 h-3 rounded-[35%]"
                     style="background: #070606; box-shadow: 1px 1px 3px rgba(0,0,0,0.6);">
                </div>
            </div>
            
            // Fire glow
            <div class="fixed bottom-0 right-0 w-[50vw] h-[40vh] pointer-events-none z-10"
                 style={format!("background: radial-gradient(ellipse 70% 50% at 80% 100%, rgba(250,136,5,{:.2}) 0%, rgba(180,80,10,{:.2}) 35%, transparent 70%);",
                        fire_intensity, fire_intensity * 0.6)}>
            </div>
            
            // Content layer
            <div class="relative z-30">
                { props.children.clone() }
            </div>
            
            // CSS
            <style>
                {".star-dim, .star-far, .star-mid, .star-near {
                    position: absolute;
                    border-radius: 50%;
                    background: white;
                }
                .star-near {
                    box-shadow: 0 0 6px rgba(255,255,255,0.6), 0 0 12px rgba(255,255,255,0.3);
                }
                .star-mid {
                    box-shadow: 0 0 4px rgba(255,255,255,0.4);
                }
                .star-far {
                    box-shadow: 0 0 2px rgba(255,255,255,0.3);
                }"}
            </style>
        </div>
    }
}

fn generate_stars(count: usize, max_size: usize, min_opacity: f32, max_opacity: f32, class_name: String) -> Html {
    use js_sys::Math::random;
    
    let stars: Vec<Html> = (0..count).map(|_| {
        // True random positioning
        let top = (random() * 100.0) as f32;
        let left = (random() * 100.0) as f32;
        
        // Random size within range
        let size = 1 + (random() * max_size as f64) as usize;
        
        // Random opacity within range
        let opacity = min_opacity + (random() as f32) * (max_opacity - min_opacity);
        
        html! {
            <div class={class_name.clone()}
                style={format!(
                    "top: {}%; left: {}%; width: {}px; height: {}px; opacity: {:.2};",
                    top, left, size, size, opacity
                )}
            >
            </div>
        }
    }).collect();
    
    html! { <>{ stars }</> }
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
