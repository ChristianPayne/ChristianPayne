// Comprehensive case study page for Ennesults Twitch bot project

use leptos::{prelude::*, task::spawn_local};
use serde::{Deserialize, Serialize};

// Struct for GitHub repository data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubRepo {
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
    pub size: u32,
    pub open_issues_count: u32,
    pub watchers_count: u32,
    pub default_branch: String,
}

// Struct for GitHub release data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubRelease {
    pub tag_name: String,
    pub name: Option<String>,
    pub published_at: String,
}

// Struct for GitHub commits data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GitHubCommitsResponse {
    pub total_count: u32,
}

// Combined repository stats
#[derive(Debug, Clone, Default)]
pub struct RepoStats {
    pub repo: Option<GitHubRepo>,
    pub latest_release: Option<GitHubRelease>,
    pub total_commits: Option<u32>,
    pub total_releases: Option<u32>,
}

// Combined data structure
#[derive(Debug, Clone)]
pub struct RepoData {
    pub repo: GitHubRepo,
    pub latest_release: Option<GitHubRelease>,
    pub total_releases: Option<u32>,
}

// Enhanced fetch function to get repo, release, and release count
async fn fetch_github_data() -> Option<RepoData> {
    leptos::logging::log!("Fetching GitHub repository data...");

    let client = reqwest::Client::new();
    let user_agent = "ChristianPayne-Portfolio";
    let repo_url = "https://api.github.com/repos/ChristianPayne/ennesults-rs";

    // Handle GitHub token (hardcoded for now)
    let auth_header = Some("token <YOUR_TOKEN_HERE>".to_string());

    if auth_header.is_some() {
        leptos::logging::log!("🔑 Using GitHub token for authentication");
    } else {
        leptos::logging::log!("⚠️ No GitHub token found, using unauthenticated requests");
    }

    // Fetch repository data
    let mut request = client
        .get(repo_url)
        .header("User-Agent", user_agent)
        .header("Accept", "application/vnd.github.v3+json");

    if let Some(ref auth) = auth_header {
        request = request.header("Authorization", auth);
    }

    let repo_data = match request.send().await {
        Ok(response) if response.status().is_success() => {
            leptos::logging::log!(
                "Rate limit remaining: {:?}",
                response.headers().get("x-ratelimit-remaining")
            );
            leptos::logging::log!(
                "Rate limit reset: {:?}",
                response.headers().get("x-ratelimit-reset")
            );
            match response.json::<GitHubRepo>().await {
                Ok(data) => {
                    leptos::logging::log!("✅ Repository data loaded: {}", &data.name);
                    Some(data)
                }
                Err(e) => {
                    leptos::logging::error!("Failed to parse repo JSON: {:?}", e);
                    None
                }
            }
        }
        Ok(response) => {
            leptos::logging::error!("Repo request failed with status: {}", response.status());
            None
        }
        Err(e) => {
            leptos::logging::error!("Repo network error: {:?}", e);
            None
        }
    };

    let repo_data = repo_data?; // Return None if repo fetch failed

    // Fetch latest release
    leptos::logging::log!("Fetching latest release...");
    let mut request = client
        .get(&format!("{}/releases/latest", repo_url))
        .header("User-Agent", user_agent);

    if let Some(ref auth) = auth_header {
        request = request.header("Authorization", auth);
    }

    let latest_release = match request.send().await {
        Ok(response) if response.status().is_success() => {
            leptos::logging::log!(
                "Rate limit remaining: {:?}",
                response.headers().get("x-ratelimit-remaining")
            );
            leptos::logging::log!(
                "Rate limit reset: {:?}",
                response.headers().get("x-ratelimit-reset")
            );
            match response.json::<GitHubRelease>().await {
                Ok(release) => {
                    leptos::logging::log!("✅ Latest release: {}", &release.tag_name);
                    Some(release)
                }
                Err(e) => {
                    leptos::logging::error!("Failed to parse release JSON: {:?}", e);
                    None
                }
            }
        }
        Ok(response) => {
            leptos::logging::log!("No releases found (status: {})", response.status());
            None
        }
        Err(e) => {
            leptos::logging::error!("Release fetch error: {:?}", e);
            None
        }
    };

    // Fetch total releases count
    leptos::logging::log!("Fetching releases count...");
    let mut request = client
        .get(&format!("{}/releases", repo_url))
        .header("User-Agent", user_agent);

    if let Some(ref auth) = auth_header {
        request = request.header("Authorization", auth);
    }

    let total_releases = match request.send().await {
        Ok(response) if response.status().is_success() => {
            match response.json::<Vec<GitHubRelease>>().await {
                Ok(releases) => {
                    let count = releases.len() as u32;
                    leptos::logging::log!("✅ Found {} releases", count);
                    Some(count)
                }
                Err(e) => {
                    leptos::logging::error!("Failed to parse releases JSON: {:?}", e);
                    None
                }
            }
        }
        Ok(response) => {
            leptos::logging::error!("Releases count request failed: {}", response.status());
            None
        }
        Err(e) => {
            leptos::logging::error!("Releases count fetch error: {:?}", e);
            None
        }
    };

    Some(RepoData {
        repo: repo_data,
        latest_release,
        total_releases,
    })
}

#[component]
pub fn EnnesultsCaseStudy() -> impl IntoView {
    // Signals for repository and related data
    let (repo_data, set_repo_data) = signal::<Option<GitHubRepo>>(None);
    let (latest_release, set_latest_release) = signal::<Option<GitHubRelease>>(None);
    let (total_releases, set_total_releases) = signal::<Option<u32>>(None);

    // Simple one-time fetch when component mounts
    leptos::logging::log!("Component mounting, scheduling API call...");

    // Use Effect::new to run only once
    let has_fetched = StoredValue::new(false);

    Effect::new(move |_| {
        if !has_fetched.get_value() {
            has_fetched.set_value(true);
            leptos::logging::log!("Initiating GitHub API fetch...");

            spawn_local(async move {
                leptos::logging::log!("About to call fetch_github_data...");
                if let Some(data) = fetch_github_data().await {
                    leptos::logging::log!("Setting all data...");
                    set_repo_data.set(Some(data.repo));
                    set_latest_release.set(data.latest_release);
                    set_total_releases.set(data.total_releases);
                } else {
                    leptos::logging::log!("No data received from API");
                }
            });
        }
    });

    view! {
        <div class="min-h-screen">
            // Hero Section
            <div class="relative overflow-hidden">
                <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
                    <div class="text-center">
                        <h1 class="text-5xl md:text-7xl font-bold mb-2">"Ennesults"</h1>
                        <p class="text-lg text-gray-600 mb-6">
                            {move || {
                                repo_data
                                    .get()
                                    .as_ref()
                                    .map(|r| r.name.clone())
                                    .unwrap_or_else(|| "ennesults-rs".to_string())
                            }}
                        </p>
                        <p class="text-xl md:text-2xl mb-8 max-w-3xl mx-auto">
                            {move || {
                                repo_data
                                    .get()
                                    .as_ref()
                                    .and_then(|r| r.description.clone())
                                    .unwrap_or_else(|| {
                                        "A Tauri/Rust Twitch bot that kindly insults people in chat"
                                            .to_string()
                                    })
                            }}
                        </p>
                        <div class="flex flex-wrap justify-center gap-4 mb-8">
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                {move || {
                                    repo_data
                                        .get()
                                        .as_ref()
                                        .and_then(|r| r.language.clone())
                                        .unwrap_or_else(|| "Rust".to_string())
                                }}
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "Tauri"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "SvelteKit"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "Twitch IRC"
                            </span>
                        </div>
                        <div class="flex justify-center gap-6">
                            <a
                                href=move || {
                                    repo_data
                                        .get()
                                        .as_ref()
                                        .map(|r| r.html_url.clone())
                                        .unwrap_or_else(|| {
                                            "https://github.com/ChristianPayne/ennesults-rs".to_string()
                                        })
                                }
                                target="_blank"
                                class="inline-flex items-center px-6 py-3 rounded-lg transition-colors border bg-white shadow-sm hover:bg-gray-50"
                            >
                                <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
                                    <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                                </svg>
                                "View on GitHub"
                            </a>
                        </div>
                    </div>
                </div>
            </div>

            // Project Overview
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-6">
                            <h2 class="text-4xl font-bold mb-6">"Project Overview"</h2>
                            <p class="text-lg">
                                "Ennesults is a sophisticated Twitch chat bot originally created for streamer Ennegineer (Enne). This Rust edition represents a complete rewrite of the original JavaScript version, focusing on performance, reliability, and user experience."
                            </p>
                            <p class="text-lg">
                                "The bot features an intuitive desktop application built with Tauri, allowing Enne to manage her bot configuration, monitor chat activity, and customize insult patterns."
                            </p>
                            <p class="text-lg">
                                "The bot also features updater functionality, allowing Enne to easily update the bot to the latest version without having to manually download the latest release."
                            </p>
                            <div class="grid grid-cols-2 gap-6">
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h3 class="font-semibold mb-2">
                                        {move || {
                                            total_releases
                                                .get()
                                                .map(|r| format!("{} Releases", r))
                                                .unwrap_or_else(|| "Loading...".to_string())
                                        }}
                                    </h3>
                                    <p class="text-sm">"Stable iterations"</p>
                                </div>
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h3 class="font-semibold mb-2">
                                        {move || {
                                            latest_release
                                                .get()
                                                .as_ref()
                                                .map(|r| r.tag_name.clone())
                                                .unwrap_or_else(|| "Loading...".to_string())
                                        }}
                                    </h3>
                                    <p class="text-sm">"Latest version"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Architecture
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Technical Architecture"</h2>
                    <div class="grid md:grid-cols-2 gap-8">
                        <div class="rounded-xl p-8 border bg-white shadow-sm">
                            <h3 class="text-2xl font-bold mb-6">"Frontend Stack"</h3>
                            <div class="space-y-4">
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"SvelteKit"</span>
                                    <span class="ml-auto text-sm">"Modern UI framework"</span>
                                </div>
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"TailwindCSS"</span>
                                    <span class="ml-auto text-sm">"Utility-first styling"</span>
                                </div>
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"ShadCN (Svelte)"</span>
                                    <span class="ml-auto text-sm">"Component library"</span>
                                </div>
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"Heroicons"</span>
                                    <span class="ml-auto text-sm">"Icon system"</span>
                                </div>
                            </div>
                        </div>
                        <div class="rounded-xl p-8 border bg-white shadow-sm">
                            <h3 class="text-2xl font-bold mb-6">"Backend Stack"</h3>
                            <div class="space-y-4">
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"Tauri"</span>
                                    <span class="ml-auto text-sm">"Desktop app framework"</span>
                                </div>
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"Rust"</span>
                                    <span class="ml-auto text-sm">"Systems programming"</span>
                                </div>
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"Twitch IRC"</span>
                                    <span class="ml-auto text-sm">"Chat integration"</span>
                                </div>
                                <div class="flex items-center">
                                    <div class="w-3 h-3 rounded-full mr-3 border bg-white"></div>
                                    <span class="font-medium">"TS RS"</span>
                                    <span class="ml-auto text-sm">"Type generation"</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Key Features
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">
                        "Key Features & Capabilities"
                    </h2>
                    <div class="grid md:grid-cols-3 gap-8">
                        <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm">
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
                            <h3 class="text-xl font-bold mb-3">"Intelligent Chat Integration"</h3>
                            <p>
                                "Seamlessly connects to Twitch IRC, monitoring chat activity and responding with perfectly timed, 'insults' that enhance community engagement."
                            </p>
                        </div>
                        <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm">
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
                                        d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                    ></path>
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Desktop Application"</h3>
                            <p>
                                "Native desktop experience with Tauri, providing low resource usage, fast performance, and the ability for streamers to host the bot locally."
                            </p>
                        </div>
                        <div class="rounded-xl p-6 border transition-colors bg-white shadow-sm">
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
                                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Type-Safe Architecture"</h3>
                            <p>
                                "Leverages TS RS for automatic type generation between Rust backend and TypeScript frontend, ensuring runtime safety and excellent developer experience."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Development Journey
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Development Journey"</h2>
                    <div class="max-w-4xl mx-auto">
                        <div class="relative">
                            <div class="absolute left-8 top-0 bottom-0 w-0.5 border-l"></div>
                            <div class="space-y-12">
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">"Project Inception"</h3>
                                        <p>
                                            "Started as a learning project to explore Rust and Tauri while improving upon the original JavaScript bot. The goal was to create a more maintainable, performant solution."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">
                                            "Architecture Design"
                                        </h3>
                                        <p>
                                            "Chose Tauri for the perfect balance of web technologies and native performance. SvelteKit provided an excellent developer experience for the frontend."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">"IRC Integration"</h3>
                                        <p>
                                            "Implemented robust Twitch IRC connectivity with proper error handling, reconnection logic, and rate limiting to ensure reliable chat interaction."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">
                                            "User Experience Focus"
                                        </h3>
                                        <p>
                                            "Built an intuitive UI allowing streamers to configure the bot, monitor activity, and customize behavior without technical knowledge."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">
                                            "Ongoing Collaboration"
                                        </h3>
                                        <p>
                                            "This project continues to evolve through collaboration with Enne and the streaming community. Regular feedback sessions drive new feature development, from command customization to UI improvements. The modular architecture enables rapid iteration and experimentation with new ideas."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Highlights
            <section class="py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Technical Highlights"</h2>
                    <div class="grid lg:grid-cols-2 gap-12">
                        <div>
                            <h3 class="text-2xl font-bold mb-6">"Performance Optimizations"</h3>
                            <div class="space-y-4">
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h4 class="font-semibold mb-2">"Memory Efficiency"</h4>
                                    <p class="text-sm">
                                        "Rust's zero-cost abstractions and ownership model ensure minimal memory usage compared to the JavaScript predecessor."
                                    </p>
                                </div>
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h4 class="font-semibold mb-2">"Native Performance"</h4>
                                    <p class="text-sm">
                                        "Tauri's native backend provides desktop-class performance while maintaining web technology familiarity."
                                    </p>
                                </div>
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h4 class="font-semibold mb-2">"Async Architecture"</h4>
                                    <p class="text-sm">
                                        "Non-blocking IRC handling ensures UI responsiveness even during high chat activity periods."
                                    </p>
                                </div>
                            </div>
                        </div>
                        <div>
                            <h3 class="text-2xl font-bold mb-6">"Developer Experience"</h3>
                            <div class="space-y-4">
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h4 class="font-semibold mb-2">"Type Safety"</h4>
                                    <p class="text-sm">
                                        "Full type safety from Rust backend to TypeScript frontend using TS RS for automatic type generation."
                                    </p>
                                </div>
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h4 class="font-semibold mb-2">"Hot Reloading"</h4>
                                    <p class="text-sm">
                                        "Development workflow with instant feedback using Vite's HMR and Tauri's development server."
                                    </p>
                                </div>
                                <div class="rounded-lg p-4 border bg-white shadow-sm">
                                    <h4 class="font-semibold mb-2">"Cross-Platform"</h4>
                                    <p class="text-sm">
                                        "Single codebase deploys to Windows, macOS, and Linux with platform-specific optimizations."
                                    </p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Footer CTA
            <section class="py-20">
                <div class="max-w-4xl mx-auto text-center px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold mb-6">"Explore the Project"</h2>
                    <p class="text-xl mb-8">
                        "Dive into the codebase and see how modern Rust and web technologies come together to create a delightful user experience."
                    </p>
                    <div class="flex flex-col sm:flex-row gap-4 justify-center">
                        <a
                            href="https://github.com/ChristianPayne/ennesults-rs"
                            target="_blank"
                            class="inline-flex items-center justify-center px-8 py-4 rounded-lg font-semibold transition-all transform hover:scale-105 border bg-white shadow-sm hover:bg-gray-50"
                        >
                            <svg class="w-5 h-5 mr-2" fill="currentColor" viewBox="0 0 24 24">
                                <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                            </svg>
                            "View Source Code"
                        </a>
                        <a
                            href="https://github.com/ChristianPayne/ennesults-rs/releases"
                            target="_blank"
                            class="inline-flex items-center justify-center px-8 py-4 rounded-lg font-semibold transition-all border bg-white shadow-sm hover:bg-gray-50"
                        >
                            <svg
                                class="w-5 h-5 mr-2"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 19l3 3m0 0l3-3m-3 3V10"
                                ></path>
                            </svg>
                            "Download Latest"
                        </a>
                    </div>
                </div>
            </section>
        </div>
    }
}
