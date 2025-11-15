use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
pub struct CaseStudy {
    pub title: &'static str,
    pub description: &'static str,
    pub image_path: Option<&'static str>,
    pub slug: &'static str, // URL-friendly version of title
    pub technologies: &'static [&'static str],
    pub github_url: Option<&'static str>,
}

static CASE_STUDIES: [CaseStudy; 4] = [
    CaseStudy {
        title: "OBS Scheduled Recordings",
        description: "A CLI tool for automating OBS recordings with precise timing control. Built with Rust, it connects to OBS WebSocket to schedule and manage recording sessions with millisecond accuracy.",
        image_path: Some("images/OBS_Scheduled_Recordings.png"),
        slug: "obs-scheduled-recordings",
        technologies: &["Rust", "OBS WebSocket", "CLI"],
        github_url: Some("https://github.com/ChristianPayne/obs-scheduled-recordings"),
    },
    CaseStudy {
        title: "Ennesults - Twitch Chat Bot",
        description: "A desktop Twitch bot for Ennegineer, rebuilt in Rust with Tauri for speed and flexibility. Easily manage chat, edit insults, and keep it up to date with a built-in auto-updater.",
        image_path: Some("images/ennesults.png"),
        slug: "ennesults",
        technologies: &["Rust", "Tauri", "SvelteKit", "Twitch IRC"],
        github_url: Some("https://github.com/ChristianPayne/ennesults-rs"),
    },
    CaseStudy {
        title: "Permission System",
        description: "A comprehensive role-based access control system implementation, featuring granular permissions, role management, and secure authentication. This system provides flexible and scalable access control for modern applications.",
        image_path: Some("images/permissions.png"),
        slug: "permission-system",
        technologies: &["TypeScript", "DynamoDB", "AppSync"],
        github_url: None,
    },
    CaseStudy {
        title: "Personal Website",
        description: "A modern, responsive portfolio website built with Rust and Leptos, featuring case studies and a clean, minimalist design. Demonstrates modern web development practices while maintaining excellent performance.",
        image_path: None,
        slug: "website",
        technologies: &["Rust", "Leptos", "Tailwind"],
        github_url: None,
    }
];

#[component]
pub fn CaseStudies() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <div class="container mx-auto px-4 max-w-7xl">
                // Hero Section
                <div class="text-center mt-8 md:mt-12">
                    <h1 class="text-5xl md:text-7xl font-bold mb-6">"Case Studies"</h1>
                    <p class="text-xl md:text-2xl mb-12 max-w-3xl mx-auto text-gray-600">
                        "Explore detailed insights into my projects, showcasing the challenges faced, solutions implemented, and results achieved."
                    </p>
                </div>

                // Case Studies Section
                <section class="py-10 md:py-20">
                    <div class="space-y-16">
                        {CASE_STUDIES
                            .iter()
                            .map(|case_study| {
                                let slug = case_study.slug;
                                view! {
                                    <Link
                                        href=format!("/case-studies/{}", slug)
                                        class="rounded-xl p-8 border bg-white shadow-sm hover:shadow-lg transition-all duration-300 cursor-pointer text-black hover:text-black hover:no-underline block"
                                    >
                                        <div class=move || {
                                            if case_study.image_path.is_some() {
                                                "flex flex-col lg:flex-row items-center gap-8 lg:gap-12"
                                            } else {
                                                "flex flex-col"
                                            }
                                        }>
                                            {move || {
                                                case_study
                                                    .image_path
                                                    .map(|image_path| {
                                                        view! {
                                                            // Image Section
                                                            <div class="w-full lg:w-1/2">
                                                                <div class="overflow-hidden rounded-lg shadow-lg">
                                                                    <img
                                                                        src=image_path
                                                                        alt=format!("{} case study image", case_study.title)
                                                                        class="w-full h-64 lg:h-80 object-cover transition-transform duration-300 hover:scale-105"
                                                                    />
                                                                </div>
                                                            </div>
                                                        }
                                                    })
                                            }} // Content Section
                                            <div class=move || {
                                                if case_study.image_path.is_some() {
                                                    "w-full lg:w-1/2 text-left"
                                                } else {
                                                    "w-full text-left"
                                                }
                                            }>
                                                <h2 class="text-3xl lg:text-4xl font-bold text-gray-800 mb-4">
                                                    {case_study.title}
                                                </h2>

                                                // Technology Tags
                                                <div class="flex flex-wrap gap-2 mb-6">
                                                    {case_study
                                                        .technologies
                                                        .iter()
                                                        .map(|tech| {
                                                            view! {
                                                                <span class="px-2 py-1 text-sm rounded-full border bg-white shadow-sm">
                                                                    {tech.to_string()}
                                                                </span>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </div>

                                                <p class="text-lg text-gray-600 leading-relaxed mb-8">
                                                    {case_study.description}
                                                </p>

                                                // Action Buttons
                                                <div class="flex flex-col sm:flex-row gap-4">
                                                    <div class="w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 rounded-lg font-semibold transition-all transform hover:scale-105 bg-highlight-100 text-white">
                                                        "View Case Study"
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

                                                    {move || {
                                                        case_study
                                                            .github_url
                                                            .as_ref()
                                                            .map(|url| {
                                                                view! {
                                                                    <a
                                                                        href=*url
                                                                        target="_blank"
                                                                        on:click=move |ev| {
                                                                            ev.stop_propagation();
                                                                        }
                                                                        class="w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 rounded-lg font-semibold border bg-white hover:bg-gray-50 transition-colors"
                                                                    >
                                                                        <svg
                                                                            class="w-5 h-5 mr-2"
                                                                            fill="currentColor"
                                                                            viewBox="0 0 24 24"
                                                                        >
                                                                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                                                        </svg>
                                                                        "View on GitHub"
                                                                    </a>
                                                                }
                                                            })
                                                    }}
                                                </div>
                                            </div>
                                        </div>
                                    </Link>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </section>
            </div>
        </div>
    }
}
