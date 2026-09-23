use leptos::*;

#[component]
pub fn Timeline(
    year_bounds:    (u16, u16),
    year_range:     ReadSignal<(u16, u16)>,
    set_year_range: WriteSignal<(u16, u16)>,
) -> impl IntoView {
    let (min_year, max_year) = year_bounds;

    let on_lo = move |ev: web_sys::Event| {
        let v: u16 = event_target_value(&ev).parse().unwrap_or(min_year);
        set_year_range.update(|(lo, hi)| *lo = v.min(*hi));
    };

    let on_hi = move |ev: web_sys::Event| {
        let v: u16 = event_target_value(&ev).parse().unwrap_or(max_year);
        set_year_range.update(|(lo, hi)| *hi = v.max(*lo));
    };

    let lo_label = move || year_range().0.to_string();
    let hi_label = move || year_range().1.to_string();

    view! {
        <div class="ge-timeline">
            <div class="ge-timeline-header">
                <span class="ge-panel-title">"Timeline"</span>
                <span class="ge-timeline-range">
                    {lo_label} " – " {hi_label}
                </span>
            </div>
            <div class="ge-slider-track ge-timeline-track">
                <div
                    class="ge-slider-fill"
                    style=move || {
                        let span = (max_year - min_year) as f32;
                        let (lo, hi) = year_range();
                        let left  = (lo - min_year) as f32 / span * 100.0;
                        let width = (hi - lo) as f32 / span * 100.0;
                        format!("left: {left:.1}%; width: {width:.1}%")
                    }
                />
                <input
                    type="range"
                    min={min_year.to_string()} max={max_year.to_string()} step="1"
                    class="ge-range ge-range-lo"
                    prop:value=move || year_range().0
                    on:input=on_lo
                />
                <input
                    type="range"
                    min={min_year.to_string()} max={max_year.to_string()} step="1"
                    class="ge-range ge-range-hi"
                    prop:value=move || year_range().1
                    on:input=on_hi
                />
            </div>
        </div>
    }
}
