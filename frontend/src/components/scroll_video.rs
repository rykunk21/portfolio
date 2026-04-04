use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlVideoElement, console};

/// ScrollVideo: Maps scroll position to video time
#[derive(Properties, PartialEq)]
pub struct ScrollVideoProps {
    pub src: String,
    #[prop_or(800)]
    pub scroll_distance: i32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ScrollVideo)]
pub fn scroll_video(props: &ScrollVideoProps) -> Html {
    let video_ref = use_node_ref();
    
    // Use refs for values that need to be read inside closures
    let duration = use_mut_ref(|| 0.0_f64);
    let video_top = use_mut_ref(|| 0.0_f64);
    let scroll_dist = props.scroll_distance as f64;

    // Effect: setup when video element becomes available
    use_effect_with(video_ref.clone(), {
        let video_ref = video_ref.clone();
        let duration = duration.clone();
        let video_top = video_top.clone();

        move |_| {
            let window = web_sys::window().expect("no window");
            
            // Only setup if video element exists
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                // Setup loadedmetadata handler
                let duration_clone = duration.clone();
                let video_clone2 = video.clone();
                let video_top_clone = video_top.clone();
                let window_for_loaded = window.clone();
                
                let loaded_closure = Closure::wrap(Box::new(move || {
                    let d = video_clone2.duration();
                    *duration_clone.borrow_mut() = d;
                    
                    // Calculate initial position
                    let rect = video_clone2.get_bounding_client_rect();
                    if let Ok(scroll_y) = window_for_loaded.scroll_y() {
                        *video_top_clone.borrow_mut() = scroll_y + rect.top();
                        let msg = format!("ScrollVideo: loaded, duration={:.2}s, video_top={:.0}px", d, scroll_y + rect.top());
                        console::log_1(&msg.into());
                    }
                    
                    // Show first frame explicitly
                    let _ = video_clone2.set_current_time(0.0);
                    
                    // Force a frame draw by playing and pausing
                    let video_for_draw = video_clone2.clone();
                    let _ = video_for_draw.play();
                    let pause_video = video_for_draw.clone();
                    let window = web_sys::window().unwrap();
                    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
                        Closure::once_into_js(move || {
                            let _ = pause_video.pause();
                            let _ = pause_video.set_current_time(0.0);
                        }).unchecked_ref(),
                        16
                    );
                }) as Box<dyn FnMut()>);

                video.set_onloadedmetadata(Some(loaded_closure.as_ref().unchecked_ref()));
                loaded_closure.forget();

                // Setup scroll handler
                let video_scroll = video.clone();
                let duration_scroll = duration.clone();
                let video_top_scroll = video_top.clone();
                
                let scroll_closure = Closure::wrap(Box::new(move || {
                    let Some(window) = web_sys::window() else { return };
                    let Ok(scroll_y) = window.scroll_y() else { return };
                    
                    let dur = *duration_scroll.borrow();
                    if dur <= 0.0 {
                        return;
                    }
                    
                    let anchor = *video_top_scroll.borrow();
                    if anchor <= 0.0 {
                        let rect = video_scroll.get_bounding_client_rect();
                        *video_top_scroll.borrow_mut() = scroll_y + rect.top();
                    }
                    
                    let anchor = *video_top_scroll.borrow();
                    let relative_scroll = scroll_y - anchor;
                    let progress = (relative_scroll / scroll_dist).clamp(0.0, 1.0);
                    let target = progress * dur;
                    
                    let current = video_scroll.current_time();
                    if (target - current).abs() > 0.033 {
                        let _ = video_scroll.set_current_time(target);
                    }
                }) as Box<dyn FnMut()>);

                let _ = window.add_event_listener_with_callback(
                    "scroll",
                    scroll_closure.as_ref().unchecked_ref(),
                );
                scroll_closure.forget();

                // Handle resize
                let video_resize = video.clone();
                let video_top_resize = video_top.clone();
                let resize_closure = Closure::wrap(Box::new(move || {
                    let Some(window) = web_sys::window() else { return };
                    let Ok(scroll_y) = window.scroll_y() else { return };
                    let rect = video_resize.get_bounding_client_rect();
                    *video_top_resize.borrow_mut() = scroll_y + rect.top();
                }) as Box<dyn FnMut()>);
                
                let _ = window.add_event_listener_with_callback(
                    "resize",
                    resize_closure.as_ref().unchecked_ref(),
                );
                resize_closure.forget();
            } else {
                console::log_1(&"ScrollVideo: video element not found, retrying".into());
            }
            
            || ()
        }
    });

    html! {
        <video
            ref={video_ref}
            class={props.class.clone()}
            src={props.src.clone()}
            muted={true}
            playsinline={true}
            preload="metadata"
            style="width: 100%; height: auto; object-fit: contain; display: block; background: transparent;"
        />
    }
}
