use leptos::*;
use crate::engine::DimRange;

#[component]
fn DualSlider(
    label: &'static str,
    range:     ReadSignal<DimRange>,
    set_range: WriteSignal<DimRange>,
) -> impl IntoView {
    let lo_pct = move || format!("{:.1}%", range.get().lo * 100.0);
    let hi_pct = move || format!("{:.1}%", range.get().hi * 100.0);

    let on_lo = move |ev: web_sys::Event| {
        let v: f32 = event_target_value(&ev).parse().unwrap_or(0.0) / 100.0;
        set_range.update(|r| r.lo = v.min(r.hi - 0.01).max(0.0));
    };
    let on_hi = move |ev: web_sys::Event| {
        let v: f32 = event_target_value(&ev).parse().unwrap_or(100.0) / 100.0;
        set_range.update(|r| r.hi = v.max(r.lo + 0.01).min(1.0));
    };

    view! {
        <div class="ge-slider">
            <div class="ge-slider-header">
                <span class="ge-slider-label">{label}</span>
                <span class="ge-slider-values">{lo_pct}" – "{hi_pct}</span>
            </div>
            <div class="ge-slider-track">
                <div class="ge-slider-fill" style=move || {
                    let r = range.get();
                    format!("left: {}%; width: {}%", r.lo * 100.0, (r.hi - r.lo) * 100.0)
                }/>
                <input type="range" min="0" max="100" step="1"
                    class="ge-range ge-range-lo"
                    prop:value=move || (range.get().lo * 100.0) as u32
                    on:input=on_lo
                />
                <input type="range" min="0" max="100" step="1"
                    class="ge-range ge-range-hi"
                    prop:value=move || (range.get().hi * 100.0) as u32
                    on:input=on_hi
                />
            </div>
        </div>
    }
}

#[component]
pub fn SliderPanel(
    joy:           ReadSignal<DimRange>,
    set_joy:       WriteSignal<DimRange>,
    sorrow:        ReadSignal<DimRange>,
    set_sorrow:    WriteSignal<DimRange>,
    intensity:     ReadSignal<DimRange>,
    set_intensity: WriteSignal<DimRange>,
    density:       ReadSignal<DimRange>,
    set_density:   WriteSignal<DimRange>,
    velocity:      ReadSignal<DimRange>,
    set_velocity:  WriteSignal<DimRange>,
) -> impl IntoView {
    view! {
        <div class="ge-slider-panel">
            <h2 class="ge-panel-title">"Dimensions"</h2>
            <DualSlider label="Joy"       range=joy       set_range=set_joy />
            <DualSlider label="Sorrow"    range=sorrow    set_range=set_sorrow />
            <DualSlider label="Intensity" range=intensity set_range=set_intensity />
            <DualSlider label="Density"   range=density   set_range=set_density />
            <DualSlider label="Velocity"  range=velocity  set_range=set_velocity />
        </div>
    }
}
