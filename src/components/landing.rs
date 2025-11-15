use leptos::either::Either;
use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
enum Event {
    Career,
    Project,
    Education,
}

#[derive(Clone)]
struct TimelineItem {
    year: String,
    title: String,
    description: String,
    event_type: Event,
}

#[component]
pub fn Landing() -> impl IntoView {
    let timeline_items = vec![
        TimelineItem {
            year: "Ongoing".to_string(),
            title: "Ennesults".to_string(),
            description: "A Tauri/Rust Twitch bot that kindly insults people in chat.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2023".to_string(),
            title: "Permission System - Model Match".to_string(),
            description: "Comprehensive permission system with cascading hierarchy, role management, and granular access control for Model Match's multi-tenant platform.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2022".to_string(),
            title: "Chrome Extension - Model Match".to_string(),
            description: "React-based extension for streamlined contact imports straight into the CRM.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2022".to_string(),
            title: "The Event Community".to_string(),
            description: "Full-stack business platform with Remix, Node, and Firebase.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2021".to_string(),
            title: "CRM Features & Custom Fields - Model Match".to_string(),
            description: "Core application features including metadata system and improved import tools.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2021 - Present".to_string(),
            title: "Started at Model Match".to_string(),
            description: "AWS Developer role focusing on mortgage recruiting SaaS platform.".to_string(),
            event_type: Event::Career,
        },
        TimelineItem {
            year: "2020-2021".to_string(),
            title: "UC Irvine Coding Bootcamp".to_string(),
            description: "Intensive full stack development program covering React, Node, Express, MongoDB, and modern web technologies.".to_string(),
            event_type: Event::Education,
        },
        TimelineItem {
            year: "2019-2020".to_string(),
            title: "Self Employed".to_string(),
            description: "Interactive developer creating custom experiences and maintaining client projects.".to_string(),
            event_type: Event::Career,
        },
        TimelineItem {
            year: "2016-2020".to_string(),
            title: "Genesis/Hyundai Wheelstands".to_string(),
            description: "Unity3D applications for auto shows featuring interactive car displays nationwide.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2016".to_string(),
            title: "Programming Journey Began".to_string(),
            description: "First coding experiences building interactive auto show applications.".to_string(),
            event_type: Event::Project,
        },
        TimelineItem {
            year: "2013-2019".to_string(),
            title: "Related Grey".to_string(),
            description: "Multi-role position: Interactive Developer, 2D/3D Designer, and IT Technician.".to_string(),
            event_type: Event::Career,
        },
    ];

    view! {
        <div class="min-h-screen">
            <div class="container mx-auto px-4 max-w-7xl">
                <div class="flex flex-col md:flex-row md:items-stretch gap-6 md:gap-8 my-4">
                    <div class="flex justify-center md:justify-start mb-2 md:mb-0">
                        <img
                            src="images/Profile_Medium.jpeg"
                            alt="Christian Payne Profile Picture"
                            class="w-full max-w-[300px] md:max-w-[400px] h-auto md:h-full rounded-lg shadow-lg object-cover"
                        />
                    </div>

                    <div class="flex flex-col md:w-1/2 md:pt-8">
                        <h2 class="text-2xl md:text-3xl font-bold text-gray-800 mb-2 md:mb-4">
                            Full Stack / Rust Developer
                        </h2>
                        <p class="text-xl md:text-2xl text-gray-800 font-semibold mb-4 md:mb-6">
                            "I build systems to create reliable software"
                        </p>
                        <p class="text-lg md:text-xl text-gray-700 mb-4 md:mb-6">
                            "My journey in tech started in 2016 with interactive auto show applications, and I've been a self starter ever since. These days, I'm doing full stack work on AWS at "
                            <a
                                href="https://modelmatch.com"
                                target="_blank"
                                rel="noopener noreferrer"
                                class="underline hover:text-highlight-100 transition-colors whitespace-nowrap"
                            >
                                "Model Match"
                            </a> ", where I focus on "
                            <Link
                                href="/case-studies/permission-system"
                                class="underline hover:text-highlight-100 transition-colors"
                            >
                                "building robust systems"
                            </Link>
                            ", CRM features, data jobs, and developer tools that make everyone's job easier."
                        </p>
                        <p class="text-lg md:text-xl text-gray-700 mb-4">
                            "I'm passionate about code quality and thoughtful architecture. Working with Rust has reinforced my belief in taking the time to solve problems the right way. Building strong foundations with type safety and performance in mind is key to long-term success."
                        </p>

                        <div class="flex-grow"></div>

                        // Contact Icons
                        <div class="flex flex-wrap justify-center md:justify-start gap-6 md:gap-8">
                            <div class="relative group">
                                <a
                                    href="https://www.linkedin.com/in/christianpayne522/"
                                    target="_blank"
                                    class="flex items-center justify-center w-16 h-16 rounded-lg border bg-white shadow-sm hover:shadow-md transition-all"
                                >
                                    <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z" />
                                    </svg>
                                </a>
                                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                                    <div class="bg-gray-900 text-white text-sm rounded-lg py-2 px-4 whitespace-nowrap">
                                        <div class="font-semibold mb-1">"LinkedIn"</div>
                                        <div class="text-gray-300">"christianpayne522"</div>
                                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
                                    </div>
                                </div>
                            </div>

                            <div class="relative group">
                                <a
                                    href="https://github.com/ChristianPayne"
                                    target="_blank"
                                    class="flex items-center justify-center w-16 h-16 rounded-lg border bg-white shadow-sm hover:shadow-md transition-all"
                                >
                                    <svg class="w-8 h-8" fill="currentColor" viewBox="0 0 24 24">
                                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                    </svg>
                                </a>
                                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                                    <div class="bg-gray-900 text-white text-sm rounded-lg py-2 px-4 whitespace-nowrap">
                                        <div class="font-semibold mb-1">"GitHub"</div>
                                        <div class="text-gray-300">"ChristianPayne"</div>
                                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
                                    </div>
                                </div>
                            </div>

                            <div class="relative group">
                                <a
                                    href="mailto:christianpayne522@gmail.com"
                                    class="flex items-center justify-center w-16 h-16 rounded-lg border bg-white shadow-sm hover:shadow-md transition-all"
                                >
                                    <svg
                                        class="w-8 h-8"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                        />
                                    </svg>
                                </a>
                                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                                    <div class="bg-gray-900 text-white text-sm rounded-lg py-2 px-4 whitespace-nowrap">
                                        <div class="font-semibold mb-1">"Email"</div>
                                        <div class="text-gray-300">
                                            "christianpayne522@gmail.com"
                                        </div>
                                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
                                    </div>
                                </div>
                            </div>

                            <div class="relative group">
                                <a
                                    href="tel:6572341562"
                                    class="flex items-center justify-center w-16 h-16 rounded-lg border bg-white shadow-sm hover:shadow-md transition-all"
                                >
                                    <svg
                                        class="w-8 h-8"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M3 5a2 2 0 012-2h3.28a1 1 0 01.948.684l1.498 4.493a1 1 0 01-.502 1.21l-2.257 1.13a11.042 11.042 0 005.516 5.516l1.13-2.257a1 1 0 011.21-.502l4.493 1.498a1 1 0 01.684.949V19a2 2 0 01-2 2h-1C9.716 21 3 14.284 3 6V5z"
                                        />
                                    </svg>
                                </a>
                                <div class="absolute bottom-full left-1/2 transform -translate-x-1/2 mb-2 opacity-0 group-hover:opacity-100 transition-opacity pointer-events-none">
                                    <div class="bg-gray-900 text-white text-sm rounded-lg py-2 px-4 whitespace-nowrap">
                                        <div class="font-semibold mb-1">"Phone"</div>
                                        <div class="text-gray-300">"(657) 234-1562"</div>
                                        <div class="absolute top-full left-1/2 transform -translate-x-1/2 w-0 h-0 border-l-4 border-r-4 border-t-4 border-transparent border-t-gray-900"></div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                // Signal-based timeline implementation
                <div class="py-10 md:py-20">
                    <div class="max-w-4xl mx-auto">
                        <h2 class="text-4xl font-bold text-center mb-12">
                            "Projects and Career Milestones"
                        </h2>
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
                                                                <span class="text-sm font-medium text-gray-500 mb-1">
                                                                    {year.clone()}
                                                                </span>
                                                                <h3 class="text-xl font-bold text-gray-900 mb-2">
                                                                    {title.clone()}
                                                                </h3>
                                                                <p class="text-sm text-gray-600 mb-2 md:text-right">
                                                                    {description.clone()}
                                                                </p>
                                                                {match &event_type {
                                                                    Event::Career => {
                                                                        view! {
                                                                            <span class="text-sm text-highlight-100">
                                                                                "Career Milestone"
                                                                            </span>
                                                                        }
                                                                    }
                                                                    Event::Project => {
                                                                        view! {
                                                                            <span class="text-sm text-gray-500">
                                                                                "Project Milestone"
                                                                            </span>
                                                                        }
                                                                    }
                                                                    Event::Education => {
                                                                        view! {
                                                                            <span class="text-sm text-gray-500">
                                                                                "Education Milestone"
                                                                            </span>
                                                                        }
                                                                    }
                                                                }}
                                                            </div>
                                                        </div>
                                                    },
                                                )
                                            };
                                            let right_content = if is_even {
                                                Either::Left(

                                                    view! {
                                                        <div class="w-full md:w-1/2 md:pl-8 pl-8 text-left mb-4 md:mb-0">
                                                            <div class="flex flex-col">
                                                                <span class="text-sm font-medium text-gray-500 mb-1">
                                                                    {year}
                                                                </span>
                                                                <h3 class="text-xl font-bold text-gray-900 mb-2">
                                                                    {title}
                                                                </h3>
                                                                <p class="text-sm text-gray-600 mb-2">{description}</p>
                                                                {match &event_type {
                                                                    Event::Career => {
                                                                        view! {
                                                                            <span class="text-sm text-highlight-100">
                                                                                "Career Milestone"
                                                                            </span>
                                                                        }
                                                                    }
                                                                    Event::Project => {
                                                                        view! {
                                                                            <span class="text-sm text-gray-500">
                                                                                "Project Milestone"
                                                                            </span>
                                                                        }
                                                                    }
                                                                    Event::Education => {
                                                                        view! {
                                                                            <span class="text-sm text-gray-500">
                                                                                "Education Milestone"
                                                                            </span>
                                                                        }
                                                                    }
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
                                                            Event::Career => {
                                                                "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-highlight-100 top-0 md:top-1/2 md:-translate-y-1/2"
                                                            }
                                                            Event::Project => {
                                                                "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-white border-2 border-highlight-100 top-0 md:top-1/2 md:-translate-y-1/2"
                                                            }
                                                            Event::Education => {
                                                                "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-white border-2 border-highlight-100 top-0 md:top-1/2 md:-translate-y-1/2"
                                                            }
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
                <section class="py-10 md:py-20">
                    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                        <h2 class="text-4xl font-bold text-center mb-12">"Case Studies"</h2>
                        <div class="grid md:grid-cols-2 lg:grid-cols-3 gap-8">
                            <Link
                                href="/case-studies/ennesults"
                                class="group rounded-xl p-6 border bg-white shadow-sm hover:shadow-md flex flex-col h-full cursor-pointer transition-all text-black hover:text-black hover:no-underline"
                            >
                                <div>
                                    <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                        <svg
                                            class="w-6 h-6"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z"
                                            ></path>
                                        </svg>
                                    </div>
                                    <h3 class="text-xl font-bold mb-3">"Ennesults Twitch Bot"</h3>
                                    <p class="text-gray-600 mb-4">
                                        "A Tauri/Rust desktop application that connects to Twitch IRC and playfully roasts chat members with configurable insult patterns."
                                    </p>
                                    <div class="flex flex-wrap gap-2 mb-4">
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "Rust"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "Tauri"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "SvelteKit"
                                        </span>
                                    </div>
                                </div>
                                <div class="mt-auto">
                                    <div class="inline-flex items-center text-black font-medium group-hover:text-highlight-100 transition-colors">
                                        "View Case Study"
                                        <svg
                                            class="ml-1 w-4 h-4"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9 5l7 7-7 7"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                            </Link>

                            <Link
                                href="/case-studies/website"
                                class="group rounded-xl p-6 border bg-white shadow-sm hover:shadow-md flex flex-col h-full cursor-pointer transition-all text-black hover:text-black hover:no-underline"
                            >
                                <div>
                                    <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                        <svg
                                            class="w-6 h-6"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z"
                                            ></path>
                                        </svg>
                                    </div>
                                    <h3 class="text-xl font-bold mb-3">"Portfolio Website"</h3>
                                    <p class="text-gray-600 mb-4">
                                        "A modern portfolio built with Leptos and Rust, showcasing projects and professional experience with a focus on performance and user experience."
                                    </p>
                                    <div class="flex flex-wrap gap-2 mb-4">
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "Rust"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "Leptos"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "TailwindCSS"
                                        </span>
                                    </div>
                                </div>
                                <div class="mt-auto">
                                    <div class="inline-flex items-center text-black font-medium group-hover:text-highlight-100 transition-colors">
                                        "View Case Study"
                                        <svg
                                            class="ml-1 w-4 h-4"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9 5l7 7-7 7"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                            </Link>

                            <Link
                                href="/case-studies/permission-system"
                                class="group rounded-xl p-6 border bg-white shadow-sm hover:shadow-md flex flex-col h-full cursor-pointer transition-all text-black hover:text-black hover:no-underline"
                            >
                                <div>
                                    <div class="w-12 h-12 rounded-lg flex items-center justify-center mb-4 border bg-gray-50">
                                        <svg
                                            class="w-6 h-6"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                                            ></path>
                                        </svg>
                                    </div>
                                    <h3 class="text-xl font-bold mb-3">"Permission System"</h3>
                                    <p class="text-gray-600 mb-4">
                                        "A scalable role-based permission architecture that manages feature access."
                                    </p>
                                    <div class="flex flex-wrap gap-2 mb-4">
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "TypeScript"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "DynamoDB"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "AppSync"
                                        </span>
                                    </div>
                                </div>
                                <div class="mt-auto">
                                    <div class="inline-flex items-center text-black font-medium group-hover:text-highlight-100 transition-colors">
                                        "View Case Study"
                                        <svg
                                            class="ml-1 w-4 h-4"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M9 5l7 7-7 7"
                                            ></path>
                                        </svg>
                                    </div>
                                </div>
                            </Link>
                        </div>

                        <div class="text-center mt-12">
                            <Link href="/case-studies">
                                <div class="inline-flex items-center px-8 py-4 rounded-lg transition-colors border bg-white shadow-sm hover:bg-gray-50 font-medium text-black hover:text-highlight-100">
                                    "View All Case Studies"
                                    <svg
                                        class="ml-2 w-5 h-5"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M9 5l7 7-7 7"
                                        ></path>
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
