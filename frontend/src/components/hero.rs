use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="relative bg-slate-50 overflow-x-hidden">
            // Background gradient
            <div class="absolute inset-0 bg-gradient-to-br from-indigo-50/50 via-white to-white -z-10 h-[800px]"></div>

            // Hero content
            <main class="max-w-7xl mx-auto px-6 pt-16 pb-24 lg:pt-24">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">

                    // Left: Copy
                    <div class="max-w-2xl">
                        <div class="inline-flex items-center px-3 py-1 rounded-full border border-gray-200 bg-white shadow-sm text-xs font-medium text-gray-600 mb-8">
                            { "Thoughtful software, built with intention" }
                        </div>

                        <h1 class="text-5xl lg:text-6xl font-bold text-gray-900 leading-[1.1] mb-6 tracking-tight">
                            { "I design & build " }
                            <span class="text-indigo-600">{ "human-centered software" }</span>
                        </h1>

                        <p class="text-lg text-gray-600 mb-8 leading-relaxed max-w-lg">
                            { "From systems thinking to clean interfaces, I help teams and individuals turn complex ideas into reliable, maintainable products." }
                        </p>

                        <div class="flex flex-col sm:flex-row gap-4 mb-10">
                            <a
                                href="#contact"
                                class="px-8 py-3.5 rounded-full bg-indigo-600 text-white font-medium hover:bg-indigo-500 transition shadow-lg shadow-indigo-200 text-center"
                            >
                                { "Start a conversation" }
                            </a>

                            <a
                                href="#process"
                                class="px-8 py-3.5 rounded-full bg-white border border-gray-200 text-gray-700 font-medium hover:bg-gray-50 transition text-center"
                            >
                                { "View my process" }
                            </a>
                        </div>

                        <div class="space-y-3">
                            { self::check("Systems-first thinking") }
                            { self::check("Clear communication") }
                            { self::check("Long-term maintainability") }
                        </div>
                    </div>

                    // Right: Visual
                    <div class="relative lg:h-[600px] flex items-center justify-center lg:justify-end">
                        <div class="absolute w-[400px] h-[400px] bg-indigo-100/50 rounded-full blur-3xl -z-10 right-0 top-10"></div>

                        <div class="relative w-[300px] h-[600px] bg-gray-900 rounded-[40px] shadow-2xl border-[8px] border-gray-900 overflow-hidden ring-1 ring-gray-900/5 animate-float">
                            <div class="absolute top-0 left-1/2 -translate-x-1/2 w-24 h-6 bg-black rounded-b-xl z-20"></div>

                            <img
                                src="https://images.unsplash.com/photo-1554463529-e27854014799?q=80&w=600&auto=format&fit=crop"
                                alt="Working session"
                                class="w-full h-full object-cover"
                            />

                            <div class="absolute inset-0 bg-gradient-to-tr from-transparent via-white/10 to-transparent pointer-events-none"></div>
                        </div>
                    </div>
                </div>

                // Trust / metrics bar
                <div class="mt-20 bg-white rounded-3xl shadow-xl border border-gray-100 p-8 grid grid-cols-1 md:grid-cols-3 divide-y md:divide-y-0 md:divide-x divide-gray-100">
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
        <div class="flex items-center gap-3 text-gray-700">
            <svg class="w-5 h-5 text-gray-900" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
            </svg>
            <span class="text-sm font-medium">{ text }</span>
        </div>
    }
}

fn metric(value: &str, label: &str) -> Html {
    html! {
        <div class="px-0 md:px-8 py-6 md:py-0">
            <div class="text-3xl font-bold text-gray-900">{ value }</div>
            <div class="text-sm text-gray-500 mt-1">{ label }</div>
        </div>
    }
}
