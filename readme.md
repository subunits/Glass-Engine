# Glass Engine

A Rust reimplementation of the IBM Glass Engine — multi-dimensional music
metadata indexing and nearest-neighbour similarity search, compiled to
WebAssembly and rendered with Leptos (CSR).

---

## Prerequisites

| Tool | Min version | Install |
|------|-------------|---------|
| Rust + Cargo | **1.80+** (edition 2024 deps) | `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \| sh` |
| wasm32 target | — | `rustup target add wasm32-unknown-unknown` |
| Trunk | any recent | `cargo install trunk` |
| wasm-opt *(optional, smaller bundles)* | — | `cargo install wasm-opt` |

> **Why 1.80+?** The `wasm-bindgen` and `leptos` dependency trees pull in
> crates that require Cargo's `edition2024` feature, stabilised in Rust 1.82.
> The apt-packaged Rust on Ubuntu 24 is 1.75 and will fail at `cargo test`.
> Use `rustup` to get a current stable toolchain.

---

## Project layout

```
glass-engine/
├── Cargo.toml              # cdylib + rlib, wasm-bindgen, leptos csr, serde
├── Trunk.toml              # build/serve config — watches src/, data/, styles/
├── index.html              # Trunk entry point; mounts <div id="app">
├── data/
│   └── catalog.json        # Track metadata — the only file you need to edit
├── styles/
│   └── main.css            # IBM Plex Mono, dark theme, responsive grid
└── src/
    ├── lib.rs              # #[wasm_bindgen(start)] + glass_search() JS API
    ├── engine/
    │   ├── mod.rs
    │   ├── track.rs        # Track · Dimensions · DimRange · Genre
    │   ├── query.rs        # RangeQuery · search() · ScoredTrack
    │   ├── catalog.rs      # Catalog backed by include_str!(catalog.json)
    │   └── tests.rs        # 7 native unit tests — no browser required
    └── ui/
        ├── mod.rs
        ├── app.rs          # Reactive signal graph; wires sliders → results
        ├── sliders.rs      # DualSlider (two-handle lo/hi) + SliderPanel
        ├── timeline.rs     # Year range slider
        └── results.rs      # ResultsList · TrackCard · DimDot fingerprint
```

---

## Running tests

The engine is entirely `no_std`-compatible and has no WASM or UI dependencies,
so tests compile and run natively — no browser, no `wasm-pack`, no headless
Chrome.

```bash
cargo test
```

Expected output:

```
running 7 tests
test engine::tests::dim_range_midpoint                 ... ok
test engine::tests::genre_filter_excludes_non_matching ... ok
test engine::tests::scores_are_normalised_0_to_1       ... ok
test engine::tests::search_returns_closest_to_target   ... ok
test engine::tests::top_k_limits_limits                ... ok
test engine::tests::weighted_dist_sq_zero_for_same_point ... ok
test engine::tests::year_range_filter_works            ... ok

test result: ok. 7 passed; 0 failed; 0 ignored
```

### What each test covers

| Test | What it asserts |
|------|----------------|
| `search_returns_closest_to_target` | A joy-high query ranks the most joyful track first |
| `genre_filter_excludes_non_matching` | Genre filter removes all non-matching tracks from results |
| `year_range_filter_works` | Year bounds `[1984, 1992]` excludes tracks outside that window |
| `top_k_limits_results` | Results never exceed `top_k` regardless of catalog size |
| `scores_are_normalised_0_to_1` | Every returned score is in `[0.0, 1.0]`; best match is exactly `1.0` |
| `dim_range_midpoint` | `DimRange { lo: 0.2, hi: 0.8 }.midpoint()` == `0.5` |
| `weighted_dist_sq_zero_for_same_point` | Distance from a point to itself is `0` |

---

## Dev server

```bash
trunk serve
# → http://localhost:8080  (hot-reload on every save)
```

Trunk watches `src/`, `data/`, `styles/`, and `index.html`. Saving any of
them triggers an incremental WASM recompile and live-reloads the browser tab.

---

## Production build

```bash
trunk build --release
```

Output lands in `dist/`. It is a fully static bundle — no server, no runtime,
no network requests after the initial load. Serve it from any CDN, GitHub
Pages, or `python3 -m http.server dist/`.

```bash
# Quick local preview of the release build
python3 -m http.server --directory dist 8081
```

---

## How the search works

Each track is a point in a **5-dimensional emotional/structural space**.
All values are normalised to `[0.0, 1.0]`.

| Dimension | What it captures |
|-----------|-----------------|
| Joy | Brightness, uplift, lightness |
| Sorrow | Melancholy, weight, grief |
| Intensity | Dynamic force, loudness arc |
| Density | Harmonic/textural complexity |
| Velocity | Rhythmic speed, pulse rate |

The slider panel produces a `RangeQuery`. Each dimension has a `[lo, hi]`
range — matching the original IBM engine's two-handled sliders. The search
proceeds in two passes:

1. **Hard filter** — tracks outside *any* dimension range, year range, or
   genre are discarded entirely.
2. **Soft rank** — remaining tracks are scored by weighted Euclidean distance
   to the midpoint of each range and sorted ascending (closest first).

```
dist(track) = √( Σ wᵢ · (track.dimᵢ − target.dimᵢ)² )
```

Scores are then normalised to `[0, 1]` within the result set so the UI
can render a meaningful similarity bar regardless of how tight the query is.

---

## Adding tracks

Edit `data/catalog.json`. The file is embedded at compile time via
`include_str!()`, so there is no runtime I/O — just rebuild after editing.

```json
{
  "id": 21,
  "title": "My Piece",
  "year": 2010,
  "duration_secs": 240,
  "genre": "chamber",
  "dims": {
    "joy":       0.6,
    "sorrow":    0.3,
    "intensity": 0.5,
    "density":   0.4,
    "velocity":  0.7
  },
  "note": "Optional annotation shown beneath the track title."
}
```

Valid `genre` values: `opera` · `film_score` · `chamber` · `orchestral` ·
`keyboard` · `choral` · `other`

---

## JS API

The WASM module exports `glass_search(queryJson) → string` for use from
plain JavaScript or any non-Leptos frontend:

```js
import init, { glass_search } from './pkg/glass_engine.js';
await init();

const results = JSON.parse(glass_search(JSON.stringify({
  joy:          { lo: 0.6, hi: 1.0 },
  sorrow:       { lo: 0.0, hi: 0.4 },
  intensity:    { lo: 0.0, hi: 1.0 },
  density:      { lo: 0.0, hi: 1.0 },
  velocity:     { lo: 0.0, hi: 1.0 },
  year_range:   [1975, 2005],
  genre_filter: null,           // or e.g. "opera"
  top_k:        10,
  weights: { joy: 1.0, sorrow: 1.0, intensity: 1.0, density: 1.0, velocity: 1.0 }
})));

// results: Array<{ track: Track, score: number, dist: number }>
```

---

## Roadmap

- **Weight sliders** — expose `weights` as a second slider row so users can
  emphasise e.g. velocity over joy
- **KD-tree** — swap brute-force search for `kiddo` once the catalog exceeds
  ~1 000 tracks
- **Audio playback** — `<audio>` tags on each `TrackCard`, pointing to files
  in `public/audio/`
- **Persistence** — `localStorage` to restore the last query on page load
- **wasm-pack output** — publish the engine as a standalone npm package
