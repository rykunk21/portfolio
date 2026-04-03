use yew::prelude::*;
/// --- Hero component ---
#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="relative overflow-x-hidden text-surface-50">

            <main class="max-w-7xl mx-auto px-6 pt-16 pb-24 lg:pt-24">
                <div class="relative grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div class="relative mx-auto w-full max-w-[500px] h-[520px] sm:h-[420px] lg:h-[650px] rounded-2xl overflow-hidden animate-float flex items-center justify-center z-0 mt-0 lg:mt-0 order-1 lg:order-2">
                        /* Foreground portfolio image - no smear, no border */
                        <img
                            src="media/portfolio.png"
                            alt="Working session"
                            class="relative z-10 w-full h-full object-contain"
                        />
                        /* Subtle highlight */
                        <div
                            class="absolute inset-0 pointer-events-none z-20"
                            style="background: linear-gradient(to top right, transparent, var(--color-neutral-900)/10, transparent);"
                        ></div>

                        // Mobile overlay title only
                        <div class="absolute inset-0 z-30 flex items-end p-4 lg:hidden">
                            <div class="bg-neutral-900/60 backdrop-blur-md rounded-lg p-3 max-w-[90%]">
                                <h1 class="font-header text-3xl leading-tight font-bold" style="color: var(--color-surface-50);">
                                    { "I design & build " }
                                    <br />
                                    <span style="color: var(--color-highlight-400);">{ "human-centered software" }</span>
                                </h1>
                            </div>
                        </div>
                    </div>

                    <div class="relative z-10 max-w-2xl order-2 lg:order-1 lg:block hidden">

                        <h1 style="color: var(--color-highlight-500);" class="font-header text-5xl lg:text-6xl font-bold leading-[1.1] mb-6 tracking-tight">
                            { "I design & build " }
                            <br />
                            <span style="color: var(--color-surface-50);">{ "human-centered software" }</span>
                        </h1>

                        <p style="color: var(--color-surface-400);" class="text-lg mb-8 leading-relaxed max-w-lg">
                            { "From systems thinking to clean interfaces, I help teams and individuals turn complex ideas into reliable, maintainable products." }
                        </p>

                        <div class="flex flex-col sm:flex-row gap-4 mb-10">
                            <a
                                href="#contact"
                                style="background-color: var(--color-highlight-500); color: var(--color-neutral-950);"
                                class="px-8 py-3.5 rounded-full font-medium hover:opacity-90 transition shadow-lg text-center"
                            >
                                { "Start a conversation" }
                            </a>

                            <a
                                href="#process"
                                style="background-color: var(--color-neutral-800); color: var(--color-surface-50); border-color: var(--color-neutral-600);"
                                class="px-8 py-3.5 rounded-full font-medium hover:opacity-90 transition text-center border"
                            >
                                { "View my process" }
                            </a>
                        </div>

                        <div class="
                          space-y-3
                          relative
                          rounded-xl
                          bg-neutral-800/70
                          backdrop-blur-md
                          p-4

                          sm:bg-transparent
                          sm:backdrop-blur-0
                          sm:p-0
                            ">
                            { check("Systems-first thinking") }
                            { check("Clear communication") }
                            { check("Long-term maintainability") }
                        </div>
                    </div>
                </div>

                <div class="lg:hidden flex flex-col gap-4 mt-6">
                    <a
                        href="#contact"
                        class="px-6 py-3 rounded-full font-medium text-center"
                        style="background-color: var(--color-highlight-500); color: var(--color-neutral-950);"
                    >
                        { "Start a conversation" }
                    </a>
                    <a
                        href="#process"
                        class="px-6 py-3 rounded-full font-medium text-center"
                        style="background-color: var(--color-neutral-800); color: var(--color-surface-50); border: 1px solid var(--color-neutral-600);"
                    >
                        { "View my process" }
                    </a>
                </div>

                <div class="mt-20 rounded-3xl shadow-xl border border-neutral-800 p-8 grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x divide-neutral-700" style="background-color: var(--color-neutral-900);">
                    { metric("10+ years", "Building software") }
                    { metric("Rust • Web • Systems", "Primary focus") }
                    { metric("Quality > Speed", "Core principle") }
                </div>
            </main>
        </section>
    }
}

// --- helpers ---

fn check(text: &str) -> Html {
    html! {
        <div class="flex items-center gap-3" style="color: var(--color-surface-200);">
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24" style="color: var(--color-highlight-500);">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span class="text-sm font-medium">{ text }</span>
        </div>
    }
}

fn metric(value: &str, label: &str) -> Html {
    html! {
        <div class="px-0 md:px-8 py-6 md:py-0">
            <div class="text-3xl font-bold" style="color: var(--color-surface-50);">{ value }</div>
            <div class="text-sm mt-1" style="color: var(--color-surface-400);">{ label }</div>
        </div>
    }
}
