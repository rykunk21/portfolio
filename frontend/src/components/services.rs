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

    let on_flip_input = {
        let flip = flip.clone();
        let showing_models = showing_models.clone();
        Callback::from(move |event: InputEvent| {
            if let Some(input) = event.target_dyn_into::<web_sys::HtmlInputElement>() {
                let value = input.value_as_number();
                flip.set(value);
                showing_models.set(value < 0.5);
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

    let models_count = models_projects.len();
    let agents_count = agents_projects.len();

    let prev_project = {
        let showing_models = showing_models.clone();
        let model_index = model_index.clone();
        let agent_index = agent_index.clone();
        Callback::from(move |_| {
            if *showing_models {
                model_index.set((*model_index + models_count - 1) % models_count);
            } else {
                agent_index.set((*agent_index + agents_count - 1) % agents_count);
            }
        })
    };

    let next_project = {
        let showing_models = showing_models.clone();
        let model_index = model_index.clone();
        let agent_index = agent_index.clone();
        Callback::from(move |_| {
            if *showing_models {
                model_index.set((*model_index + 1) % models_count);
            } else {
                agent_index.set((*agent_index + 1) % agents_count);
            }
        })
    };

    let (current_title, current_desc, current_icon, current_details) = if *showing_models {
        let p = &models_projects[*model_index % models_count];
        (p.title, p.description, p.icon, p.details)
    } else {
        let p = &agents_projects[*agent_index % agents_count];
        (p.title, p.description, p.icon, p.details)
    };

    let current_index = if *showing_models { *model_index } else { *agent_index };
    let total_count = if *showing_models { models_count } else { agents_count };

    let card_bg = if *showing_models { "var(--color-highlight-500)" } else { "var(--color-primary-500)" };
    let card_text = "var(--color-neutral-950)";

    html! {
        <div class="flex flex-col items-center justify-center min-h-screen" style="background-color: var(--color-neutral-950);">
            
            // Main display area - full viewport card
            <div class="relative w-full max-w-6xl mx-4 flex-1 flex flex-col">
                
                // Navigation - Previous
                <button 
                    onclick={prev_project}
                    class="absolute left-0 top-1/2 -translate-y-1/2 z-10 p-4 rounded-full transition hover:scale-110"
                    style="background-color: rgba(255,255,255,0.1); color: var(--color-surface-50);"
                >
                    { "←" }
                </button>

                // Navigation - Next
                <button 
                    onclick={next_project}
                    class="absolute right-0 top-1/2 -translate-y-1/2 z-10 p-4 rounded-full transition hover:scale-110"
                    style="background-color: rgba(255,255,255,0.1); color: var(--color-surface-50);"
                >
                    { "→" }
                </button>

                // Main card
                <div 
                    class="flex-1 flex flex-col items-center justify-center p-12 rounded-2xl shadow-2xl transition-all duration-500 mx-16"
                    style={format!("background-color: {}; color: {};", card_bg, card_text)}
                >
                    // Category badge
                    <div 
                        class="px-4 py-2 rounded-full text-sm font-bold mb-8"
                        style="background-color: rgba(0,0,0,0.2);"
                    >
                        { if *showing_models { "MODEL" } else { "AGENT" } }
                    </div>

                    // Project icon
                    <div class="text-8xl mb-8">{ current_icon }</div>

                    // Project title
                    <h2 class="text-5xl font-bold mb-4 text-center">{ current_title }</h2>

                    // Description
                    <p class="text-2xl text-center mb-6 opacity-90 max-w-2xl">
                        { current_desc }
                    </p>

                    // Details
                    <p class="text-lg text-center mb-8 opacity-80 max-w-xl">
                        { current_details }
                    </p>

                    // View button
                    <button 
                        class="px-8 py-3 rounded-full font-bold transition hover:scale-105"
                        style="background-color: rgba(0,0,0,0.8); color: white;"
                    >
                        { "View Project" }
                    </button>

                    // Dot indicators
                    <div class="flex gap-3 mt-8">
                        { for (0..total_count).map(|i| html! {
                            <div 
                                class={if i == current_index { "w-4 h-4 rounded-full transition-all" } else { "w-3 h-3 rounded-full opacity-50 transition-all" }}
                                style="background-color: rgba(0,0,0,0.6);"
                            />
                        })}
                    </div>
                </div>
            </div>

            // Bottom flip control
            <div class="flex items-center gap-6 p-8">
                <span 
                    class="text-xl font-bold transition-colors"
                    style={if *showing_models { "color: var(--color-highlight-400);" } else { "color: var(--color-surface-500);" }}
                >
                    { "Models" }
                </span>

                // Flip slider
                <div class="relative w-80 h-12 rounded-full overflow-hidden cursor-pointer"
                     style="background: linear-gradient(to right, var(--color-highlight-500), var(--color-primary-500));"
                >
                    <input
                        type="range"
                        min="0"
                        max="1"
                        step="0.01"
                        value={flip.to_string()}
                        oninput={on_flip_input}
                        class="absolute inset-0 w-full h-full opacity-0 cursor-pointer"
                    />
                    // Sliding indicator
                    <div 
                        class="absolute top-1 h-10 w-20 rounded-full flex items-center justify-center text-sm font-bold transition-all duration-300 shadow-lg"
                        style={format!(
                            "background-color: rgba(255,255,255,0.9); left: calc({}% - 40px); color: var(--color-neutral-950);",
                            *flip * 100.0
                        )}
                    >
                        { if *showing_models { "←" } else { "→" } }
                    </div>
                </div>

                <span 
                    class="text-xl font-bold transition-colors"
                    style={if !*showing_models { "color: var(--color-primary-400);" } else { "color: var(--color-surface-500);" }}
                >
                    { "Agents" }
                </span>
            </div>
        </div>
    }
}
