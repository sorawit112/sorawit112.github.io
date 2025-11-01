use leptos::prelude::*;

#[component]
pub fn Education() -> impl IntoView {
    view! {
        <section id="education" class="py-12">
            <div class="container mx-auto px-6">
                <h2 class="text-3xl font-bold mb-4 text-primary font-heading">"Education & Research"</h2>
                <div class="bg-white p-6 rounded-lg shadow-md">
                    <h3 class="text-xl font-bold text-primary">"B.Eng., Robotics & Automation (First-Class Honours, GPA 3.91)"</h3>
                    <p class="text-sm text-gray-600 mb-2">"2015 – 2019 | King Mongkut’s University of Technology Thonburi"</p>
                    <ul class="list-disc list-inside text-gray-600 space-y-1">
                        <li>"Senior Project: Autonomous Multi-Floor Robot Navigation"</li>
                        <li>"World Congress on Undergraduate Research, Oldenburg, Germany (May 2019)."</li>
                        <li>"Teaching/RA — Intro to Robotics, Control Theory, Python (2017–2019)."</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}
