use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

/// Simple scroll-scrubbed video
/// Maps scroll position to video time
#[derive(Properties, PartialEq)]
pub struct ScrollVideoProps {
    pub src: String,
    /// Pixels to scroll for full video play
    #[prop_or(800)]
    pub scroll_distance: i32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ScrollVideo)]
pub fn scroll_video(props: &ScrollVideoProps) -> Html {
    let video_ref = use_node_ref();
    let is_ready = use_state(|| false);

    // Setup scroll listener once video is loaded
    {
        let video_ref = video_ref.clone();
        let is_ready = is_ready.clone();
        let scroll_distance = props.scroll_distance as f64;

        use_effect_with((), move |_| {
            // Wait for video to have duration
            let check_ready = {
                let video_ref = video_ref.clone();
                let is_ready = is_ready.clone();
                move || {
                    if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                        if video.duration() > 0.0 {
                            let _ = video.pause();
                            is_ready.set(true);
                            return true;
                        }
                    }
                    false
                }
            };

            // Poll until ready
            let window = web_sys::window().unwrap();
            let poll_closure = Closure::wrap(Box::new(move || {
                if check_ready() {
                    // Stop polling - video is ready
                }
            }) as Box<dyn FnMut()>);

            let interval_id = window.set_interval_with_callback_and_timeout_and_arguments_0(
                poll_closure.as_ref().unchecked_ref(),
                100,
            ).ok();
            poll_closure.forget();

            // Scroll handler
            let scroll_closure = Closure::wrap(Box::new(move || {
                if !*is_ready {
                    return;
                }

                if let (Some(window), Some(video)) = (
                    web_sys::window(),
                    video_ref.cast::<HtmlVideoElement>(),
                ) {
                    if let (Ok(scroll_y), Ok(rect)) = (
                        window.scroll_y(),
                        video.get_bounding_client_rect().dyn_into::<web_sys::DomRect>(),
                    ) {
                        let video_top = scroll_y + rect.top();
                        let relative_scroll = scroll_y - video_top;
                        let duration = video.duration();

                        if duration > 0.0 {
                            let progress = (relative_scroll / scroll_distance)
                                .clamp(0.0, 1.0);
                            let _ = video.set_current_time(progress * duration);
                        }
                    }
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback(
                    "scroll",
                    scroll_closure.as_ref().unchecked_ref(),
                );
            }
            scroll_closure.forget();

            // Cleanup interval
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
            style="width: 100%; height: 100%; object-fit: cover;"
        />
    }
}
