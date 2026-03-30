/// CSS for mobile scroll snap - short form media-like experience
/// Applied globally for mobile breakpoint only
pub const SCROLL_SNAP_CSS: &str = r#"
/* Mobile scroll snap - short form media experience */
@media (max-width: 767px) {
    html {
        scroll-behavior: smooth;
    }
    
    body {
        overflow-y: scroll;
        scroll-snap-type: y mandatory;
        scroll-padding-top: 0;
    }
    
    /* All major sections snap to viewport */
    section {
        scroll-snap-align: start;
        scroll-snap-stop: always;
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
    }
    
    /* Hero section full viewport */
    #hero,
    section:first-of-type {
        min-height: 100vh;
    }
    
    /* Smooth momentum on supporting browsers */
    @supports (scroll-snap-type: y mandatory) {
        body {
            overscroll-behavior-y: contain;
        }
    }
}

/* Reduced motion - disable snap for accessibility */
@media (prefers-reduced-motion: reduce) {
    html {
        scroll-behavior: auto;
    }
    
    body {
        scroll-snap-type: none;
    }
    
    section {
        scroll-snap-align: none;
    }
}
"#;

/// Initialize scroll snap by injecting CSS
pub fn init_scroll_snap() {
    use web_sys::{window, Document};
    use wasm_bindgen::JsCast;
    
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            if let Ok(style_element) = document.create_element("style") {
                style_element.set_text_content(Some(SCROLL_SNAP_CSS));
                if let Some(head) = document.head() {
                    let _ = head.append_child(&style_element);
                }
            }
        }
    }
}
