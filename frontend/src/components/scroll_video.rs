use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlVideoElement;
use std::cell::RefCell;
use std::rc::Rc;

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
    // Use Rc<RefCell> so closure can read current values
    let duration = Rc::new(RefCell::new(0.0_f64));
    let video_top = Rc::new(RefCell::new(0.0_f64));
    let is_ready = Rc::new(RefCell::new(false));
    let scroll_dist = props.scroll_distance as f64;

    {
        let video_ref = video_ref.clone();
        let duration = duration.clone();
        let video_top = video_top.clone();
        let is_ready = is_ready.clone();

        use_effect_with((), move |_| {
            let window = web_sys::window().unwrap();

            // Poll until video loaded
            let poll_video = {
                let video_ref = video_ref.clone();
                let duration = duration.clone();
                let is_ready = is_ready.clone();
                move || {
                    if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                        let d = video.duration();
                        if d > 0.0 {
                            *duration.borrow_mut() = d;
                            *is_ready.borrow_mut() = true;
                            let _ = video.pause();
                            return true;
                        }
                    }
                    false
                }
            };

            let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
                Closure::wrap(Box::new(move || { poll_video(); }) as Box<dyn FnMut()>)
                    .as_ref().unchecked_ref(),
                100,
            ).ok();

            // Scroll handler - reads fresh values each time
            let scroll_handler = {
                let video_ref = video_ref.clone();
                let duration = duration.clone();
                let video_top = video_top.clone();
                let is_ready = is_ready.clone();
                
                move || {
                    if !*is_ready.borrow() {
                        // Try to update position anyway
                        if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                            let rect = video.get_bounding_client_rect();
                            if let Some(window) = web_sys::window() {
                                if let Ok(scroll_y) = window.scroll_y() {
                                    *video_top.borrow_mut() = scroll_y + rect.top();
                                }
                            }
                        }
                        return;
                    }

                    if let (Some(window), Some(video)) = (
                        web_sys::window(),
                        video_ref.cast::<HtmlVideoElement>(),
                    ) {
                        if let Ok(scroll_y) = window.scroll_y() {
                            let rect = video.get_bounding_client_rect();
                            *video_top.borrow_mut() = scroll_y + rect.top();
                            
                            let relative_scroll = scroll_y - *video_top.borrow();
                            let progress = (relative_scroll / scroll_dist).clamp(0.0, 1.0);
                            let target = progress * *duration.borrow();
                            
                            let _ = video.set_current_time(target);
                        }
                    }
                }
            };

            let closure = Closure::wrap(Box::new(scroll_handler) as Box<dyn FnMut()>);
            let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            
            // Also call once for initial position
            closure.as_ref().unchecked_ref::<js_sys::Function>().call0(&JsValue::NULL).ok();
            
            closure.forget();

            move || {
                if let Some(id) = interval_id {
                    window.clear_interval_with_handle(id);
                }
            }
        });
    }

    html! {
        <video
            ref={video_ref}
            class={props.class.clone()}
            src={props.src.clone()}
            muted={true}
            playsinline={true}
            preload="auto"
            style="width: 100%; height: 100%; object-fit: contain;"
        />
    }
}
