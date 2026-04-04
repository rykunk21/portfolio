use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlVideoElement, window};

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
    let duration = use_state(|| 0.0_f64);
    let start_y = use_state(|| 0.0_f64);
    let scroll_dist = props.scroll_distance as f64;

    // Setup: get video element and attach handlers
    use_effect_with(video_ref.clone(), {
        let video_ref = video_ref.clone();
        let duration = duration.clone();
        let start_y = start_y.clone();
        
        move |_| {
            let Some(video) = video_ref.cast::<HtmlVideoElement>() else {
                return;
            };
            let win = window().expect("no window");
            
            // When metadata loads: store duration and starting position
            let loaded = {
                let dur_setter = duration.clone();
                let y_setter = start_y.clone();
                let vid = video.clone();
                Closure::<dyn FnMut()>::new(move || {
                    let d = vid.duration();
                    if d > 0.0 {
                        dur_setter.set(d);
                        let rect = vid.get_bounding_client_rect();
                        let scroll = window().unwrap().scroll_y().unwrap_or(0.0);
                        y_setter.set(scroll + rect.top());
                    }
                })
            };
            video.set_onloadedmetadata(Some(loaded.as_ref().unchecked_ref()));
            loaded.forget();
            
            // On scroll: update video time based on scroll position
            // Re-read duration/start_y inside the closure to get fresh values
            let vid = video.clone();
            let dur = duration.clone();
            let sy = start_y.clone();
            let on_scroll = Closure::<dyn FnMut()>::new(move || {
                let d = *dur;
                let y = *sy;
                if d <= 0.0 || y <= 0.0 { return; }
                let scroll = window().unwrap().scroll_y().unwrap_or(0.0);
                let progress = ((scroll - y) / scroll_dist).clamp(0.0, 1.0);
                let target = progress * d;
                let _ = vid.set_current_time(target);
            });
            win.add_event_listener_with_callback("scroll", on_scroll.as_ref().unchecked_ref()).ok();
            on_scroll.forget();
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
            style="width: 100%; height: auto; display: block;"
        />
    }
}
