# Glass Engine — Rust/WASM

A Rust reimplementation of the IBM Glass Engine: multi-dimensional music
metadata indexing and similarity search, compiled to WebAssembly and served
as a Leptos CSR app.

## Architecture

```
src/
├── lib.rs               # WASM entry point + optional JS-callable glass_search()
├── engine/
│   ├── track.rs         # Track, Dimensions, DimRange, Genre
│   ├── query.rs         # RangeQuery, search(), ScoredTrack
│   ├── catalog.rs       # Catalog (loads catalog.json at compile time)
│   └── tests.rs         # Unit tests (cargo test, no browser needed)
└── ui/
    ├── app.rs           # Root Leptos component — reactive signal graph
    ├── sliders.rs       # DualSlider + SliderPanel (two-handled range sliders)
    ├── timeline.rs      # Year range slider
    └── results.rs       # ResultsList + TrackCard + DimDot
data/
└── catalog.json         # Track metadata — edit this to add your own works
styles/
└── main.css             # IBM Plex Mono, dark theme, responsive
```

## How the search works

Each track is a point in a **5-dimensional emotional/structural space**:

| Dimension | What it captures |
|-----------|-----------------|
| Joy       | Brightness, uplift |
| Sorrow    | Melancholy, weight |
| Intensity | Dynamic force |
| Density   | Harmonic / textural complexity |
| Velocity  | Rhythmic speed |

The slider panel produces a `RangeQuery`: each dimension has a `[lo, hi]`
range. Tracks outside *any* range are excluded. The search target is the
midpoint of each range, and remaining tracks are ranked by **weighted
Euclidean distance** to that target.

Scores are normalised to `[0, 1]` within each result set (1.0 = closest).

## Quick start

```bash
# Prerequisites
rustup target add wasm32-unknown-unknown
cargo install trunk

# Dev server with hot reload
trunk serve

# Open http://localhost:8080
```

## Run tests (native, no browser needed)

```bash
cargo test
```

## Add tracks

Edit `data/catalog.json`. Each entry:

```json
{
  "id": 21,
  "title": "My Piece",
  "year": 2010,
  "duration_secs": 240,
  "genre": "chamber",       // opera|film_score|chamber|orchestral|keyboard|choral|other
  "dims": {
    "joy":       0.6,       // all values 0.0–1.0
    "sorrow":    0.3,
    "intensity": 0.5,
    "density":   0.4,
    "velocity":  0.7
  },
  "note": "Optional annotation shown in the UI."
}
```

## JS API (optional)

The WASM module also exports `glass_search(queryJson)` for use from plain JS
or other frameworks:

```js
import init, { glass_search } from './pkg/glass_engine.js';
await init();

const results = JSON.parse(glass_search(JSON.stringify({
  joy:       { lo: 0.6, hi: 1.0 },
  sorrow:    { lo: 0.0, hi: 0.4 },
  intensity: { lo: 0.0, hi: 1.0 },
  density:   { lo: 0.0, hi: 1.0 },
  velocity:  { lo: 0.0, hi: 1.0 },
  year_range: [1975, 2005],
  genre_filter: null,
  top_k: 10,
  weights: { joy:1.0, sorrow:1.0, intensity:1.0, density:1.0, velocity:1.0 }
})));
```

## Production build

```bash
trunk build --release
# Output in dist/ — serve as a static site, no server required
```
