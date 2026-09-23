use leptos::*;
use crate::engine::ScoredTrack;

#[component]
pub fn ResultsList(results: Signal<Vec<ScoredTrack>>) -> impl IntoView {
    view! {
        <div class="ge-results">
            <Show
                when=move || results().is_empty()
                fallback=move || view! {
                    <ul class="ge-track-list">
                        <For
                            each=move || results()
                            key=|st| st.track.id
                            children=|st| view! { <TrackCard scored=st /> }
                        />
                    </ul>
                }
            >
                <p class="ge-empty">"No pieces match the current filters."</p>
            </Show>
        </div>
    }
}

// ── TrackCard ─────────────────────────────────────────────────────────────────

#[component]
fn TrackCard(scored: ScoredTrack) -> impl IntoView {
    let track = scored.track.clone();
    let score = scored.score;

    // Score bar width as a percentage
    let bar_width = format!("{:.1}%", score * 100.0);
    let score_pct  = format!("{:.0}%", score * 100.0);

    view! {
        <li class="ge-track">
            <div class="ge-track-meta">
                <span class="ge-track-title">{track.title.clone()}</span>
                <span class="ge-track-info">
                    {track.year.to_string()} " · " {track.genre.label()} " · " {track.duration_str()}
                </span>
            </div>

            // Similarity bar
            <div class="ge-score-bar-bg">
                <div class="ge-score-bar" style=format!("width: {bar_width}") />
                <span class="ge-score-label">{score_pct}</span>
            </div>

            // Dimensional fingerprint — a mini radar-like dot row
            <div class="ge-dims">
                <DimDot label="J" value=track.dims.joy />
                <DimDot label="S" value=track.dims.sorrow />
                <DimDot label="I" value=track.dims.intensity />
                <DimDot label="D" value=track.dims.density />
                <DimDot label="V" value=track.dims.velocity />
            </div>

            {track.note.as_ref().map(|n| view! {
                <p class="ge-track-note">{n.clone()}</p>
            })}
        </li>
    }
}

/// Small visual indicator: a labeled dot scaled to the dimension value.
#[component]
fn DimDot(label: &'static str, value: f32) -> impl IntoView {
    let size = format!("{}px", (6.0 + value * 14.0) as u32);
    let opacity = format!("{:.2}", 0.3 + value * 0.7);
    view! {
        <span class="ge-dim-dot" title=format!("{}: {:.0}%", label, value * 100.0)>
            <span class="ge-dim-label">{label}</span>
            <span class="ge-dot" style=format!("width:{size};height:{size};opacity:{opacity}") />
        </span>
    }
}
