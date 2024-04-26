pub mod app;
pub mod error_template;
#[cfg(feature = "ssr")]
pub mod fileserv;

// UI components
pub mod components;
pub mod pages;

// Functionality
pub mod api;
leptos_i18n::load_locales!();

// Integration

#[cfg(feature = "ssr")]
use sea_orm::DatabaseConnection;

#[cfg(feature = "ssr")]
#[derive(Clone, Debug)]
pub struct AppState {
    pub conn: DatabaseConnection,
    pub secret_key: String,
}

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
