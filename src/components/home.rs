use leptos::either::Either;
use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
enum EventType {
    MainEvent,
    SubEvent,
}

#[derive(Clone)]
struct TimelineItem {
    year: String,
    description: String,
    event_type: EventType,
}

#[component]
pub fn Home() -> impl IntoView {
    let timeline_items = vec![
        TimelineItem {
            year: "Present".to_string(),
            description: "Current Role at Model Match".to_string(),
            event_type: EventType::MainEvent,
        },
        TimelineItem {
            year: "2023 Q4".to_string(),
            description: "Launched new feature X".to_string(),
            event_type: EventType::SubEvent,
        },
        TimelineItem {
            year: "2023 Q4".to_string(),
            description: "Launched new feature X".to_string(),
            event_type: EventType::SubEvent,
        },
        TimelineItem {
            year: "2023".to_string(),
            description: "Started at Model Match".to_string(),
            event_type: EventType::MainEvent,
        },
        TimelineItem {
            year: "2023 Q2".to_string(),
            description: "Completed project Y".to_string(),
            event_type: EventType::SubEvent,
        },
        TimelineItem {
            year: "2022".to_string(),
            description: "Previous Experience".to_string(),
            event_type: EventType::MainEvent,
        },
        TimelineItem {
            year: "2022 Q3".to_string(),
            description: "Led team initiative Z".to_string(),
            event_type: EventType::SubEvent,
        },
        TimelineItem {
            year: "2021".to_string(),
            description: "Earlier Experience".to_string(),
            event_type: EventType::MainEvent,
        },
        TimelineItem {
            year: "2021 Q1".to_string(),
            description: "Implemented system A".to_string(),
            event_type: EventType::SubEvent,
        },
        TimelineItem {
            year: "2020".to_string(),
            description: "Beginning of Journey".to_string(),
            event_type: EventType::MainEvent,
        },
    ];

    view! {
        <div class="container mx-auto px-4 max-w-7xl">
            <div class="flex flex-col md:flex-row md:items-start gap-8 my-8">
                <div class="flex justify-center md:justify-start">
                    <img
                        src="images/Profile_Medium.jpeg"
                        alt="Christian Payne Profile Picture"
                        class="w-full max-w-[300px] md:max-w-[500px] rounded-lg shadow-lg"
                    />
                </div>

                <div class="mb-8 md:mb-0">
                    <h2 class="text-2xl md:text-3xl font-bold text-gray-800 mb-4">About Me</h2>
                    <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-4">
                        I am a developer who works with code to solve problems.
                    </p>
                    <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-4">
                        I pride myself on attention to detail and creating strong foundations for others to build off of.
                    </p>
                    <p class="text-lg md:text-xl text-gray-700 leading-relaxed mb-6">
                        Based in Southern California, I am currently working as a Developer at Model Match.
                    </p>

                    <Link href="/case-studies">
                        <div class="inline-flex items-center text-blue-600 font-medium transition-all duration-300 hover:text-blue-700 hover:translate-x-2 relative group cursor-pointer">
                            "Case Studies"
                            <svg
                                class="ml-2 w-5 h-5 transition-transform duration-300 group-hover:translate-x-1"
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
                            <div class="absolute bottom-0 left-0 w-0 h-0.5 bg-blue-600 transition-all duration-300 ease-in-out group-hover:w-full"></div>
                        </div>
                    </Link>
                </div>
            </div>

            <Divider />

            // Signal-based timeline implementation
            <div class="relative mt-12">
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
                                let description = item.description.clone();
                                let is_main_event = matches!(item.event_type, EventType::MainEvent);
                                let left_content = if is_even {
                                    Either::Left(

                                        view! { <div class="hidden md:block w-1/2"></div> },
                                    )
                                } else {
                                    Either::Right(
                                        view! {
                                            <div class="w-full md:w-1/2 md:pr-8 pl-8 text-left md:text-right mb-4 md:mb-0">
                                                <h3 class="text-lg font-semibold">{year.clone()}</h3>
                                                <p class="text-gray-600 mt-2">{description.clone()}</p>
                                                {if !is_main_event {
                                                    view! {
                                                        <p class="text-sm text-gray-500 mt-1">
                                                            "Project Milestone"
                                                        </p>
                                                    }
                                                } else {
                                                    view! {
                                                        <p class="text-sm text-blue-500 mt-1">"Career Milestone"</p>
                                                    }
                                                }}
                                            </div>
                                        },
                                    )
                                };
                                let right_content = if is_even {
                                    Either::Left(

                                        view! {
                                            <div class="w-full md:w-1/2 md:pl-8 pl-8 text-left mt-4 md:mt-0">
                                                <h3 class="text-lg font-semibold">{year}</h3>
                                                <p class="text-gray-600 mt-2">{description}</p>
                                                {if !is_main_event {
                                                    view! {
                                                        <p class="text-sm text-gray-500 mt-1">
                                                            "Project Milestone"
                                                        </p>
                                                    }
                                                } else {
                                                    view! {
                                                        <p class="text-sm text-blue-500 mt-1">"Career Milestone"</p>
                                                    }
                                                }}
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
                                            if is_main_event {
                                                "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-blue-500"
                                            } else {
                                                "absolute left-4 md:left-1/2 transform -translate-x-1/2 w-4 h-4 rounded-full bg-white border-2 border-blue-500"
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
    }
}
