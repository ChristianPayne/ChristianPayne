use crate::components::ennesults::Ennesults;
use crate::components::home::Home;
use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::hooks::use_params_map;
use leptos_router::path;
use thaw::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <ConfigProvider>
            <Router>
                <Layout>
                    <LayoutHeader class="p-4 flex justify-between items-center">
                        <Link href="/">
                            <Button appearance=ButtonAppearance::Transparent>
                                <h1 class="text-2xl font-medium">Christian Payne</h1>
                            </Button>
                        </Link>

                        <div class="flex gap-4">
                            <Link href="/ennesults">
                                <Button appearance=ButtonAppearance::Transparent>
                                    "Ennesults"
                                </Button>
                            </Link>
                            <Link href="https://christianpayne.substack.com">
                                <Button appearance=ButtonAppearance::Transparent>
                                    "Latent Space"
                                </Button>
                            </Link>
                        </div>
                    </LayoutHeader>
                    <Divider />

                    <Layout class="p-4">
                        <Routes fallback=|| view! { <div>"Not Found"</div> }>
                            <Route path=path!("/") view=Home />
                            <Route path=path!("/ennesults") view=Ennesults />
                        </Routes>
                    </Layout>
                </Layout>
            </Router>
        </ConfigProvider>
    }
}
