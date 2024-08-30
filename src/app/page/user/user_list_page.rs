use std::time::Duration;
use std::cmp::{min, max};

use leptos::*;
use leptos::set_timeout_with_handle;

#[component]
pub fn UserListPage() -> impl IntoView {
    
    use crate::repository::user_repository::find_paginated;
    
    let (page_size, set_page_size) = create_signal::<i64>(10);
    let (page, set_page) = create_signal::<i64>(1);
    let (total_users, set_total_users) = create_signal::<i64>(0);
    let (email_address, set_email_address) = create_signal::<Option<String>>(None);
    let (typing_timeout, _) = create_signal::<Option<i32>>(None);

    let pagination = create_resource( move || (page.get(), page_size.get(), email_address.get()), |value| find_paginated(value.0, value.1, value.2));

    view! {
        <section id="users">
            <div class="container">
                <div class="d-flex flex-row-reverse my-2">
                    <button type="button" class="btn btn-primary d-flex align-items-center"> // (click)="openCreate()"
                        <i class="bi bi-person-add" style="font-size: 1.4rem; margin-right: 0.5rem;"></i> 
                        <span style="font-size: 1.1rem;"> "Create" </span>
                    </button>
                </div>
                <div class="card">
                    <div class="card-body">
                        <form autocomplete="off">
                            <div class="d-flex justify-content-between">
                                <select id="selectPageSize" class="form-select" aria-label="Select page size" style="width: 5rem;"
                                formControlName="pageSize" on:change=move |ev| {
                                    let value = event_target_value(&ev);  // Get the value from the event
                                    set_page_size.set(value.parse().unwrap_or(10));  // Set the signal, default to 10 if parsing fails
                                }>
                                    <option value="10" selected={move || page_size.get() == 10}>{"10"}</option>
                                    <option value="25" selected={move || page_size.get() == 25}>{"25"}</option>
                                    <option value="50" selected={move || page_size.get() == 50}>{"50"}</option>
                                </select>
                                // TODO - update with the latest value
                                <input id="search" class="form-control" type="text" placeholder="Search email address"
                                    aria-label="email address" formControlName="email" style="max-width: 25rem;" on:input=move |ev| {
                                    let value = event_target_value(&ev);

                                    // Clear the previous timeout if any
                                    if let Some(timeout_id) = typing_timeout.get() {
                                        web_sys::window().unwrap().clear_timeout_with_handle(timeout_id);
                                    }

                                    // Set a new timeout
                                    _ = set_timeout_with_handle(
                                        move || {                                    
                                            // logging::log!("input: {:?}", latest_value);
                                            set_email_address.update(|v| {
                                                if value.trim().is_empty() {
                                                    *v = None;
                                                } else {
                                                    *v = Some(value.trim().to_owned());
                                                }
                                            });
                                        },
                                        Duration::from_secs(1) // 1 second debounce
                                    );                                
                                }/>
                            </div>
                        </form>

                        <div class="table-responsive">
                            <table class="table table-hover table-striped">
                            <thead>
                                <tr>
                                <th scope="col">"#"</th>
                                <th scope="col">"Names"</th>
                                <th scope="col">"Email Address"</th>
                                <th scope="col">"Enabled"</th>
                                <th scope="col">"Role"</th>
                                <th scope="col">"Created"</th>
                                <th scope="col">"Actions"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <Suspense fallback=move || view! { 
                                    <tr>
                                        <td colspan="7">
                                            <div class="lds-roller" style="position: relative; left: 50%;">
                                            <div></div>
                                            <div></div>
                                            <div></div>
                                            <div></div>
                                            <div></div>
                                            <div></div>
                                            <div></div>
                                            <div></div>
                                            </div>
                                        </td>
                                    </tr>
                                    }.into_view()>
                                    {move ||  {
                                        match pagination.get() {
                                            Some(Ok(result)) => {
                                                set_page_size.update(|p_size| *p_size = result.page_size);
                                                set_page.update(|p| *p = result.page);
                                                set_total_users.update(|total| *total = result.total);

                                                result.data.into_iter().map(|user| view! {
                                                    <tr> 
                                                        <td>
                                                        { user.user_id }.
                                                        </td>
                                                        <td>
                                                            <span>
                                                            {user.name}
                                                            </span>
                                                        </td>
                                                        <td>
                                                            <span>
                                                            { user.email_address }
                                                            </span>
                                                        </td>
                                                        <td> 
                                                            {
                                                                if user.email_confirmed {
                                                                    view! { <i class="bi bi-toggle-on" style="color: green; font-size: 2rem;"></i> }.into_view()
                                                                } else {
                                                                    view! { <i class="bi bi-toggle-on" style="color: red; font-size: 2rem;"></i> }.into_view()
                                                                }
                                                            }
                                                        </td>
                                                        <td>
                                                            <span class="badge rounded-pill bg-info text-dark p-2" style="min-width: 4rem;">
                                                                { user.role }
                                                            </span>
                                                        </td>
                                                        <td>
                                                            <span>
                                                            { user.created_at.to_string() }
                                                            </span>
                                                        </td>
                                                        <td>
                                                            <div class="d-flex align-items-center">
                                                                <button type="button" class="btn btn-action btn-outline-info me-2">// (click)="openEdit(user)"
                                                                    <i class="bi bi-pen-fill"></i>
                                                                </button>
                                                                <button type="button" class="btn btn-action btn-outline-danger me-2">// (click)="openDelete(user)"
                                                                    <i class="bi bi-trash"></i>
                                                                </button>
                                                            </div>
                                                        </td>
                                                    </tr>
                                                }).collect_view()
                                            },
                                            Some(Err(e)) => view! { <tr><td colspan="7">{format!("Error occurred: {:?}", e.to_string()) }</td></tr> }.into_view(),
                                            None => view! {
                                                <tr>
                                                    <td colspan="7">
                                                        <div class="lds-roller" style="position: relative; left: 50%;">
                                                        <div></div>
                                                        <div></div>
                                                        <div></div>
                                                        <div></div>
                                                        <div></div>
                                                        <div></div>
                                                        <div></div>
                                                        <div></div>
                                                        </div>
                                                    </td>
                                                </tr>
                                            }.into_view(),
                                        }
                                    }}
                                </Suspense>
                            </tbody>
                            </table>
                        </div>
                        <div class="d-flex flex-row-reverse mt-2">
                            <Suspense fallback=move || view! { 
                                <nav aria-label="Page navigation">
                                    <ul class="pagination">
                                        <li class="page-item disabled"><a class="page-link" href="#" tabindex="-1" aria-disabled="true">Previous</a></li>
                                        <li class="page-item disabled"><a class="page-link" href="#" tabindex="-1" aria-disabled="true">Next</a></li>
                                    </ul>
                                </nav>
                            }.into_view()>
                                <nav aria-label="Page navigation example">
                                    <ul class="pagination">
                                        <li class="page-item">
                                            <a class="page-link" href="#" on:click=move |_| {
                                                let previous_page = max(1, page.get() - 1);
                                                if previous_page >= 1 {
                                                    set_page.update(|p| *p = previous_page);
                                                }
                                            }>
                                                "Previous"
                                            </a>
                                        </li>
                                        { move || {
                                            let _total_pages = (total_users.get() as f64 / page_size.get() as f64).ceil() as i64;
                                            // logging::log!("total_pages: {}", total_pages);
                                            
                                            // TODO - find solution to the bug - https://book.leptos.dev/ssr/24_hydration_bugs.html
                                            // (1..=total_pages).map(move |i| {
                                                // let active_class = if i == page.get() { "active" } else { "" };
                                                // view! {
                                        //             <li id={i.to_string()} class={format!("page-item {}", active_class)}>
                                        //                 <a class="page-link" href="#" on:click=move |_| {
                                        //                     set_page.update(|p| *p = i);
                                        //                 }>
                                        //                     {i.to_string()}
                                        //                 </a>
                                        //             </li>
                                                // }
                                            // }).collect_view()
                                            }
                                        }
                                        <li class="page-item">
                                            <a class="page-link" href="#" on:click=move |_| {
                                                let total_pages = (total_users.get() as f64 / page_size.get() as f64).ceil() as i64;
                                                let next_page = min(total_users.get(), page.get() + 1);
                                                if next_page <= total_pages {
                                                    set_page.update(|p| *p = next_page);
                                                }
                                            }>
                                                "Next"
                                            </a>
                                        </li>
                                    </ul>
                                </nav>
                            </Suspense>
                        </div>
                    </div>
                </div>        
            </div>
        </section>
    }
}