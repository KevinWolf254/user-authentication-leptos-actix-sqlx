use leptos::*;
use leptos_meta::*;
use leptos_router::*;

pub mod page;
use page::{navigation_page::NavigationPage, user::{user_list_page::UserListPage, user_profile_page::UserProfilePage}};
pub mod component;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/user-authentication-leptos-actix-sqlx.css"/>
        <script src="/assets/bootstrap.bundle.min.js"></script>

        // sets the document title
        <Title text="Welcome to Leptos"/>

        // content for this welcome page
        <Router>
        
            <main>
                <Routes>
                    <Route path="" view=NavigationPage>
                        <Route path="/users" view=UserListPage/>
                        <Route path="/users/:id" view=UserProfilePage/>
                    </Route>
                    // <Route path="/*any" view=NotFoundPage/>
                </Routes>
            </main>
        </Router>
    }
}
