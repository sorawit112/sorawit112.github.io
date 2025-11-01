use leptos::prelude::*;

#[component]
fn ProjectItem(
    title: &'static str,
    duration: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white p-6 rounded-lg shadow-md">
            <h3 class="text-xl font-bold text-primary">{title}</h3>
            <p class="text-sm text-gray-600 mb-2">{duration}</p>
            <p class="text-gray-600">{description}</p>
        </div>
    }
}

#[component]
pub fn Projects() -> impl IntoView {
    view! {
        <section id="projects" class="py-12">
            <div class="container mx-auto px-6">
                <h2 class="text-3xl font-bold mb-4 text-primary font-heading">"Projects"</h2>
                <div class="grid md:grid-cols-2 gap-6">
                    <ProjectItem
                        title="CP-Meiji Depalletizing Robot"
                        duration="Apr 2025 – Present"
                        description="Automated depalletizing for box unpacking & line feeding (ABB manipulator + Mitsubishi PLC). Industrial transformation: 6 operators → 3."
                    />
                    <ProjectItem
                        title="Ajinomoto Autonomous Vibration System"
                        duration="Nov 2024 – Jan 2025"
                        description="Depalletize sacks → vibration table → palletize back; KUKA manipulator + Omron PLC. Operators reduced 6 → 1."
                    />
                    <ProjectItem
                        title="Obodroid Mobile Robot Software"
                        duration="Jul 2021 – Dec 2024"
                        description="Software Architecture, navigation, docking, localization, obstacle avoidance, multi-maps routing, behavior trees; ROS1→ROS2 migration."
                    />
                    <ProjectItem
                        title="Manipulator Energy Optimization"
                        duration="Sep – Dec 2022"
                        description="Optimized velocity/acceleration to reduce energy via ROS-based simulation & search."
                    />
                    <ProjectItem
                        title="Autonomous Operation for Multi-Floor Robot Navigation"
                        duration="Aug 2018 - MAY 2019"
                        description="Differential drive robot and 5-DOF manipulator for elevator operation; Task Planner for multi-floor navigation using StateMachine"
                    />
                </div>
            </div>
        </section>
    }
}
