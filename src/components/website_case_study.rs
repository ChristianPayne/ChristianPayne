use leptos::prelude::*;
use thaw::*;

#[component]
pub fn WebsiteCaseStudy() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            // Hero Section
            <div class="relative overflow-hidden">
                <div class="relative max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12 sm:py-24">
                    <div class="text-center">
                        <h1 class="text-5xl md:text-7xl font-bold mb-2">"Personal Website"</h1>
                        <p class="text-lg text-gray-600 mb-6">"Portfolio & Case Studies"</p>
                        <p class="text-xl md:text-2xl mb-8 max-w-3xl mx-auto">
                            "My personal corner of the web, built with Rust and Leptos. A place to share my projects and experiences, with a focus on clean design and smooth performance."
                        </p>
                        <div class="flex flex-wrap justify-center gap-4 mb-8">
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "Rust"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "Leptos"
                            </span>
                            <span class="px-4 py-2 rounded-full border bg-white shadow-sm">
                                "Tailwind"
                            </span>
                        </div>
                    </div>
                </div>
            </div>

            // Project Overview
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-6">
                            <h2 class="text-4xl font-bold mb-6">"Project Overview"</h2>
                            <p class="text-lg">
                                "This website serves as both a portfolio and a platform for sharing detailed case studies of my work. Built with Rust and Leptos, it demonstrates modern web development practices while maintaining excellent performance and developer experience."
                            </p>
                            <p class="text-lg">
                                "The site features a clean, minimalist design with a focus on typography and spacing. It's built to be fast, accessible, and responsive across all devices. The case studies section provides in-depth looks at various projects, showcasing technical details and implementation approaches."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // History & Motivation
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-6">
                            <h2 class="text-4xl font-bold mb-6">"History & Motivation"</h2>
                            <p class="text-lg">
                                "The initial motivation for this website was to create a platform that could showcase both my work and my writing. Traditional portfolio sites often focus solely on projects, while blogs are separate entities. I wanted to combine these aspects into a cohesive experience."
                            </p>
                            <p class="text-lg">
                                "The choice of Rust and Leptos was driven by a desire to build something performant and maintainable. Rust's safety guarantees and Leptos's modern approach to web development made it an ideal combination for a personal website that would grow over time."
                            </p>
                            <p class="text-lg">
                                "The design philosophy emphasizes readability and user experience. The grid background adds subtle texture without distraction, while the typography system ensures content is easily digestible. The case studies section was designed to provide depth and context to each project, going beyond simple project listings."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Technical Architecture
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Technical Architecture"</h2>
                </div>
            </section>

            // Security & Implementation
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="max-w-4xl mx-auto">
                        <div class="space-y-8">
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Leptos Framework"</h3>
                                <p class="text-lg">
                                    "The site is built using Leptos, a Rust framework for building web applications. Leptos provides a modern, reactive programming model while maintaining Rust's performance and safety guarantees. The framework's component system allows for clean, maintainable code organization."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Tailwind CSS"</h3>
                                <p class="text-lg">
                                    "Styling is handled through Tailwind CSS, providing a utility-first approach to design. This allows for rapid development while maintaining consistency across the site. Custom configurations extend Tailwind with brand colors and typography settings."
                                </p>
                            </div>
                            <div>
                                <h3 class="text-2xl font-bold mb-4">"Font System"</h3>
                                <p class="text-lg">
                                    "The site uses a custom font system with Space Grotesk as the primary typeface and Roboto as a fallback. Fonts are loaded locally for optimal performance, with a carefully selected range of weights to support various text styles throughout the site."
                                </p>
                            </div>
                        </div>
                    </div>
                </div>
            </section>

            // Key Features
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Key Features"</h2>
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
                                        d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Responsive Design"</h3>
                            <p>
                                "Fully responsive layout that adapts to any screen size. Mobile-first approach with careful attention to typography and spacing across all breakpoints."
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
                                        d="M13 10V3L4 14h7v7l9-11h-7z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Performance"</h3>
                            <p>
                                "Optimized for speed with local font loading, minimal dependencies, and efficient asset delivery. Built with performance in mind from the ground up."
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
                                        d="M4 5a1 1 0 011-1h14a1 1 0 011 1v2a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM4 13a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H5a1 1 0 01-1-1v-6zM16 13a1 1 0 011-1h2a1 1 0 011 1v6a1 1 0 01-1 1h-2a1 1 0 01-1-1v-6z"
                                    ></path>
                                </svg>
                            </div>
                            <h3 class="text-xl font-bold mb-3">"Case Studies"</h3>
                            <p>
                                "Detailed case studies showcasing projects with technical details, challenges faced, and solutions implemented. Each case study provides valuable insights into the development process."
                            </p>
                        </div>
                    </div>
                </div>
            </section>

            // Development Journey
            <section class="py-12 sm:py-20">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <h2 class="text-4xl font-bold text-center mb-12">"Development Journey"</h2>
                    <div class="max-w-4xl mx-auto">
                        <div class="relative">
                            <div class="absolute left-8 top-0 bottom-0 w-0.5 border-l"></div>
                            <div class="space-y-12">
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">"Initial Setup"</h3>
                                        <p>
                                            "Started with a basic Leptos setup and integrated Tailwind CSS. Established the core layout and navigation structure, focusing on mobile responsiveness and clean design."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">"Case Studies"</h3>
                                        <p>
                                            "Developed the case studies section with a focus on detailed project documentation. Implemented a consistent structure for presenting technical information and project outcomes."
                                        </p>
                                    </div>
                                </div>
                                <div class="relative flex items-start">
                                    <div class="absolute left-6 top-1/2 -translate-y-1/2 w-4 h-4 rounded-full border-4 bg-white"></div>
                                    <div class="ml-16 rounded-lg p-6 border bg-white shadow-sm">
                                        <h3 class="text-xl font-bold mb-2">"Future Plans"</h3>
                                        <p>
                                            "Planning to add a blog section for sharing technical insights and experiences. Also considering adding interactive elements to case studies and improving the overall user experience."
                                        </p>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </section>
        </div>
    }
}
