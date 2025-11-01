use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="container mx-auto p-8 my-8 bg-white rounded-lg shadow-lg text-center">
            <h2 class="text-2xl font-semibold mb-4">"Contact Me"</h2>
            <p class="text-gray-700 mb-4">"Feel free to reach out to me via email or connect on LinkedIn."</p>
            <div class="flex justify-center space-x-6">
                <a href="mailto:john.doe@example.com" class="text-blue-500 hover:underline flex items-center">
                    <img src="https://img.icons8.com/ios-filled/50/000000/email.png" alt="Email" class="w-6 h-6 mr-2"/>
                    "john.doe@example.com"
                </a>
                <a href="https://linkedin.com/in/johndoe" target="_blank" class="text-blue-500 hover:underline flex items-center">
                    <img src="https://img.icons8.com/ios-filled/50/000000/linkedin.png" alt="LinkedIn" class="w-6 h-6 mr-2"/>
                    "LinkedIn/johndoe"
                </a>
                <a href="https://github.com/yourusername" target="_blank" class="text-blue-500 hover:underline flex items-center">
                    <img src="https://img.icons8.com/ios-filled/50/000000/github.png" alt="GitHub" class="w-6 h-6 mr-2"/>
                    "GitHub/yourusername"
                </a>
            </div>
        </section>
    }
}
