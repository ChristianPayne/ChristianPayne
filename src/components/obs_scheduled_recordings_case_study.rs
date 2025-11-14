use leptos::prelude::*;

#[component]
pub fn ObsScheduledRecordingsCaseStudy() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            // Hero Section
            <div class="relative overflow-hidden">
                <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-24">
                    <div class="text-center">
                        <h1 class="text-5xl md:text-7xl font-bold mb-2">
                            "OBS Scheduled Recordings"
                        </h1>
                        <p class="text-lg text-gray-600 mb-6">"obs-scheduled-recordings"</p>
                        <p class="text-xl md:text-2xl mb-8 max-w-3xl mx-auto">
                            "A CLI tool for automating OBS recordings with precise timing control"
                        </p>
                    </div>
                </div>
            </div>

            // What is OBS Section
            <section class="py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
                        <div>
                            <h2 class="text-3xl font-bold mb-6">"What is OBS?"</h2>
                            <p class="text-lg text-gray-600 mb-6">
                                "OBS (Open Broadcaster Software) is a free and open-source software suite for recording and streaming. It's widely used by content creators, streamers, and professionals for capturing and broadcasting video content."
                            </p>
                            <p class="text-lg text-gray-600 mb-6">
                                "OBS allows users to capture their screen, webcam, and audio sources, then combine them into a single video stream or recording. It's particularly popular in the streaming community, but its powerful features make it useful for any video production needs."
                            </p>
                        </div>
                        <div class="bg-white rounded-xl p-8 shadow-sm">
                            <h3 class="text-xl font-semibold mb-4">"Key OBS Features"</h3>
                            <ul class="space-y-4">
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"Screen and window capture"</span>
                                </li>
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"Multiple audio and video sources"</span>
                                </li>
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"Scene management and transitions"</span>
                                </li>
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"WebSocket API for automation"</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>

            // Project Overview Section
            <section class="py-16 bg-gray-50">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-12 items-center">
                        <div>
                            <h2 class="text-3xl font-bold mb-6">"Project Overview"</h2>
                            <p class="text-lg text-gray-600 mb-6">
                                "OBS Scheduled Recordings is a command-line tool that connects to OBS WebSocket to automate recording sessions. It allows users to schedule precise start and end times for their recordings, making it perfect for content creators who need to record at specific times."
                            </p>
                            <p class="text-lg text-gray-600 mb-6">
                                "[PLACEHOLDER: Add information about why this tool was created and its history]"
                            </p>
                        </div>
                        <div class="bg-white rounded-xl p-8 shadow-sm">
                            <h3 class="text-xl font-semibold mb-4">"Key Features"</h3>
                            <ul class="space-y-4">
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"Precise scheduling with millisecond accuracy"</span>
                                </li>
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"Simple command-line interface"</span>
                                </li>
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"OBS WebSocket integration"</span>
                                </li>
                                <li class="flex items-start">
                                    <svg
                                        class="w-6 h-6 text-green-500 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M5 13l4 4L19 7"
                                        ></path>
                                    </svg>
                                    <span>"Cross-platform compatibility"</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Details Section
            <section class="py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-3xl font-bold mb-12 text-center">"Technical Implementation"</h2>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-12">
                        <div class="bg-white rounded-xl p-8 shadow-sm">
                            <h3 class="text-xl font-semibold mb-4">"Technology Stack"</h3>
                            <div class="space-y-4">
                                <div>
                                    <h4 class="font-medium mb-2">"Core Technologies"</h4>
                                    <div class="flex flex-wrap gap-2">
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "Rust"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "OBS WebSocket"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "CLI"
                                        </span>
                                    </div>
                                </div>
                                <div>
                                    <h4 class="font-medium mb-2">"Key Dependencies"</h4>
                                    <div class="flex flex-wrap gap-2">
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "tokio"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "clap"
                                        </span>
                                        <span class="px-3 py-1 text-sm rounded-full border bg-gray-50">
                                            "obs-websocket-rs"
                                        </span>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="bg-white rounded-xl p-8 shadow-sm">
                            <h3 class="text-xl font-semibold mb-4">"Architecture"</h3>
                            <p class="text-gray-600 mb-4">
                                "The application is built with Rust, leveraging its strong type system and async runtime for reliable scheduling and WebSocket communication. The CLI interface is powered by clap, providing a user-friendly way to configure recording schedules."
                            </p>
                            <p class="text-gray-600">
                                "The core functionality revolves around precise timing control and reliable OBS integration, ensuring recordings start and stop exactly when specified."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Call to Action Section
            <section class="py-16">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <h2 class="text-3xl font-bold mb-8">"Ready to Try It Out?"</h2>
                    <p class="text-xl text-gray-600 mb-8 max-w-3xl mx-auto">
                        "Check out the project on GitHub and start automating your OBS recordings today."
                    </p>
                    <a
                        href="https://github.com/ChristianPayne/obs-scheduled-recordings"
                        target="_blank"
                        class="inline-flex items-center px-8 py-4 rounded-lg bg-black text-white font-semibold hover:bg-gray-800 transition-colors"
                    >
                        <svg class="w-6 h-6 mr-2" fill="currentColor" viewBox="0 0 24 24">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z" />
                        </svg>
                        "View on GitHub"
                    </a>
                </div>
            </section>
        </div>
    }
}
