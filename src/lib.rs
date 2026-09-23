mod engine;
mod ui;

use wasm_bindgen::prelude::*;
use leptos::*;
use ui::App;

/// WASM entry point — called by the Trunk-generated JS bootstrap.
#[wasm_bindgen(start)]
pub fn main() {
    // Pipe panics to the browser console instead of silently swallowing them
    console_error_panic_hook::set_once();

    // Mount the Leptos app into <div id="app">
    mount_to_body(|| view! { <App /> });
}

// ── Optional: public WASM API for testing from JS ─────────────────────────────

/// Run a search from JavaScript. Accepts and returns JSON strings.
/// 
/// ```js
/// const results = glassSearch(
///   JSON.stringify({ joy:{lo:0.6,hi:1.0}, ... }),  // RangeQuery JSON
/// );
/// console.log(JSON.parse(results));
/// ```
#[wasm_bindgen]
pub fn glass_search(query_json: &str) -> String {
    use engine::{Catalog, RangeQuery};

    let catalog = Catalog::load();
    let query: RangeQuery = match serde_json::from_str(query_json) {
        Ok(q) => q,
        Err(e) => return format!("{{\"error\": \"{e}\"}}"),
    };
    let results = catalog.search(&query);
    serde_json::to_string(&results).unwrap_or_else(|e| format!("{{\"error\": \"{e}\"}}"))
}
