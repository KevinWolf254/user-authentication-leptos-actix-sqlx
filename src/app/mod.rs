use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod page;
use page::{home_page::HomePage, not_found_page::NotFoundPage};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

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
