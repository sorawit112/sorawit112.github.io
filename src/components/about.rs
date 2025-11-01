use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section id="about" class="py-12">
            <div class="container mx-auto px-6">
                <h2 class="text-3xl font-bold mb-4 text-primary font-heading">"About Me"</h2>
                <div class="bg-white p-6 rounded-lg shadow-md">
                    <p class="mb-4 text-gray-700 leading-relaxed">
                        "Robotics & Automation engineer with 6+ years across mobile robots and industrial manipulators. Experienced in ROS → ROS2 migrations, navigation/localization, behavior trees, and deploying ABB/KUKA cells that transform manual lines into autonomous production."
                    </p>
                    <ul class="flex flex-wrap gap-4">
                        <li class="bg-gray-200 text-black px-4 py-2 rounded-full shadow-md text-sm">"Tel: +66911130549"</li>
                        <li class="bg-gray-200 text-black px-4 py-2 rounded-full shadow-md text-sm">"Mail: sorawit.inp@gmail.com"</li>
                        <a href="https://github.com/sorawit112" target="_blank" rel="noopener" class="bg-gray-200 text-black px-4 py-2 rounded-full hover:bg-gray-800 hover:text-white transition-colors">"GitHub"</a>
                    </ul>
                </div>
            </div>
        </section>
    }
}
