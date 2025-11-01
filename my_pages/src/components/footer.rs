use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-primary text-white text-center py-6 mt-12">
            <div class="container mx-auto">
                <p class="mb-2">"© 2025 Sorawit Inprom. All rights reserved."</p>
            </div>
        </footer>
    }
}
