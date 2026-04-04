use yew::prelude::*;
use crate::components::ui_depth::{UIDepth, FloatingElement};
use crate::components::sprite_animation::SpriteAnimation;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="relative min-h-screen overflow-visible text-surface-50 flex items-center"
        >
            // ===== FLOATING ELEMENTS (around the edges) =====
            
            // Far back: subtle glow blobs
            <FloatingElement depth={0.2} top_pct={10} left_pct={5} width={200} height={200}
            >
                <div class="w-full h-full rounded-full bg-gradient-to-br from-highlight-500/10 to-transparent blur-3xl" />
            </FloatingElement>
            
            <FloatingElement depth={0.15} top_pct={60} left_pct={85} width={250} height={250}
            >
                <div class="w-full h-full rounded-full bg-gradient-to-tl from-orange-500/10 to-transparent blur-3xl" />
            </FloatingElement>
            
            // Mid depth: floating shapes
            <FloatingElement depth={0.4} top_pct={20} left_pct={90} width={80} height={80}
            >
                <div class="w-full h-full rounded-full border border-surface-600/30 backdrop-blur-sm" />
            </FloatingElement>
            
            <FloatingElement depth={0.35} top_pct={70} left_pct={8} width={60} height={60}
            >
                <div class="w-full h-full rotate-45 border border-highlight-500/20" />
            </FloatingElement>
            
            // Near: floating accent elements
            <FloatingElement depth={0.6} top_pct={15} left_pct={15} width={40} height={40}
            >
                <div class="w-full h-full rounded-lg bg-highlight-400/20 backdrop-blur-md" />
            </FloatingElement>
            
            <FloatingElement depth={0.7} top_pct={80} left_pct={75} width={120} height={8}
            >
                <div class="w-full h-full rounded-full bg-gradient-to-r from-transparent via-surface-400/30 to-transparent" />
            </FloatingElement>
            
            // ===== MAIN CONTENT (at depth=1.0, normal scroll) =====
            <div class="relative z-20 w-full max-w-7xl mx-auto px-6 py-24"
            >
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center"
                >
                    // Video side - scroll-scrubbed animation
                    <div class="order-1 lg:order-2 relative"
                    >
                        <UIDepth depth={1.0} shadow={0.3} z_offset={10}
                        >
                            <div class="relative aspect-square max-w-lg mx-auto rounded-2xl overflow-hidden bg-neutral-950"
                            >
                                <SpriteAnimation
                                    frame_path_template={"/media/anim/frame_{:04}.png"}
                                    frame_count={250}
                                    scroll_distance={800}
                                    class={classes!("w-full", "h-full")}
                                />
                            </div>
                        </UIDepth>
                    </div>
                    
                    // Text side
                    <div class="order-2 lg:order-1"
                    >
                        <UIDepth depth={1.0} shadow={0.2} z_offset={15}
                        >
                            <h1 class="text-5xl lg:text-6xl font-bold leading-tight mb-6"
                                 style="color: var(--color-surface-50);"
                            >
                                { "I design " }
                                <span style="color: var(--color-highlight-500);">{ "& build" }</span>
                                <br />
                                { "human-centered" }
                                <br />
                                { "software" }
                            </h1>
                        </UIDepth>
                        
                        <UIDepth depth={1.0} shadow={0.1} z_offset={12} y_offset={20}
                        >
                            <p class="text-lg mb-8 max-w-md"
                               style="color: var(--color-surface-400);"
                            >
                                { "From systems thinking to clean interfaces, I help teams turn complex ideas into reliable products." }
                            </p>
                        </UIDepth>
                        
                        <UIDepth depth={1.0} shadow={0.15} z_offset={14} y_offset={40}
                        >
                            <div class="flex flex-wrap gap-4"
                            >
                                <a href="#contact"
                                   class="px-8 py-3 rounded-full font-medium transition-all hover:scale-105"
                                   style="background: var(--color-highlight-500); color: var(--color-neutral-950);"
                                >
                                    { "Start a conversation" }
                                </a>
                                
                                <a href="#process"
                                   class="px-8 py-3 rounded-full font-medium border transition-all hover:scale-105"
                                   style="border-color: var(--color-surface-600); color: var(--color-surface-100);"
                                >
                                    { "View process" }
                                </a>
                            </div>
                        </UIDepth>
                    </div>
                </div>
            </div>
        </section>
    }
}
