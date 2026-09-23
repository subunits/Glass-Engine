use leptos::*;
use crate::engine::{Catalog, DimRange, Dimensions, Genre, RangeQuery};
use super::{sliders::SliderPanel, results::ResultsList, timeline::Timeline};

#[component]
pub fn App() -> impl IntoView {
    // Load catalog once at startup (it's compile-time embedded)
    let catalog = store_value(Catalog::load());

    // ── Query state (drives everything reactively) ────────────────────────
    let (joy,       set_joy)       = create_signal(DimRange::full());
    let (sorrow,    set_sorrow)    = create_signal(DimRange::full());
    let (intensity, set_intensity) = create_signal(DimRange::full());
    let (density,   set_density)   = create_signal(DimRange::full());
    let (velocity,  set_velocity)  = create_signal(DimRange::full());
    let (year_range, set_year_range) = create_signal((1960u16, 2030u16));
    let (genre_filter, set_genre_filter) = create_signal::<Option<Genre>>(None);

    // Derived: build a RangeQuery from current signal values
    let query = move || RangeQuery {
        joy:          joy(),
        sorrow:       sorrow(),
        intensity:    intensity(),
        density:      density(),
        velocity:     velocity(),
        year_range:   year_range(),
        genre_filter: genre_filter(),
        top_k:        12,
        weights:      Dimensions::UNIT_WEIGHT,
    };

    // Derived: search results, re-computed whenever any signal changes
    let results = move || catalog.get_value().search(&query());

    // ── Year bounds from the catalog ──────────────────────────────────────
    let year_bounds = catalog.get_value().year_bounds();

    view! {
        <div class="glass-engine">
            <header class="ge-header">
                <h1>"Glass Engine"</h1>
                <p class="ge-subtitle">
                    "Navigate music through joy, sorrow, intensity, density & velocity."
                </p>
            </header>

            <main class="ge-main">
                <aside class="ge-sidebar">
                    // ── Emotional dimension sliders ──────────────────────
                    <SliderPanel
                        joy=joy set_joy=set_joy
                        sorrow=sorrow set_sorrow=set_sorrow
                        intensity=intensity set_intensity=set_intensity
                        density=density set_density=set_density
                        velocity=velocity set_velocity=set_velocity
                    />

                    // ── Genre filter ────────────────────────────────────
                    <div class="ge-genre-filter">
                        <label>"Genre"</label>
                        <select on:change=move |ev| {
                            let val = event_target_value(&ev);
                            set_genre_filter(if val == "all" { None } else {
                                serde_json::from_str(&format!("\"{}\"", val)).ok()
                            });
                        }>
                            <option value="all">"All genres"</option>
                            {Genre::all().iter().map(|g| {
                                let key = serde_json::to_string(g)
                                    .unwrap_or_default()
                                    .trim_matches('"')
                                    .to_owned();
                                view! { <option value={key}>{g.label()}</option> }
                            }).collect_view()}
                        </select>
                    </div>
                </aside>

                <section class="ge-content">
                    // ── Year timeline ────────────────────────────────────
                    <Timeline
                        year_bounds=year_bounds
                        year_range=year_range
                        set_year_range=set_year_range
                    />

                    // ── Results ──────────────────────────────────────────
                    <ResultsList results=Signal::derive(results) />
                </section>
            </main>
        </div>
    }
}
