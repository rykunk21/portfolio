use yew::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlVideoElement;

/// ScrollVideo: Scrub through video based on scroll position
/// Converts scroll progress to video currentTime for smooth animation
#[derive(Properties, PartialEq)]
pub struct ScrollVideoProps {
    pub src: String,
    #[prop_or_default]
    pub poster: Option<String>,
    /// Total scroll distance (in pixels) to play full video
    #[prop_or(1000)]
    pub scroll_distance: i32,
    /// Optional: start offset from section top (px)
    #[prop_or(0)]
    pub start_offset: i32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ScrollVideo)]
pub fn scroll_video(props: &ScrollVideoProps) -> Html {
    let video_ref = use_node_ref();
    let scroll_y = use_state(|| 0.0_f64);
    let section_top = use_state(|| 0.0_f64);
    let is_loaded = use_state(|| false);

    // Get section position on mount
    {
        let section_top = section_top.clone();
        let video_ref = video_ref.clone();
        use_effect_with((), move |_| {
            if let Some(window) = web_sys::window() {
                if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                    if let Some(rect) = video.get_bounding_client_rect().dyn_ref::<web_sys::DomRect>() {
                        let scroll = window.scroll_y().unwrap_or(0.0);
                        section_top.set(scroll + rect.top());
                    }
                }
            }
            || {}
        });
    }

    // Update video time based on scroll
    {
        let video_ref = video_ref.clone();
        let section_top_val = *section_top;
        let scroll_distance = props.scroll_distance as f64;
        let start_offset = props.start_offset as f64;
        let is_loaded_val = *is_loaded;

        use_effect_with(scroll_y.clone(), move |y| {
            if !is_loaded_val {
                return;
            }
            
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                let duration = video.duration();
                if duration > 0.0 {
                    let relative_scroll = **y - section_top_val + start_offset;
                    let raw_progress = relative_scroll / scroll_distance;
                    let progress = raw_progress.clamp(0.0, 1.0);
                    
                    let target_time = progress * duration;
                    let _ = video.set_current_time(target_time);
                }
            }
        });
    }

    // Scroll listener
    {
        let scroll_y = scroll_y.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if let Some(w) = web_sys::window() {
                    if let Ok(y) = w.scroll_y() {
                        scroll_y.set(y);
                    }
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
            closure.forget();
        });
    }

    let on_loaded = {
        let is_loaded = is_loaded.clone();
        let video_ref = video_ref.clone();
        Callback::from(move |_| {
            is_loaded.set(true);
            if let Some(video) = video_ref.cast::<HtmlVideoElement>() {
                let _ = video.pause();
                let _ = video.set_current_time(0.0);
            }
        })
    };

    html! {
        <video
            ref={video_ref}
            class={props.class.clone()}
            src={props.src.clone()}
            poster={props.poster.clone()}
            muted={true}
            playsinline={true}
            preload="auto"
            onloadedmetadata={on_loaded}
            style="object-fit: cover;"
        />
    }
}

/// Section wrapper with scroll-controlled video background
#[derive(Properties, PartialEq)]
pub struct ScrollVideoSectionProps {
    #[prop_or_default]
    pub children: Children,
    pub video_src: String,
    #[prop_or(2000)]
    pub scroll_distance: i32,
}

#[function_component(ScrollVideoSection)]
pub fn scroll_video_section(props: &ScrollVideoSectionProps) -> Html {
    html! {
        <section class="relative min-h-screen overflow-visible">
            <div class="fixed inset-0 -z-10">
                <ScrollVideo
                    src={props.video_src.clone()}
                    scroll_distance={props.scroll_distance}
                    class={classes!(
                        "w-full",
                        "h-full",
                        "object-cover"
                    )}
                />
            </div>
            <div class="relative z-10 pt-screen">
                {props.children.clone()}
            </div>
        </section>
    }
}
