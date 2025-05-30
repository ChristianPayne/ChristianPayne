use crate::components::{CaseStudies, Ennesults, Home};
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Router>
                <Layout>
                    <LayoutHeader class="p-4">
                        <div class="flex flex-col md:flex-row md:justify-between md:items-center gap-4">
                            <div class="flex justify-center md:justify-start">
                                <Link href="/">
                                    <Button appearance=ButtonAppearance::Transparent>
                                        <h1 class="text-2xl font-medium">Christian Payne</h1>
                                    </Button>
                                </Link>
                            </div>

                            <div class="flex flex-wrap gap-4 justify-center md:justify-end">
                                <Link href="/ennesults">
                                    <Button appearance=ButtonAppearance::Transparent>
                                        "Ennesults"
                                    </Button>
                                </Link>
                                <Link href="/case-studies">
                                    <Button appearance=ButtonAppearance::Transparent>
                                        "Case Studies"
                                    </Button>
                                </Link>
                                <Link href="https://christianpayne.substack.com">
                                    <Button appearance=ButtonAppearance::Transparent>
                                        "Latent Space"
                                    </Button>
                                </Link>
                            </div>
                        </div>
                    </LayoutHeader>
                    <Divider />

                    <Layout class="px-4">
                        <Routes fallback=|| view! { <div>"Not Found"</div> }>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/ennesults") view=Ennesults />
                            <Route path=path!("/case-studies") view=CaseStudies />
                        </Routes>
                    </Layout>
                </Layout>
            </Router>
        </ConfigProvider>
    }
}
