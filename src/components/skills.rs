use std::vec;

use leptos::prelude::*;

#[component]
fn SkillCategory(title: &'static str, skills: Vec<&'static str>) -> impl IntoView {
    view! {
        <div class="mb-6">
            <h3 class="text-xl font-bold text-primary mb-2">{title}</h3>
            <ul class="flex flex-wrap gap-2 ">
                {skills.into_iter().map(|skill| {
                    view! { <li class="text-black px-4 py-2 rounded-full shadow-md bg-grey text-sm"> {skill}</li> }
                }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn Skills() -> impl IntoView {
    let robotics_skills = vec![
        "a-star, dijkstra, RRT, conflict-based",
        "SLAM, VIO, VSLAM, Particle Filter, KF, EKF",
        "Behavior Trees, State Machines",
        "OpenCV, YOLO, ResNet",
        "ONNYX",
        "ABB, KUKA",
        "Cognex, RealSense, Mechmind",
        "MQTT, HTTP, WebSocket, Modbus",
        "Intel NUC / Xavier AGX / Khadas",
    ];
    let programming_skills = vec![
        "C++, Python, Rust",
        "MATLAB, Simulink",
        "PLC (Ladder)",
        "SolidWork, Blender",
        "Windows, Ubuntu",
    ];
    let framework_skills = vec![
        "Leptos, Actix",
        "Tailwind",
        "Elasticsearch, Kibana & Logstash",
        "ROS, ROS2",
    ];
    let simualtion_skills = vec!["Gazebo", "Issac Sim, Isaac Lab, Isaac Gym"];
    let soft_skills = vec![
        "Team Leadership",
        "R&D Collaboration",
        "Project Management",
        "Thai (Native)",
        "English Speaking and Writing (A2), Listening (B1), Reading (B2)",
    ];

    view! {
        <section id="skills" class="py-12">
            <div class="container mx-auto px-6">
                <h2 class="text-3xl font-bold mb-4 text-primary font-heading">"Skills"</h2>
                <div class="bg-white p-6 rounded-lg shadow-md">
                    <SkillCategory title="Robotics" skills=robotics_skills />
                    <SkillCategory title="Programming & Tools" skills=programming_skills />
                    <SkillCategory title="Frameworks" skills=framework_skills />
                    <SkillCategory title="Simulations" skills=simualtion_skills />
                    <SkillCategory title="Soft & Language" skills=soft_skills />
                </div>
            </div>
        </section>
    }
}
