use leptos::prelude::*;

#[component]
fn ExperienceItem(
    title: &'static str,
    company: &'static str,
    duration: &'static str,
    description: Vec<&'static str>,
) -> impl IntoView {
    view! {
        <div class="bg-white p-6 rounded-lg shadow-md mb-6">
            <div class="flex justify-between items-baseline">
                <h3 class="text-xl font-bold text-primary">{title}</h3>
                <span class="text-sm text-gray-600">{duration}</span>
            </div>
            <p class="text-md text-gray-700 mb-2">{company}</p>
            <ul class="list-disc list-inside text-gray-600 space-y-1">
                {description.into_iter().map(|desc| view! {<li>{desc}</li> }).collect_view()}
            </ul>
        </div>
    }
}

#[component]
pub fn Experience() -> impl IntoView {
    view! {
        <section id="experience" class="py-12">
            <div class="container mx-auto px-6">
                <h2 class="text-3xl font-bold mb-4 text-primary font-heading">"Professional Experience"</h2>
                <div class="space-y-6">
                    <ExperienceItem
                        title="Robotics & AI Software Developer"
                        company="Obodroid Corporation, Bangkok"
                        duration="Jul 2021 – Present"
                        description=vec![
                            "Lead Mobile Robot Navigation and Localization Team",
                            "Designs robot software architecture and robot behavior for coffee vending machine robot, build on top of partner's mobile robot base which controlled via websocket API",
                            "Develop Robot Behavior using Behavaior Tree data structure from BehaviorTree.cpp also implement BehaviorTree Converter which converts .xml (file created from Groot) to PyTree structure for running in ROS2 rclpy node",
                            "Develop Navigation Routing Structure for multi-maps navigation",
                            "Leverage multi-sensors (e.g., 2D LIDAR, IMU, UWB) to enhance robot localization",
                            "Develop Precision Automatic Docking System which use datas from Odometry, Camera, Lidar and IMU",
                            "Develop base drivers for differential & ackermann kinematics",
                            "Migrated the stack from ROS1 to ROS2",
                            "Led palletizing automation using KUKA & ABB manipulators.",
                            "Develop Digital Twin for palletizing automation using Robot Studio and OPC-UA protocol"
                        ]
                    />
                    <ExperienceItem
                        title="System Integration & R&D Consultant"
                        company="Freelance, Bangkok"
                        duration="Jul 2021 – Present"
                        description=vec![
                            "Contributed with advisor from university to create demo multi-robot for delivery application.",
                            "Develop algorithm to optimize energy consumption of industrail robot trajectory",
                            "Integrated Industrial Automation System using KUKA, ABB manipulator",
                        ]
                    />
                    <ExperienceItem
                        title="Automation Engineer"
                        company="Asahi‑Thai Alloy, Samut Prakan"
                        duration="May 2019 – Jul 2021"
                        description=vec![
                            "Improve production process using Lean Automation Methodology by layout optimization and waste reduction",
                            "Deployed KUKA robots for faucet production",
                            "Improved cycle time & accuracy via smart automation.",
                        ]
                    />
                     <ExperienceItem
                        title="Backend Developer (Part‑time, R&D)"
                        company="CoXSys Robotics, Bangkok"
                        duration="May – Aug 2020"
                        description=vec![
                            "Centralized & decentralized control for multi‑agent robots",
                            "Developed Coverage path planning algorithm using automatic routing convex vertices (generated from map boundaries) for UVC robots.",
                        ]
                    />
                </div>
            </div>
        </section>
    }
}
