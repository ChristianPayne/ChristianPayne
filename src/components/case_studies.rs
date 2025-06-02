use leptos::prelude::*;
use thaw::*;

#[derive(Clone)]
pub struct CaseStudy {
    pub title: String,
    pub description: String,
    pub image_path: String,
    pub slug: String, // URL-friendly version of title
}

#[component]
pub fn CaseStudies() -> impl IntoView {
    let case_studies = vec![
        CaseStudy {
            title: "E-commerce Platform Redesign".to_string(),
            description: "A comprehensive redesign of a major e-commerce platform that improved user experience and increased conversion rates by 35%. This project involved user research, prototyping, and extensive A/B testing to optimize the checkout flow and product discovery features.".to_string(),
            image_path: "images/example-twitch.jpg".to_string(),
            slug: "ennesults".to_string(),
        },
        CaseStudy {
            title: "AI-Powered Analytics Dashboard".to_string(), 
            description: "Development of a sophisticated analytics dashboard that leverages machine learning to provide actionable insights for business users. The solution reduced data analysis time by 80% and improved decision-making across multiple departments.".to_string(),
            image_path: "images/analytics-dashboard.jpg".to_string(),
            slug: "".to_string(),
        },
        CaseStudy {
            title: "Mobile App for Healthcare".to_string(),
            description: "A mobile application designed to streamline patient care and improve communication between healthcare providers. The app increased patient satisfaction scores by 25% and reduced administrative overhead by 40%.".to_string(),
            image_path: "images/healthcare-app.jpg".to_string(),
            slug: "".to_string(),
        },
        CaseStudy {
            title: "Enterprise System Integration".to_string(),
            description: "A complex integration project that connected multiple legacy systems with modern cloud-based solutions. This initiative improved data consistency and reduced operational costs by 30% while maintaining 99.9% uptime.".to_string(),
            image_path: "images/enterprise-integration.jpg".to_string(),
            slug: "".to_string(),
        },
    ];

    view! {
        <div class="container mx-auto max-w-7xl">
            <div class="my-8">
                <h1 class="text-3xl md:text-4xl font-bold text-gray-800 mb-8 text-center">
                    Case Studies
                </h1>
                <p class="text-lg text-gray-600 text-center mb-12 max-w-3xl mx-auto">
                    "Explore detailed insights into my recent projects, showcasing the challenges faced, solutions implemented, and results achieved."
                </p>

                <div class="space-y-16">
                    {case_studies
                        .iter()
                        .enumerate()
                        .map(|(index, case_study)| {
                            let is_even = index % 2 == 0;
                            let title = case_study.title.clone();
                            let description = case_study.description.clone();
                            let image_path = case_study.image_path.clone();
                            let slug = case_study.slug.clone();

                            view! {
                                <Link href=format!("/case-studies/{}", slug)>
                                    <Button
                                        appearance=ButtonAppearance::Transparent
                                        class="w-full p-0 h-auto group relative overflow-hidden transition-all duration-300 ease-in-out hover:bg-gray-50"
                                    >
                                        <div class="w-full relative">
                                            {if is_even {
                                                // Image on left, content on right
                                                view! {
                                                    <div class="flex flex-col lg:flex-row items-center gap-8 lg:gap-12 py-8">
                                                        <div class="w-full lg:w-1/2">
                                                            <div class="overflow-hidden rounded-lg shadow-lg">
                                                                <img
                                                                    src=image_path
                                                                    alt=format!("{} case study image", title)
                                                                    class="w-full h-64 lg:h-80 object-cover transition-transform duration-300 group-hover:scale-105"
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="w-full lg:w-1/2 text-left">
                                                            <h2 class="text-2xl lg:text-3xl font-bold text-gray-800 mb-4 transition-colors duration-300 group-hover:text-blue-600">
                                                                {title}
                                                            </h2>
                                                            <p class="text-lg text-gray-600 leading-relaxed mb-6">
                                                                {description}
                                                            </p>
                                                            <div class="inline-flex items-center text-blue-600 font-medium transition-all duration-300 group-hover:text-blue-700 group-hover:translate-x-2 relative">
                                                                "View Case Study"
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
                                                                // Animated underline for "View Case Study"
                                                                </svg>
                                                                <div class="absolute bottom-0 left-0 w-0 h-0.5 bg-blue-600 transition-all duration-300 ease-in-out group-hover:w-full"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            } else {
                                                // Image on right, content on left
                                                view! {
                                                    <div class="flex flex-col lg:flex-row-reverse items-center gap-8 lg:gap-12 py-8">
                                                        <div class="w-full lg:w-1/2">
                                                            <div class="overflow-hidden rounded-lg shadow-lg">
                                                                <img
                                                                    src=image_path
                                                                    alt=format!("{} case study image", title)
                                                                    class="w-full h-64 lg:h-80 object-cover transition-transform duration-300 group-hover:scale-105"
                                                                />
                                                            </div>
                                                        </div>
                                                        <div class="w-full lg:w-1/2 text-left">
                                                            <h2 class="text-2xl lg:text-3xl font-bold text-gray-800 mb-4 transition-colors duration-300 group-hover:text-blue-600">
                                                                {title}
                                                            </h2>
                                                            <p class="text-lg text-gray-600 leading-relaxed mb-6">
                                                                {description}
                                                            </p>
                                                            <div class="inline-flex items-center text-blue-600 font-medium transition-all duration-300 group-hover:text-blue-700 group-hover:translate-x-2 relative">
                                                                "View Case Study"
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
                                                                // Animated underline for "View Case Study"
                                                                </svg>
                                                                <div class="absolute bottom-0 left-0 w-0 h-0.5 bg-blue-600 transition-all duration-300 ease-in-out group-hover:w-full"></div>
                                                            </div>
                                                        </div>
                                                    </div>
                                                }
                                            }}
                                        </div>
                                    </Button>
                                </Link>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}
