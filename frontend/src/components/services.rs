use yew::prelude::*;

#[derive(Clone)]
struct Project {
    title: &'static str,
    description: &'static str,
    icon: &'static str,
    details: &'static str,
}

#[function_component(Services)]
pub fn services() -> Html {
    let flip = use_state(|| 0.0);
    let showing_models = use_state(|| true);
    let model_index = use_state(|| 0);
    let agent_index = use_state(|| 0);

    let on_flip_click = {
        let flip = flip.clone();
        let showing_models = showing_models.clone();
        Callback::from(move |_| {
            let new_showing = !*showing_models;
            flip.set(if new_showing { 0.0 } else { 1.0 });
            showing_models.set(new_showing);
        })
    };

    let on_prev = {
        let showing_models = showing_models.clone();
        let model_idx = model_index.clone();
        let agent_idx = agent_index.clone();
        Callback::from(move |_| {
            if *showing_models {
                let count = 4;
                model_idx.set((*model_idx + count - 1) % count);
            } else {
                let count = 4;
                agent_idx.set((*agent_idx + count - 1) % count);
            }
        })
    };

    let on_next = {
        let showing_models = showing_models.clone();
        let model_idx = model_index.clone();
        let agent_idx = agent_index.clone();
        Callback::from(move |_| {
            if *showing_models {
                let count = 4;
                model_idx.set((*model_idx + 1) % count);
            } else {
                let count = 4;
                agent_idx.set((*agent_idx + 1) % count);
            }
        })
    };

    // Models projects
    let models_projects = vec![
        Project {
            title: "Portfolio Site",
            description: "Modern portfolio built with Yew and Rust",
            icon: "🎨",
            details: "A complete website redesign focusing on performance and accessibility",
        },
        Project {
            title: "Component Library",
            description: "Reusable UI components with Tailwind styling",
            icon: "📦",
            details: "Design system implementation with 50+ reusable components",
        },
        Project {
            title: "WASM Visualization",
            description: "High-performance data visualization in the browser",
            icon: "📊",
            details: "Real-time data rendering using Rust and WebAssembly",
        },
        Project {
            title: "Design System",
            description: "Comprehensive design tokens and theming",
            icon: "🎯",
            details: "End-to-end design system from tokens to implementation",
        },
    ];

    // Agents projects  
    let agents_projects = vec![
        Project {
            title: "Dev Assistant",
            description: "AI-powered development helper with context awareness",
            icon: "🤖",
            details: "Context-aware AI that understands your codebase",
        },
        Project {
            title: "Code Review Bot",
            description: "Automated code review with suggestions",
            icon: "🔍",
            details: "Smart code review automation with Rust analysis",
        },
        Project {
            title: "Workflow Automation",
            description: "Smart task automation for productivity",
            icon: "⚡",
            details: "Intelligent workflow optimization and task automation",
        },
        Project {
            title: "Knowledge Base",
            description: "Intelligent documentation and search",
            icon: "📚",
            details: "RAG-powered documentation with natural language search",
        },
    ];

    let (current_title, current_desc, current_icon, current_details, current_index, total_count) = if *showing_models {
        let p = &models_projects[*model_index % models_projects.len()];
        (p.title, p.description, p.icon, p.details, *model_index, models_projects.len())
    } else {
        let p = &agents_projects[*agent_index % agents_projects.len()];
        (p.title, p.description, p.icon, p.details, *agent_index, agents_projects.len())
    };

    let rotation = *flip * 180.0;

    html! {
        <div 
            class="flex flex-col items-center justify-center min-h-screen w-full px-2 overflow-x-hidden"
            style="box-sizing: border-box;"
        >
            // Header - mobile-first sizing
            <div class="text-center mb-3 w-full">
                <h2 
                    class="text-lg font-bold mb-1"
                    style="color: var(--color-surface-50);"
                >
                    { if *showing_models { "Models" } else { "Agents" } }
                </h2>
                <p 
                    class="text-xs"
                    style="color: var(--color-surface-400);"
                >
                    { "Tap arrows to navigate" }
                </p>
            </div>

            // Navigation row
            <div class="flex items-center justify-between w-full max-w-xs px-2 gap-2 mb-3">
                <button 
                    onclick={on_prev}
                    class="w-8 h-8 rounded-full flex items-center justify-center text-lg transition hover:scale-110"
                    style="background-color: rgba(255,255,255,0.1); color: var(--color-surface-50);"
                >
                    { "←" }
                </button>

                // Dots
                <div class="flex gap-1">
                    { for (0..total_count).map(|i| html! {
                        <div 
                            class={if i == current_index { 
                                "w-2 h-2 rounded-full transition-all bg-white" 
                            } else { 
                                "w-1.5 h-1.5 rounded-full opacity-50 transition-all bg-white/70" 
                            }}
                        />
                    })}
                </div>

                <button 
                    onclick={on_next}
                    class="w-8 h-8 rounded-full flex items-center justify-center text-lg transition hover:scale-110"
                    style="background-color: rgba(255,255,255,0.1); color: var(--color-surface-50);"
                >
                    { "→" }
                </button>
            </div>

            // 3D Flip container
            <div 
                class="relative w-[85vw] aspect-[4/5] max-w-xs"
                style="perspective: 1000px;"
            >
                <div 
                    class="w-full h-full transition-transform duration-700"
                    style={format!(
                        "transform-style: preserve-3d; transform: rotateY({}deg);",
                        rotation
                    )}
                >
                    // Models side
                    <div 
                        class="absolute inset-0 flex flex-col items-center justify-center p-4 rounded-xl shadow-2xl"
                        style="background-color: var(--color-highlight-500); backface-visibility: hidden;"
                    >
                        <div class="text-5xl mb-3">{ current_icon }</div>
                        
                        <h3 class="text-base font-bold mb-2 text-center text-neutral-950">
                            { current_title }
                        </h3>
                        
                        <p class="text-xs text-center mb-2 text-neutral-900 opacity-90 px-2">
                            { current_desc }
                        </p>
                        
                        <p class="text-xs text-center text-neutral-800 opacity-80 mb-3 px-2">
                            { current_details }
                        </p>

                        <button class="px-4 py-2 rounded-full text-xs font-bold mt-auto" style="background-color: rgba(0,0,0,0.8); color: white;">
                            { "View Project" }
                        </button>
                    </div>

                    // Agents side
                    <div 
                        class="absolute inset-0 flex flex-col items-center justify-center p-4 rounded-xl shadow-2xl"
                        style="background-color: var(--color-primary-600); backface-visibility: hidden; transform: rotateY(180deg);"
                    >
                        <div class="text-5xl mb-3">{ current_icon }</div>
                        
                        <h3 class="text-base font-bold mb-2 text-center text-white">
                            { current_title }
                        </h3>
                        
                        <p class="text-xs text-center mb-2 text-white opacity-90 px-2">
                            { current_desc }
                        </p>
                        
                        <p class="text-xs text-center text-white opacity-80 mb-3 px-2">
                            { current_details }
                        </p>

                        <button class="px-4 py-2 rounded-full text-xs font-bold mt-auto" style="background-color: rgba(0,0,0,0.8); color: white;">
                            { "View Project" }
                        </button>
                    </div>
                </div>
            </div>

            // Flip button
            <div class="flex items-center justify-center gap-2 mt-5">
                <span 
                    class="text-xs font-medium"
                    style={if *showing_models { "color: var(--color-highlight-400);" } else { "color: var(--color-surface-500);" }}
                >
                    { "M" }
                </span>

                <button 
                    class="w-10 h-10 rounded-full flex items-center justify-center text-lg transition hover:scale-110"
                    style="background-color: var(--color-surface-800); color: var(--color-surface-100);"
                    onclick={on_flip_click}
                >
                    { if *showing_models { "→" } else { "←" } }
                </button>

                <span 
                    class="text-xs font-medium"
                    style={if !*showing_models { "color: var(--color-primary-400);" } else { "color: var(--color-surface-500);" }}
                >
                    { "A" }
                </span>
            </div>
        </div>
    }
}
