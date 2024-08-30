use leptos::*;
use leptos_router::Outlet;

/// Renders the home page of your application.
#[component]
pub fn NavigationPage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (collapse, set_collapse) = create_signal(false);
    let toggle_collapse = move |_| set_collapse.update(|c| { *c = !*c; } );

    view! {
        <div class="wrapper">
            // <!-- Sidebar -->
            <aside id="sidebar" class:collapsed=collapse>
                <div class="h-100">
                    <div class="sidebar-logo">
                        <a href="#">"CodzSword"</a>
                    </div>
                    // <!-- Sidebar Navigation -->
                    <ul class="sidebar-nav">
                        <li class="sidebar-header">
                            "Administration"
                        </li>
                        <li class="sidebar-item">
                            <a href="users" class="sidebar-link">
                                <i class="bi bi-people-fill pe-2"></i>
                                "Users"
                            </a>
                        </li>
                    </ul>
                    <ul class="sidebar-nav">
                        <li class="sidebar-header">
                            "Tools & Components"
                        </li>
                        <li class="sidebar-item">
                            <a href="#" class="sidebar-link">
                                <i class="bi bi-list-task pe-2"></i>
                                "Profile"
                            </a>
                        </li>
                        <li class="sidebar-item">
                            <a href="#" class="sidebar-link collapsed" data-bs-toggle="collapse" data-bs-target="#pages"
                                aria-expanded="false" aria-controls="pages">
                                <i class="bi bi-file-earmark-text pe-2"></i>
                                "Pages"
                            </a>
                            <ul id="pages" class="sidebar-dropdown collapse ml-3" data-bs-parent="#sidebar">
                                <li class="sidebar-item ">
                                    <a href="#" class="sidebar-link sub-item">"Analytics"</a>
                                </li>
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item">"Ecommerce"</a>
                                </li>
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item">"Crypto"</a>
                                </li>
                            </ul>
                        </li>
                        <li class="sidebar-item">
                            <a href="#" class="sidebar-link collapsed" data-bs-toggle="collapse" data-bs-target="#dashboard"
                                aria-expanded="false" aria-controls="dashboard">
                                <i class="bi bi-sliders pe-2"></i>
                                "Dashboard"
                            </a>
                            <ul id="dashboard" class="sidebar-dropdown collapse" data-bs-parent="#sidebar">
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item">"Dashboard Analytics"</a>
                                </li>
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item">"Dashboard Ecommerce"</a>
                                </li>
                            </ul>
                        </li>
                        <li class="sidebar-item">
                            <a href="#" class="sidebar-link collapsed" data-bs-toggle="collapse" data-bs-target="#auth"
                                aria-expanded="false" aria-controls="auth">
                                <i class="bi bi-person pe-2"></i>
                                "Auth"
                            </a>
                            <ul id="auth" class="sidebar-dropdown collapse" data-bs-parent="#sidebar">
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item">"Login"</a>
                                </li>
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item">"Register"</a>
                                </li>
                            </ul>
                        </li>
                        <li class="sidebar-header">
                            "Multi Level Nav"
                        </li>
                        <li class="sidebar-item">
                            <a href="#" class="sidebar-link collapsed" data-bs-toggle="collapse" data-bs-target="#multi"
                                aria-expanded="false" aria-controls="multi">
                                <i class="bi bi-diagram-3-fill pe-2"></i>
                                "Multi Level"
                            </a>
                            <ul id="multi" class="sidebar-dropdown collapse" data-bs-parent="#sidebar">
                                <li class="sidebar-item">
                                    <a href="#" class="sidebar-link sub-item collapsed" data-bs-toggle="collapse"
                                        data-bs-target="#multi-two" aria-expanded="false" aria-controls="multi-two">
                                        "Two Links"
                                    </a>
                                    <ul id="multi-two" class="sidebar-dropdown collapse">
                                        <li class="sidebar-item">
                                            <a href="#" class="sidebar-link sub-item">"Link 1"</a>
                                        </li>
                                        <li class="sidebar-item">
                                            <a href="#" class="sidebar-link sub-item">"Link 2"</a>
                                        </li>
                                    </ul>
                                </li>
                            </ul>
                        </li>
                    </ul>
                </div>
            </aside>
            // <!-- Main Component -->
            <div class="main">
                <main class="content">
                    <nav class="navbar navbar-expand-lg navbar-light bg-light">
                        <div class="container-fluid">
                            <button class="btn btn-outline-secondary px-2 py-0" on:click=toggle_collapse>
                                <i class="bi bi-list" style="font-size: 1.5rem;"></i>
                            </button>
                            // <a class="navbar-brand" href="#">Navbar</a>
                            <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarSupportedContent" aria-controls="navbarSupportedContent" aria-expanded="false" aria-label="Toggle navigation">
                                <span class="navbar-toggler-icon"></span>
                            </button>
                            <div class="collapse navbar-collapse" id="navbarSupportedContent">
                                <ul class="navbar-nav me-auto mb-2 mb-lg-0">
                                //     <li class="nav-item">
                                //         <a class="nav-link active" aria-current="page" href="#">Home</a>
                                //     </li>
                                //     <li class="nav-item">
                                //         <a class="nav-link" href="#">Link</a>
                                //     </li>
                                //     <li class="nav-item dropdown">
                                //         <a class="nav-link dropdown-toggle" href="#" id="navbarDropdown" role="button" data-bs-toggle="dropdown" aria-expanded="false">
                                //         Dropdown
                                //         </a>
                                //         <ul class="dropdown-menu" aria-labelledby="navbarDropdown">
                                //             <li><a class="dropdown-item" href="#">Action</a></li>
                                //             <li><a class="dropdown-item" href="#">Another action</a></li>
                                //             <li><hr class="dropdown-divider"/></li>
                                //             <li><a class="dropdown-item" href="#">Something else here</a></li>
                                //         </ul>
                                //     </li>
                                //     <li class="nav-item">
                                //         <a class="nav-link disabled" href="#" tabindex="-1" aria-disabled="true">Disabled</a>
                                //     </li>
                                </ul>
                                <form class="d-flex">
                                    <input class="form-control me-2" type="search" placeholder="Search" aria-label="Search"/>
                                    <button class="btn btn-outline-success" type="submit">Search</button>
                                </form>
                            </div>
                        </div>
                    </nav>
                    <div class="mb-3">
                        // <h3>Bootstrap Sidebar Tutorial</h3>
                        <Outlet/>
                    </div>
                </main>
            </div>
        </div>
    }
}