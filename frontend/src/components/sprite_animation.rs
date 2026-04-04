use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlImageElement;

/// SpriteAnimation: Scroll-scrubbed PNG sequence
#[derive(Properties, PartialEq)]
pub struct SpriteAnimationProps {
    /// Base path to frames (e.g., "/media/anim/frame_{:04}.png")
    pub frame_path_template: String,
    pub frame_count: usize,
    /// Pixels to scroll for full animation
    #[prop_or(800)]
    pub scroll_distance: i32,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SpriteAnimation)]
pub fn sprite_animation(props: &SpriteAnimationProps) -> Html {
    let current_frame = use_state(|| 0_usize);
    let container_ref = use_node_ref();

    // Scroll handler
    {
        let current_frame = current_frame.clone();
        let container_ref = container_ref.clone();
        let frame_count = props.frame_count;
        let scroll_distance = props.scroll_distance as f64;

        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                if let Some(window) = web_sys::window() {
                    if let Ok(scroll_y) = window.scroll_y() {
                        if let Some(container) = container_ref.cast::<web_sys::HtmlElement>() {
                            let rect = container.get_bounding_client_rect();
                            let container_top = scroll_y + rect.top();
                            let relative_scroll = scroll_y - container_top;
                            
                            let progress = (relative_scroll / scroll_distance).clamp(0.0, 1.0);
                            let frame_idx = ((progress * (frame_count - 1) as f64) as usize)
                                .min(frame_count - 1);
                            
                            current_frame.set(frame_idx);
                        }
                    }
                }
            }) as Box<dyn FnMut()>);

            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
                // Initial call
                closure.as_ref().unchecked_ref::<js_sys::Function>().call0(&JsValue::NULL).ok();
            }
            closure.forget();
            
            || {}
        });
    }

    // Build frame path
    let frame_path = props.frame_path_template.replace("{:04}", &format!("{:04}", *current_frame));

    html! {
        <div ref={container_ref} class={props.class.clone()}>
            <img 
                src={frame_path}
                alt=""
                style="width: 100%; height: 100%; object-fit: contain;"
            />
        </div>
    }
}

/// Preloads frames in background
pub fn preload_frames(base_path: &str, count: usize) {
    for i in 0..count {
        let path = base_path.replace("{:04}", &format!("{:04}", i));
        let img = HtmlImageElement::new().unwrap();
        img.set_src(&path);
    }
}
