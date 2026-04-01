use yew::prelude::*;
use web_sys::window;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::spawn_local;

/// Loading screen that displays while critical assets preload
/// Prevents partial image rendering and provides branded entry experience
#[derive(Properties, PartialEq)]
pub struct LoadingScreenProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoadingScreen)]
pub fn loading_screen(props: &LoadingScreenProps) -> Html {
    let is_loading = use_state(|| true);
    let progress = use_state(|| 0);
    
    // Asset preloading logic
    {
        let is_loading = is_loading.clone();
        let progress = progress.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                // Critical assets to preload
                let critical_images = vec![
                    "media/portfolio.png",
                    "media/headshot.jpg",
                    "media/logo.svg",
                ];
                
                let total = critical_images.len();
                let mut loaded = 0;
                
                // Preload each image
                for src in &critical_images {
                    if preload_single_image(src).await {
                        loaded += 1;
                        progress.set((loaded * 100) / total);
                    } else {
                        // Continue even if one fails
                        loaded += 1;
                        progress.set((loaded * 100) / total);
                    }
                }
                
                // Ensure we show 100%
                progress.set(100);
                
                // Small delay for visual completion
                wasm_bindgen_futures::JsFuture::from(js_sys::Promise::new(
                    &mut |resolve, _| {
                        let window = web_sys::window().unwrap();
                        window.set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve, 500
                        ).unwrap();
                    }
                )).await.ok();
                
                // Hide loading screen
                is_loading.set(false);
            });
            
            || {}
        });
    }
    
    html! {
        <>
            // Loading overlay
            if *is_loading {
                <div 
                    style="
                        position: fixed;
                        inset: 0;
                        z-index: 9999;
                        display: flex;
                        flex-direction: column;
                        align-items: center;
                        justify-content: center;
                        background: linear-gradient(135deg, #0a0a0f 0%, #18181b 50%, #231810 100%);
                        transition: opacity 0.5s ease-out;
                    "
                >
                    // Brand/logo area
                    <div style="margin-bottom: 2rem; text-align: center;">
                        <h1 
                            style="
                                font-size: 2.5rem;
                                font-weight: 300;
                                color: #f1f2f4;
                                margin: 0;
                                letter-spacing: 0.05em;
                            "
                        >
                            { "Ryan Kunkel" }
                        </h1>
                        <p style="color: #8f95a3; margin-top: 0.5rem; font-size: 1rem;">
                            { "Loading experience..." }
                        </p>
                    </div>
                    
                    // Animated loading indicator
                    <div style="width: 60px; height: 60px; position: relative; margin-bottom: 2rem;">
                        // Spinner ring
                        <div 
                            style="
                                position: absolute;
                                inset: 0;
                                border: 2px solid rgba(250, 136, 5, 0.2);
                                border-radius: 50%;
                            "
                        ></div>
                        
                        // Animated spinner
                        <div 
                            style="
                                position: absolute;
                                inset: 0;
                                border: 2px solid transparent;
                                border-top-color: #fa8805;
                                border-radius: 50%;
                                animation: spin 1s linear infinite;
                            "
                        ></div>
                    </div>
                    
                    // Progress bar
                    <div 
                        style="
                            width: 200px;
                            height: 2px;
                            background: rgba(255, 255, 255, 0.1);
                            border-radius: 1px;
                            overflow: hidden;
                        "
                    >
                        <div 
                            style={format!(
                                "width: {}%; height: 100%; background: #fa8805; transition: width 0.3s ease-out;",
                                *progress
                            )}
                        ></div>
                    </div>
                    
                    // Progress percentage
                    <p style="color: #8f95a3; font-size: 0.875rem; margin-top: 1rem; font-variant-numeric: tabular-nums;">
                        { format!("{}%", *progress) }
                    </p>
                    
                    // CSS animations
                    <style>
                        {"@keyframes spin {
                            from { transform: rotate(0deg); }
                            to { transform: rotate(360deg); }
                        }"}
                    </style>
                </div>
            }
            
            // Actual content (renders immediately but hidden behind overlay)
            <div style={if *is_loading { "visibility: hidden; height: 0; overflow: hidden;" } else { "" }}
            >
                { props.children.clone() }
            </div>
        </>
    }
}

/// Preload a single image using a simple Promise-based approach
async fn preload_single_image(src: &str) -> bool {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return false,
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return false,
    };
    
    // Create image element
    let img = match document.create_element("img") {
        Ok(i) => i,
        Err(_) => return false,
    };
    
    let img_element = match img.dyn_into::<web_sys::HtmlImageElement>() {
        Ok(el) => el,
        Err(_) => return false,
    };
    
    img_element.set_src(src);
    
    // Create a Promise that resolves when image loads
    let promise = js_sys::Promise::new(&mut |resolve, reject| {
        let img_load = img_element.clone();
        let img_error = img_element.clone();
        
        let onload = Closure::once_into_js(move || {
            let _ = resolve.call0(&JsValue::UNDEFINED);
        });
        
        let onerror = Closure::once_into_js(move || {
            let _ = reject.call0(&JsValue::UNDEFINED);
        });
        
        img_load.set_onload(Some(onload.unchecked_ref()));
        img_error.set_onerror(Some(onerror.unchecked_ref()));
    });
    
    // Wait for image with 10 second timeout
    let timeout = js_sys::Promise::new(
        &mut |resolve, _| {
            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, 10000
            );
        }
    );
    
    // Race between load and timeout
    let result = js_sys::Promise::race(
        &js_sys::Array::from_iter([
            promise.unchecked_ref(),
            timeout.unchecked_ref(),
        ])
    );
    
    wasm_bindgen_futures::JsFuture::from(result).await.is_ok()
}