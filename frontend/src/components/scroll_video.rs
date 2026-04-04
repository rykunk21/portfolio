use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

/// ScrollVideo: Scrub through video based on scroll position
#[derive(Properties, PartialEq)]
pub struct ScrollVideoProps {
    pub src: String,
    #[prop_or_default]
    pub poster: Option<String>,
    /// Pixels to scroll for full video duration
    #[prop_or(800)]
    pub scroll_distance: i32,
    /// Element selector for scroll tracking (default: self)
    #[prop_or_default]
    pub track_selector: Option<String>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ScrollVideo)]
pub fn scroll_video(props: &ScrollVideoProps) -> Html {
    let video_ref = use_node_ref();
    let container_ref = use_node_ref();
    let duration = use_state(|| 0.0_f64);
    let container_top = use_state(|| 0.0_f64);

    // Update container position and duration
    {
        let duration = duration.clone();
        let container_top = container_top.clone();
        let container_ref = container_ref.clone();
        let video_ref = video_ref.clone();
        
        use_effect_with((), move |_| {
            let update_position = {
                let container_top = container_top.clone();
                let container_ref = container_ref.clone();
                move || {
                    if let Some(window) = web_sys::window() {
                        if let Some(element) = container_ref.cast::<web_sys::HtmlElement>() {
                            let rect = element.get_bounding_client_rect();
                            let scroll_y = window.scroll_y().unwrap_or(0.0);
                            container_top.set(scroll_y + rect.top());
                        }
                    }
                }
            };

            // Initial position
            update_position();
            
            // Update on resize
            let window = web_sys::window().unwrap();
            let closure = Closure::wrap(Box::new(move || update_position()) as Box<dyn FnMut()>);
            let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
            closure.forget();

            // Check duration once video loads
            let check_duration = {
                let duration = duration.clone();
                let video_ref = video_ref.clone();
                move || {
                    if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                        let d = video.duration();
                        if d > 0.0 {
                            duration.set(d);
                            let _ = video.pause();
                        }
                    }
                }
            };
            
            // Check every 100ms until loaded
            let interval_closure = Closure::wrap(Box::new(check_duration) as Box<dyn FnMut()>);
            let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
                interval_closure.as_ref().unchecked_ref(),
                100
            ).unwrap_or(0);
            interval_closure.forget();
            
            move || {
                window.clear_interval_with_handle(interval_id);
            }
        });
    }

    // Scroll handler
    {
        let video_ref = video_ref.clone();
        let duration_state = duration.clone();
        let container_top_state = container_top.clone();
        let scroll_distance = props.scroll_distance as f64;

        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                let duration = *duration_state;
                let container_top = *container_top_state;
                
                if duration <= 0.0 {
                    return;
                }
                
                if let Some(window) = web_sys::window() {
                    if let Ok(scroll_y) = window.scroll_y() {
                        if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                            let relative_scroll = scroll_y - container_top;
                            let progress = (relative_scroll / scroll_distance).clamp(0.0, 1.0);
                            let target_time = progress * duration;
                            
                            let _ = video.set_current_time(target_time);
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

    html! {
        <div ref={container_ref} class={props.class.clone()} style="width: 100%; height: 100%;"
        >
            <video
                ref={video_ref}
                src={props.src.clone()}
                poster={props.poster.clone()}
                muted={true}
                playsinline={true}
                preload="auto"
                style="width: 100%; height: 100%; object-fit: cover; display: block;"
            />
        </div>
    }
}
