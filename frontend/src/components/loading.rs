use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;

/// A unique scramble-to-reveal "hello" loader
/// Letters decode from random characters with a terminal-like aesthetic
#[derive(Properties, PartialEq)]
pub struct LoadingScreenProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoadingScreen)]
pub fn loading_screen(props: &LoadingScreenProps) -> Html {
    let is_loading = use_state(|| true);
    
    // Scramble animation state
    let display_text = use_state(|| String::new());
    let show_cursor = use_state(|| true);
    let is_revealed = use_state(|| false);
    
    use_effect_with((), {
        let is_loading = is_loading.clone();
        let display_text = display_text.clone();
        let show_cursor = show_cursor.clone();
        let is_revealed = is_revealed.clone();
        
        move |_| {
            spawn_local(async move {
                // Target text
                let target = "hello";
                let chars: Vec<char> = target.chars().collect();
                
                // Start with scrambled random characters
                let mut current: Vec<char> = chars.iter().map(|_| random_char()).collect();
                display_text.set(current.iter().collect());
                
                // Scramble animation: each letter cycles through random chars before locking in
                for (i, target_char) in chars.iter().enumerate() {
                    let mut cycles = 0;
                    let max_cycles = 8 + (i * 2); // Each letter takes longer to resolve
                    
                    while cycles < max_cycles {
                        // Update this position with random char
                        current[i] = random_char();
                        display_text.set(current.iter().collect());
                        
                        // Faster scramble at first, slower as we approach target
                        let delay: i32 = 50 + (cycles * 20) as i32;
                        sleep_ms(delay.min(150)).await;
                        cycles += 1;
                    }
                    
                    // Lock in the correct character
                    current[i] = *target_char;
                    display_text.set(current.iter().collect());
                    
                    // Brief pause between letters
                    sleep_ms(100).await;
                }
                
                // Pause with completed text
                sleep_ms(400).await;
                
                // Hide cursor
                show_cursor.set(false);
                
                // Mark as revealed for exit animation
                is_revealed.set(true);
                
                // Wait for exit animation to start
                sleep_ms(600).await;
                
                // Hide loading screen
                is_loading.set(false);
            });
            || {}
        }
    });
    
    let cursor_class = if *show_cursor { "opacity-100" } else { "opacity-0" };
    let exit_class = if *is_revealed { "hello-exit" } else { "" };
    
    html! {
        <>
            if *is_loading {
                <div 
                    class="fixed inset-0 z-50 flex items-center justify-center bg-neutral-950"
                    style="transition: opacity 0.5s ease-out;"
                >
                    <div 
                        class={classes!("hello-container", exit_class)}
                        style="font-family: 'JetBrains Mono', 'Fira Code', monospace;"
                    >
                        <span 
                            class="text-6xl md:text-8xl font-light tracking-wider"
                            style="color: #fa8805; text-shadow: 0 0 40px rgba(250, 136, 5, 0.3);"
                        >
                            { display_text.chars().enumerate().map(|(i, c)| {
                                html! {
                                    <span 
                                        key={i}
                                        class="inline-block hello-letter"
                                        style={format!(
                                            "animation-delay: {}ms;",
                                            i * 50
                                        )}
                                    >
                                        { c }
                                    </span>
                                }
                            }).collect::<Html>() }
                        </span>
                        
                        // Blinking cursor
                        <span 
                            class={classes!("ml-1", "text-6xl", "md:text-8xl", cursor_class)}
                            style="color: #fa8805; animation: blink 1s step-end infinite;"
                        >
                            { "_" }
                        </span>
                    </div>
                    
                    // CSS animations
                    <style>
                        {"@keyframes blink {
                            0%, 100% { opacity: 1; }
                            50% { opacity: 0; }
                        }
                        
                        @keyframes letter-settle {
                            0% { transform: translateY(-5px) scale(1.1); filter: blur(2px); }
                            50% { transform: translateY(2px) scale(0.95); }
                            100% { transform: translateY(0) scale(1); filter: blur(0); }
                        }
                        
                        .hello-letter {
                            display: inline-block;
                            animation: letter-settle 0.4s ease-out forwards;
                        }
                        
                        .hello-container {
                            transition: transform 0.6s cubic-bezier(0.4, 0, 0.2, 1), 
                                        opacity 0.6s cubic-bezier(0.4, 0, 0.2, 1);
                        }
                        
                        .hello-exit {
                            transform: translateY(-30px) scale(0.9);
                            opacity: 0;
                        }"}
                    </style>
                </div>
            }
            
            // Content layer
            <div 
                class={if *is_loading { "content-hidden" } else { "content-reveal" }}
            >
                { props.children.clone() }
            </div>
            
            <style>
                {".content-hidden {
                    opacity: 0;
                    transform: scale(0.98);
                    filter: blur(8px);
                }
                
                .content-reveal {
                    opacity: 1;
                    transform: scale(1);
                    filter: blur(0);
                    transition: opacity 0.8s cubic-bezier(0.4, 0, 0.2, 1),
                                transform 0.8s cubic-bezier(0.4, 0, 0.2, 1),
                                filter 0.8s ease-out;
                }"}
            </style>
        </>
    }
}

// Generate random ASCII character (letters and numbers)
fn random_char() -> char {
    use js_sys::Math::random;
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*";
    let idx = (random() * chars.len() as f64) as usize;
    chars.chars().nth(idx).unwrap_or('?')
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
