use crate::components::{
    CaseStudies, EnnesultsCaseStudy, Home, ObsScheduledRecordingsCaseStudy,
    PermissionSystemCaseStudy, WebsiteCaseStudy,
};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use std::collections::HashMap;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    let brand_colors = RwSignal::new(HashMap::from([
        (10, "#060201"),
        (20, "#23120B"),
        (30, "#3C1C12"),
        (40, "#512316"),
        (50, "#662A1A"),
        (60, "#7C311E"),
        (70, "#933822"),
        (80, "#AB3F26"),
        (90, "#C34629"),
        (100, "#DB4D2D"),
        (110, "#F45431"),
        (120, "#FF6A45"),
        (130, "#FF8665"),
        (140, "#FF9F82"),
        (150, "#FFB59F"),
        (160, "#FFCABE"),
    ]));

    let theme = RwSignal::new(Theme::custom_light(&brand_colors.get()));

    // Add state for mobile menu
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);

    view! {
        <ConfigProvider theme>
            <Router>
                <Layout class="background-grid font-sans">
                    <LayoutHeader class="p-4 bg-white relative">
                        <div class="flex flex-col md:flex-row md:justify-between md:items-center gap-4">
                            <div class="flex justify-between items-center">
                                <Link href="/">
                                    <Button appearance=ButtonAppearance::Transparent>
                                        <h1 class="text-2xl">Christian Payne</h1>
                                    </Button>
                                </Link>

                                // Add hamburger button for mobile
                                <Button
                                    appearance=ButtonAppearance::Transparent
                                    class="md:hidden"
                                    on:click=move |_| set_mobile_menu_open.update(|v| *v = !*v)
                                >
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
                                            d="M4 6h16M4 12h16M4 18h16"
                                        ></path>
                                    </svg>
                                </Button>
                            </div>

                            // Mobile menu container with transitions
                            <div class=move || {
                                let base_classes = "absolute md:relative left-0 right-0 top-full bg-white md:shadow-none shadow-lg transition-all duration-300 ease-in-out z-50";
                                let mobile_classes = if is_mobile_menu_open.get() {
                                    "opacity-100 translate-y-0 md:translate-y-0"
                                } else {
                                    "opacity-0 -translate-y-4 pointer-events-none md:opacity-100 md:translate-y-0 md:pointer-events-auto"
                                };
                                format!("{} {}", base_classes, mobile_classes)
                            }>
                                // Navigation items
                                <div class="flex flex-col md:flex-row gap-4 p-4 md:p-0">
                                    <Link href="/case-studies">
                                        <Button
                                            appearance=ButtonAppearance::Transparent
                                            on_click=move |_| {
                                                set_mobile_menu_open.update(|v| *v = false)
                                            }
                                        >
                                            "Case Studies"
                                        </Button>
                                    </Link>
                                    <a
                                        href="https://christianpayne.substack.com"
                                        target="_blank"
                                        rel="noopener noreferrer"
                                    >
                                        <Button
                                            appearance=ButtonAppearance::Transparent
                                            class="inline-flex items-center"
                                        >
                                            "Latent Space"
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
                                                    d="M10 6H6a2 2 0 00-2 2v10a2 2 0 002 2h10a2 2 0 002-2v-4M14 4h6m0 0v6m0-6L10 14"
                                                ></path>
                                            </svg>
                                        </Button>
                                    </a>
                                </div>
                            </div>
                        </div>
                    </LayoutHeader>
                    <Divider />

                    <Layout class="px-4">
                        <Routes fallback=|| view! { <div>"Not Found"</div> }>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/case-studies") view=CaseStudies />
                            <Route path=path!("/case-studies/ennesults") view=EnnesultsCaseStudy />
                            <Route
                                path=path!("/case-studies/permission-system")
                                view=PermissionSystemCaseStudy
                            />
                            <Route path=path!("/case-studies/website") view=WebsiteCaseStudy />
                            <Route
                                path=path!("/case-studies/obs-scheduled-recordings")
                                view=ObsScheduledRecordingsCaseStudy
                            />
                        </Routes>
                    </Layout>
                </Layout>
            </Router>
        </ConfigProvider>
    }
}
