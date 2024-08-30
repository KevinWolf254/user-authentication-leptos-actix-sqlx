use serde::{Deserialize, Serialize};

pub mod app;
mod entity;
mod repository;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    use leptos::*;

    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub page_size: i64
}

#[cfg(feature = "ssr")]
pub struct CountResult {
    pub count: Option<i64>
}