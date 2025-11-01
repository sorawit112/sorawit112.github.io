use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-primary text-black shadow-md top-0 z-50">
            <div class="container mx-auto px-6 py-4 flex justify-between items-center">
                <div>
                    <h1 class="text-2xl font-bold font-heading">"Sorawit Inprom"</h1>
                    <p class="text-sm text-black">"Robotics & Automation Engineer"</p>
                </div>
                <nav class="hidden md:flex space-x-6">
                    <a href="#about" class="hover:text-accent">"About"</a>
                    <a href="#experience" class="hover:text-accent">"Experience"</a>
                    <a href="#projects" class="hover:text-accent">"Projects"</a>
                    <a href="#education" class="hover:text-accent">"Education"</a>
                    <a href="#skills" class="hover:text-accent">"Skills"</a>
                </nav>
            </div>
        </header>
    }
}
