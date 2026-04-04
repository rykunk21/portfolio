use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlVideoElement;

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
    let duration = use_state(|| 0.0_f64);
    let video_top_offset = use_state(|| 0.0_f64); // Distance from top of document to video
    let is_ready = use_state(|| false);

    // Store closures for cleanup
    let scroll_closure_handle = use_state(|| None::<(Closure<dyn FnMut()>, web_sys::Window)>);
    let interval_handle = use_state(|| None::<i32>);

    // Effect for setting up video loading and scroll handling
    use_effect_with(video_ref.clone(), {
        let video_ref = video_ref.clone();
        let duration = duration.clone();
        let video_top_offset = video_top_offset.clone();
        let is_ready = is_ready.clone();
        let scroll_dist = props.scroll_distance as f64;
        let scroll_closure_handle = scroll_closure_handle.clone();
        let interval_handle = interval_handle.clone();

        move |_| {
            let window = web_sys::window().expect("no window");
            let window_for_interval = window.clone();

            // Set up interval to poll for video metadata loaded
            let video_ref_poll = video_ref.clone();
            let duration_poll = duration.clone();
            let is_ready_poll = is_ready.clone();
            let video_top_offset_poll = video_top_offset.clone();
            let window_for_poll = window.clone();

            let poll_closure = Closure::wrap(Box::new(move || {
                if let Some(video) = video_ref_poll.cast::<HtmlVideoElement>() {
                    let d = video.duration();
                    if d > 0.0 && !*is_ready_poll {
                        // Video metadata loaded - get initial position and pause
                        duration_poll.set(d);

                        // Calculate video's position relative to document
                        let rect = video.get_bounding_client_rect();
                        if let Ok(scroll_y) = window_for_poll.scroll_y() {
                            video_top_offset_poll.set(rect.top() + scroll_y);
                        }

                        let _ = video.pause();
                        is_ready_poll.set(true);
                    }
                }
            }) as Box<dyn FnMut()>);

            let interval_id = window_for_interval
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    poll_closure.as_ref().unchecked_ref(),
                    100,
                )
                .ok();

            interval_handle.set(interval_id);
            poll_closure.forget(); // Keep alive for interval

            // Set up scroll handler
            let video_ref_scroll = video_ref.clone();
            let duration_scroll = *duration;
            let video_top_scroll = *video_top_offset;
            let is_ready_scroll = *is_ready;

            let scroll_closure = Closure::wrap(Box::new(move || {
                if !is_ready_scroll {
                    // Video not ready yet, try to calculate position anyway
                    if let Some(video) = video_ref_scroll.cast::<HtmlVideoElement>() {
                        let rect = video.get_bounding_client_rect();
                        if let Some(window) = web_sys::window() {
                            if let Ok(scroll_y) = window.scroll_y() {
                                let _ = scroll_y + rect.top(); // Update if needed
                            }
                        }
                    }
                    return;
                }

                if let (Some(window), Some(video)) = (
                    web_sys::window(),
                    video_ref_scroll.cast::<HtmlVideoElement>(),
                ) {
                    if let Ok(scroll_y) = window.scroll_y() {
                        // Calculate how far video has moved from initial position
                        let rect = video.get_bounding_client_rect();
                        let current_video_top = scroll_y + rect.top();

                        // Use the stored initial offset if available, otherwise fallback
                        let video_anchor = if video_top_scroll > 0.0 {
                            video_top_scroll
                        } else {
                            current_video_top
                        };

                        // relative_scroll: how far we've scrolled past the video's starting position
                        let relative_scroll = scroll_y - video_anchor;
                        let progress = (relative_scroll / scroll_dist).clamp(0.0, 1.0);
                        let target = progress * duration_scroll;

                        let _ = video.set_current_time(target);
                    }
                }
            }) as Box<dyn FnMut()>);

            let _ = window.add_event_listener_with_callback(
                "scroll",
                scroll_closure.as_ref().unchecked_ref(),
            );

            scroll_closure_handle.set(Some((scroll_closure, window.clone())));

            // Initial call
            if let Ok(scroll_y) = window.scroll_y() {
                if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                    let rect = video.get_bounding_client_rect();
                    let current_top = scroll_y + rect.top();
                    video_top_offset.set(current_top);
                }
            }

            // Cleanup function
            move || {
                if let Some(id) = interval_id {
                    window.clear_interval_with_handle(id);
                }
                if let Some((closure, w)) = (*scroll_closure_handle).as_ref() {
                    let _ = w.remove_event_listener_with_callback(
                        "scroll",
                        closure.as_ref().unchecked_ref::<js_sys::Function>(),
                    );
                }
            }
        }
    });

    // Re-sync when video becomes ready (duration changes)
    use_effect_with(*is_ready, {
        let video_ref = video_ref.clone();
        let video_top_offset = video_top_offset.clone();

        move |ready| {
            if *ready {
                // Get accurate position now that video is rendered
                if let Some(window) = web_sys::window() {
                    if let Ok(scroll_y) = window.scroll_y() {
                        if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                            let rect = video.get_bounding_client_rect();
                            video_top_offset.set(scroll_y + rect.top());
                        }
                    }
                }
            }
            || {}
        }
    });

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
