use yew::prelude::*;
use crate::components::ui_depth::{UIDepth, LayerGroup};

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="relative overflow-x-hidden text-surface-50 min-h-screen flex flex-col items-center justify-center">
            
            // Background gradient (subtle, behind content)
            <div class="absolute inset-0"
                 style="background: radial-gradient(ellipse 80% 50% at 50% 30%, rgba(20,30,50,0.3) 0%, transparent 70%);"
            />
            
            <LayerGroup base_depth={0.8}>
                // Hero title - front layer
                <div class="text-center mb-12 relative z-10">
                    <UIDepth depth={0.7} shadow={0.5} z_offset={10}>
                        <h1 class="text-5xl md:text-7xl font-light tracking-tight mb-4"
                             style="color: var(--color-surface-50); text-shadow: 0 2px 20px rgba(0,0,0,0.3);"
                        >
                            { "Portfolio" }
                        </h1>
                    </UIDepth>
                    
                    <UIDepth depth={0.75} shadow={0.4} z_offset={8}>
                        <p class="text-xl md:text-2xl font-light opacity-80 max-w-2xl mx-auto"
                           style="color: var(--color-surface-300);"
                        >
                            { "Showcasing creative work with depth" }
                        </p>
                    </UIDepth>
                </div>
                
                // Hero image - middle layer (more depth)
                <div class="relative w-full max-w-4xl mx-auto mb-12 px-4">
                    <UIDepth depth={0.6} shadow={0.7} z_offset={5}>
                        <div class="relative aspect-video rounded-lg overflow-hidden"
                             style="transform: perspective(1000px) rotateY(-2deg) rotateX(2deg);"
                        >
                            <img
                                src="media/portfolio.png"
                                alt="Working session"
                                class="w-full h-full object-contain"
                            />
                        </div>
                    </UIDepth>
                </div>
                
                // CTA Buttons - shallow layer, floating above
                <div class="flex flex-wrap gap-4 justify-center relative z-10">
                    <UIDepth depth={0.85} shadow={0.3} z_offset={15}>
                        <a href="#services" 
                           class="inline-flex items-center px-8 py-3 rounded-full font-medium transition-all duration-300 hover:scale-105"
                           style="background: var(--color-highlight-500); color: var(--color-neutral-950);"
                        >
                            { "View Services" }
                        </a>
                    </UIDepth>
                    
                    <UIDepth depth={0.9} shadow={0.2} z_offset={12}>
                        <a href="#contact"
                           class="inline-flex items-center px-8 py-3 rounded-full font-medium transition-all duration-300 hover:scale-105 border-2"
                           style="border-color: var(--color-surface-600); color: var(--color-surface-50);"
                        >
                            { "Get in Touch" }
                        </a>
                    </UIDepth>
                </div>
            </LayerGroup>
            
            // Scroll indicator
            <div class="absolute bottom-8 left-1/2 -translate-x-1/2 animate-bounce">
                <div class="w-6 h-10 rounded-full border-2 border-surface-600 flex justify-center pt-2">
                    <div class="w-1.5 h-3 bg-surface-400 rounded-full animate-pulse"></div>
                </div>
            </div>
        </section>
    }
}
