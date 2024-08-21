use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod page;
use page::{home_page::HomePage, not_found_page::NotFoundPage};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/node_modules/bootstrap/dist/js/bootstrap.bundle.min.js")]
extern "C" {
    fn load_bootstrap();
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(module = "/node_modules/bootstrap/dist/js/bootstrap.bundle.min.js.map")]
extern "C" {
    fn load_bootstrap_map();
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    #[cfg(target_arch = "wasm32")]
    {
        // Load Bootstrap when the component is mounted
        load_bootstrap();
    }

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/user-authentication-leptos-actix-sqlx.css"/>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFoundPage/>
                </Routes>
            </main>
        </Router>
    }
}
