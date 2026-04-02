use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// Loading screen that displays while critical assets preload
#[derive(Properties, PartialEq)]
pub struct LoadingScreenProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoadingScreen)]
pub fn loading_screen(props: &LoadingScreenProps) -> Html {
    let is_loading = use_state(|| true);
    let progress = use_state(|| 0);
    
    use_effect_with((), {
        let is_loading = is_loading.clone();
        let progress = progress.clone();
        move |_| {
            spawn_local(async move {
                let critical_images = vec![
                    "media/portfolio.png",
                    "media/headshot.jpg",  
                    "media/logo.svg",
                ];
                
                let total = critical_images.len();
                for (i, src) in critical_images.iter().enumerate() {
                    preload_single_image(src).await;
                    progress.set(((i + 1) * 100) / total);
                }
                
                progress.set(100);
                
                // Small delay for visual completion
                sleep_ms(500).await;
                is_loading.set(false);
            });
            || {}
        }
    });
    
    html! {
        <>
            if *is_loading {
                <div class="fixed inset-0 z-50 flex flex-col items-center justify-center"
                    style="background: linear-gradient(135deg, #0a0a0f 0%, #18181b 50%, #231810 100%);"
                >
                    <div class="text-center mb-8">
                        <h1 class="text-4xl font-light tracking-wide mb-2" style="color: #f1f2f4;"
                        >{ "Ryan Kunkel" }</h1>
                        <p style="color: #8f95a3;">{ "Loading experience..." }</p>
                    </div>
                    
                    <div class="relative w-16 h-16 mb-8">
                        <div class="absolute inset-0 rounded-full border-2"
                            style="border-color: rgba(250, 136, 5, 0.2);"
                        ></div>
                        <div class="absolute inset-0 rounded-full border-2 border-t-orange-500 animate-spin"
                            style="border-top-color: #fa8805;"
                        ></div>
                    </div>
                    
                    <div class="w-48 h-0.5 mb-4 rounded overflow-hidden"
                        style="background: rgba(255, 255, 255, 0.1);"
                    >
                        <div class="h-full transition-all duration-300"
                            style={format!("background: #fa8805; width: {}%", *progress)}
                        ></div>
                    </div>
                    
                    <p style="color: #8f95a3; font-size: 0.875rem;"
                    >{ format!("{}%", *progress) }</p>
                </div>
            }
            
            <div 
                style={if *is_loading { "visibility: hidden; position: absolute;" } else { "" }}
            >
                { props.children.clone() }
            </div>
        </>
    }
}

async fn preload_single_image(src: &str) {
    let window = match web_sys::window() {
        Some(w) => w,
        None => return,
    };
    
    let document = match window.document() {
        Some(d) => d,
        None => return,
    };
    
    let img = match document.create_element("img") {
        Ok(i) => i,
        Err(_) => return,
    };
    
    let img_element = match img.dyn_into::<web_sys::HtmlImageElement>() {
        Ok(el) => el,
        Err(_) => return,
    };
    
    // FIX: Set CORS to anonymous
    let _ = img_element.set_attribute("crossorigin", "anonymous");
    img_element.set_src(src);
    
    // Create promise that resolves on load or error (error handling prevents CORS hanging)
    let img_load = img_element.clone();
    let img_error = img_element.clone();
    
    let promise = js_sys::Promise::new(
        &mut |resolve, _| {
            let onload = Closure::once_into_js({
                let resolve = resolve.clone();
                move || { let _ = resolve.call0(&JsValue::UNDEFINED); }
            });
            
            let onerror = Closure::once_into_js({
                let resolve = resolve.clone();
                move || { let _ = resolve.call0(&JsValue::UNDEFINED); } // FIX: resolve on error too
            });
            
            img_load.set_onload(Some(onload.unchecked_ref()));
            img_error.set_onerror(Some(onerror.unchecked_ref()));
        }
    );
    
    // Wait max 3 seconds
    let timeout = js_sys::Promise::new(
        &mut |resolve, _| {
            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, 3000
            );
        }
    );
    
    // Race between image and timeout
    let race = js_sys::Promise::race(
        &js_sys::Array::from_iter([
            promise.unchecked_ref::<JsValue>(),
            timeout.unchecked_ref::<JsValue>(),
        ])
    );
    
    wasm_bindgen_futures::JsFuture::from(race).await.ok();
}

async fn sleep_ms(ms: i32) {
    let promise = js_sys::Promise::new(
        &mut |resolve, _| {
            let window = web_sys::window().unwrap();
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                &resolve, ms
            );
        }
    );
    wasm_bindgen_futures::JsFuture::from(promise).await.ok();
}
