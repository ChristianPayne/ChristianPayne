use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
pub struct CaseStudy {
    pub title: String,
    pub description: String,
    pub image_path: String,
    pub slug: String, // URL-friendly version of title
    pub technologies: Vec<String>,
    pub github_url: Option<String>,
}

#[component]
pub fn CaseStudies() -> impl IntoView {
    let case_studies = vec![
        CaseStudy {
            title: "OBS Scheduled Recordings".to_string(),
            description: "A CLI tool for automating OBS recordings with precise timing control. Built with Rust, it connects to OBS WebSocket to schedule and manage recording sessions with millisecond accuracy.".to_string(),
            image_path: "images/example-obs.jpg".to_string(),
            slug: "obs-scheduled-recordings".to_string(),
            technologies: vec!["Rust".to_string(), "OBS WebSocket".to_string(), "CLI".to_string()],
            github_url: Some("https://github.com/ChristianPayne/obs-scheduled-recordings".to_string()),
        },
        CaseStudy {
            title: "Ennesults - Twitch Chat Bot".to_string(),
            description: "A sophisticated Twitch chat bot built with Rust and Tauri, originally created for streamer Ennegineer. This desktop application features chat integration, native desktop performance, and type-safe architecture with automatic updates.".to_string(),
            image_path: "images/example-twitch.jpg".to_string(),
            slug: "ennesults".to_string(),
            technologies: vec!["Rust".to_string(), "Tauri".to_string(), "SvelteKit".to_string(), "Twitch IRC".to_string()],
            github_url: Some("https://github.com/ChristianPayne/ennesults-rs".to_string()),
        },
        CaseStudy {
            title: "Permission System".to_string(),
            description: "A comprehensive role-based access control system implementation, featuring granular permissions, role management, and secure authentication. This system provides flexible and scalable access control for modern applications.".to_string(),
            image_path: "images/example-permissions.jpg".to_string(),
            slug: "permission-system".to_string(),
            technologies: vec!["TypeScript".to_string(), "GraphQL".to_string(), "AppSync".to_string()],
            github_url: None,
        },
        CaseStudy {
            title: "Personal Website".to_string(),
            description: "A modern, responsive portfolio website built with Rust and Leptos, featuring case studies and a clean, minimalist design. Demonstrates modern web development practices while maintaining excellent performance.".to_string(),
            image_path: "images/example-website.jpg".to_string(),
            slug: "website".to_string(),
            technologies: vec!["Rust".to_string(), "Leptos".to_string(), "Tailwind".to_string()],
            github_url: None,
        },
    ];

    view! {
        <div class="min-h-screen">
            // Hero Section
            <div class="relative overflow-hidden">
                <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
                    <div class="text-center">
                        <h1 class="text-5xl md:text-7xl font-bold mb-6">"Case Studies"</h1>
                        <p class="text-xl md:text-2xl mb-12 max-w-3xl mx-auto text-gray-600">
                            "Explore detailed insights into my projects, showcasing the challenges faced, solutions implemented, and results achieved."
                        </p>
                    </div>
                </div>
            </div>

            // Case Studies Section
            <section class="my-8">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="space-y-16">
                        {case_studies
                            .iter()
                            .map(|case_study| {
                                let title = case_study.title.clone();
                                let description = case_study.description.clone();
                                let image_path = case_study.image_path.clone();
                                let slug = case_study.slug.clone();
                                let technologies = case_study.technologies.clone();
                                let github_url = case_study.github_url.clone();

                                view! {
                                    <div class="rounded-xl p-8 border bg-white shadow-sm hover:shadow-lg transition-all duration-300">
                                        <div class="flex flex-col lg:flex-row items-center gap-8 lg:gap-12">
                                            // Image Section
                                            <div class="w-full lg:w-1/2">
                                                <div class="overflow-hidden rounded-lg shadow-lg">
                                                    <img
                                                        src=image_path
                                                        alt=format!("{} case study image", title)
                                                        class="w-full h-64 lg:h-80 object-cover transition-transform duration-300 hover:scale-105"
                                                    />
                                                </div>
                                            </div>

                                            // Content Section
                                            <div class="w-full lg:w-1/2 text-left">
                                                <h2 class="text-3xl lg:text-4xl font-bold text-gray-800 mb-4">
                                                    {title.clone()}
                                                </h2>

                                                // Technology Tags
                                                <div class="flex flex-wrap gap-2 mb-6">
                                                    {technologies
                                                        .iter()
                                                        .map(|tech| {
                                                            view! {
                                                                <span class="px-2 py-1 text-sm rounded-full border bg-white shadow-sm">
                                                                    {tech.clone()}
                                                                </span>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}
                                                </div>

                                                <p class="text-lg text-gray-600 leading-relaxed mb-8">
                                                    {description}
                                                </p>

                                                // Action Buttons
                                                <div class="flex flex-col sm:flex-row gap-4">
                                                    <Link href=format!("/case-studies/{}", slug)>
                                                        <Button
                                                            appearance=ButtonAppearance::Primary
                                                            class="w-full sm:w-auto inline-flex items-center justify-center px-6 py-3 rounded-lg font-semibold transition-all transform hover:scale-105"
                                                        >
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
                                                        </Button>
                                                    </Link>

                                                    {move || {
                                                        github_url
                                                            .as_ref()
                                                            .map(|url| {
                                                                view! {
                                                                    <a
                                                                        href=url.clone()
                                                                        target="_blank"
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
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </div>
                </div>
            </section>
        </div>
    }
}
