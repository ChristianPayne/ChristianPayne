use leptos::either::Either;
use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
enum EventType {
    CareerMilestone,
    ProjectMilestone,
}

#[derive(Clone)]
struct TimelineItem {
    year: String,
    title: String,
    description: String,
    event_type: EventType,
}

#[component]
pub fn Home() -> impl IntoView {
    let timeline_items = vec![
        TimelineItem {
            year: "2023 - Present".to_string(),
            title: "Ennesults".to_string(),
            description: "Rust/Tauri desktop app for Twitch chat interactions with real-time IRC integration".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2023".to_string(),
            title: "Permission System - Model Match".to_string(),
            description: "Flagship scalable, role-based permission architecture integrated with payment system".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2022".to_string(),
            title: "Chrome Extension - Model Match".to_string(),
            description: "React-based extension for streamlined contact imports straight into the CRM.".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2022".to_string(),
            title: "The Event Community".to_string(),
            description: "Full-stack business platform with Remix, Node, and Firebase.".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2021".to_string(),
            title: "CRM Features & Custom Fields - Model Match".to_string(),
            description: "Core application features including metadata system and improved import tools".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2021 - Present".to_string(),
            title: "Started at Model Match".to_string(),
            description: "AWS Developer role focusing on mortgage recruiting SaaS platform.".to_string(),
            event_type: EventType::CareerMilestone,
        },
        TimelineItem {
            year: "2020-2021".to_string(),
            title: "UC Irvine Coding Bootcamp".to_string(),
            description: "Intensive full stack development program covering React, Node, Express, MongoDB, and modern web technologies".to_string(),
            event_type: EventType::CareerMilestone,
        },
        TimelineItem {
            year: "2019-2020".to_string(),
            title: "Self Employed".to_string(),
            description: "Interactive developer creating custom experiences and maintaining client projects".to_string(),
            event_type: EventType::CareerMilestone,
        },
        TimelineItem {
            year: "2016-2020".to_string(),
            title: "Genesis/Hyundai Wheelstands".to_string(),
            description: "Unity3D applications for auto shows featuring interactive car displays nationwide".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2016".to_string(),
            title: "Programming Journey Began".to_string(),
            description: "First coding experiences building interactive auto show applications".to_string(),
            event_type: EventType::ProjectMilestone,
        },
        TimelineItem {
            year: "2013-2019".to_string(),
            title: "Related Grey".to_string(),
            description: "Multi-role position: Interactive Developer, 2D/3D Designer, and IT Technician".to_string(),
            event_type: EventType::CareerMilestone,
        },
    ];

    view! {
        <div class="min-h-screen" style="background-color: #fafafa; background-image: radial-gradient(circle, rgba(0,0,0,.15) 1px, transparent 1px); background-size: 20px 20px;">
            <div class="container mx-auto px-4 max-w-7xl">
                <div class="flex flex-col md:flex-row md:items-start gap-8 my-8">
                    <div class="flex justify-center md:justify-start">
                        <img
                            src="images/Profile_Medium.jpeg"
                            alt="Christian Payne Profile Picture"
                            class="max-w-[300px] md:max-w-[400px] h-auto rounded-lg shadow-lg object-cover"
                        />
                    </div>

                    <div class="mb-8 md:mb-0">
                        <h2 class="text-2xl md:text-3xl font-bold text-gray-800 mb-4">About Me</h2>
                        <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-4">
                            "Hey there! I'm a developer who gets genuinely excited about making things work better."
                        </p>
                        <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-4">
                            "I got into programming back in 2016 when working on apps for auto shows then I got hooked on the satisfaction of building something from scratch that actually solves real problems. These days, I'm working with Rust a lot, which feels like the perfect fit for someone who obsesses over the details."
                        </p>
                        <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-4">
                            "Working for Model Match here in Southern California, I get to work on some really interesting challenges—from building robust permission systems that manage features for user roles, to creating developer CLI tools that make everyone's job easier. I've also built a browser extension and various ETL jobs that keep our data clean and up to date."
                        </p>
                        <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-6">
                            "I'm the kind of person who will spend a good amount of time up front to think about the best way to solve a problem. My teammates probably think I'm a bit obsessive about code quality, but I'd rather build something solid once than patch it forever."
                        </p>
                    </div>
                </div>

                <Divider />

                // Signal-based timeline implementation
                <div class="py-20">
                    <div class="max-w-4xl mx-auto">
                        <h2 class="text-4xl font-bold text-center mb-12">"Projects and Career Milestones"</h2>
                        <div class="rounded-xl p-8 border bg-white shadow-sm">
                            <div class="relative">
                                // Timeline line - left side on mobile, centered on desktop
                                <div class="absolute left-4 md:left-1/2 transform md:-translate-x-1/2 h-full w-0.5 bg-gray-300"></div>

                                // Timeline items
                                <div class="space-y-2">
                                    {timeline_items
                                        .iter()
                                        .enumerate()
                                        .map(|(index, item)| {
                                            let is_even = index % 2 == 0;
                                            let year = item.year.clone();
                                            let title = item.title.clone();
                                            let description = item.description.clone();
                                            let event_type = item.event_type.clone();
                                            let left_content = if is_even {
                                                Either::Left(

                                                    view! { <div class="hidden md:block w-1/2"></div> },
                                                )
                                            } else {
                                                Either::Right(
                                                    view! {
                                                        <div class="w-full md:w-1/2 md:pr-8 pl-8 text-left md:text-right mb-4 md:mb-0">
                                                            <div class="flex flex-col md:items-end">
                                                                <span class="text-sm font-medium text-gray-500 mb-1">{year.clone()}</span>
                                                                <h3 class="text-xl font-bold text-gray-900 mb-2">{title.clone()}</h3>
                                                                <p class="text-sm text-gray-600 mb-2 md:text-right">{description.clone()}</p>
                                                                {match &event_type {
                                                                    EventType::CareerMilestone => view! {
                                                                        <span class="text-sm text-blue-500">"Career Milestone"</span>
                                                                    },
                                                                    EventType::ProjectMilestone => view! {
                                                                        <span class="text-sm text-gray-500">"Project Milestone"</span>
                                                                    },
                                                                }}
                                                            </div>
                                                        </div>
                                                    },
                                                )
                                            };
                                            let right_content = if is_even {
                                                Either::Left(

                                                    view! {
                                                        <div class="w-full md:w-1/2 md:pl-8 pl-8 text-left mt-4 md:mt-0">
                                                            <div class="flex flex-col">
                                                                <span class="text-sm font-medium text-gray-500 mb-1">{year}</span>
                                                                <h3 class="text-xl font-bold text-gray-900 mb-2">{title}</h3>
                                                                <p class="text-sm text-gray-600 mb-2">{description}</p>
                                                                {match &event_type {
                                                                    EventType::CareerMilestone => view! {
                                                                        <span class="text-sm text-blue-500">"Career Milestone"</span>
                                                                    },
                                                                    EventType::ProjectMilestone => view! {
                                                                        <span class="text-sm text-gray-500">"Project Milestone"</span>
                                                                    },
                                                                }}
                                                            </div>
                                                        </div>
                                                    },
                                                )
                                            } else {
                                                Either::Right(
                                                    view! { <div class="hidden md:block w-1/2"></div> },
                                                )
                                            };

                                            view! {
                                                <div class="relative flex flex-col md:flex-row items-start md:items-center">
                                                    {left_content}
                                                    <div class=move || {
                                                        match &event_type {
                                                            EventType::CareerMilestone => "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-blue-500",
                                                            EventType::ProjectMilestone => "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-white border-2 border-blue-500",
                                                        }
                                                    }></div> {right_content}
                                                </div>
                                            }
                                        })
                                        .collect::<Vec<_>>()}
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Case Studies Section
                <section class="py-20">
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <h2 class="text-4xl font-bold text-center mb-12">"Case Studies"</h2>
                        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm hover:shadow-md">
                                <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"></path>
                                    </svg>
                                </div>
                                <h3 class="text-xl font-bold mb-3">"Ennesults Twitch Bot"</h3>
                                <p class="text-gray-600 mb-4">"A Tauri/Rust desktop application that connects to Twitch IRC and playfully roasts chat members with configurable insult patterns."</p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"Rust"</span>
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"Tauri"</span>
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"SvelteKit"</span>
                                </div>
                                <Link href="/case-studies/ennesults">
                                    <div class="inline-flex items-center text-blue-600 font-medium hover:text-blue-700 transition-colors">
                                        "View Case Study"
                                        <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                        </svg>
                                    </div>
                                </Link>
                            </div>

                            <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm hover:shadow-md">
                                <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"></path>
                                    </svg>
                                </div>
                                <h3 class="text-xl font-bold mb-3">"Portfolio Website"</h3>
                                <p class="text-gray-600 mb-4">"A modern portfolio built with Leptos and Rust, showcasing projects and professional experience with a focus on performance and user experience."</p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"Rust"</span>
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"Leptos"</span>
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"TailwindCSS"</span>
                                </div>
                                <Link href="/case-studies/portfolio">
                                    <div class="inline-flex items-center text-blue-600 font-medium hover:text-blue-700 transition-colors">
                                        "View Case Study"
                                        <svg class="ml-1 w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                        </svg>
                                    </div>
                                </Link>
                            </div>

                            <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm hover:shadow-md">
                                <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19.428 15.428a2 2 0 00-1.022-.547l-2.387-.477a6 6 0 00-3.86.517l-.318.158a6 6 0 01-3.86.517L6.05 15.21a2 2 0 00-1.806.547M8 4h8l-1 1v5.172a2 2 0 00.586 1.414l5 5c1.26 1.26.367 3.414-1.415 3.414H4.828c-1.782 0-2.674-2.154-1.414-3.414l5-5A2 2 0 009 10.172V5L8 4z"></path>
                                    </svg>
                                </div>
                                <h3 class="text-xl font-bold mb-3">"Coming Soon"</h3>
                                <p class="text-gray-600 mb-4">"More case studies and projects are on the way. Check back soon to see additional examples of my work and development process."</p>
                                <div class="flex flex-wrap gap-2 mb-4">
                                    <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">"TBD"</span>
                                </div>
                                <div class="inline-flex items-center text-gray-500 font-medium">
                                    "Coming Soon"
                                </div>
                            </div>
                        </div>

                        <div class="text-center mt-12">
                            <Link href="/case-studies">
                                <div class="inline-flex items-center px-8 py-4 rounded-lg transition-colors border bg-white shadow-sm hover:bg-gray-50 font-medium">
                                    "View All Case Studies"
                                    <svg class="ml-2 w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7"></path>
                                    </svg>
                                </div>
                            </Link>
                        </div>
                    </div>
                </section>
            </div>
        </div>
    }
}
