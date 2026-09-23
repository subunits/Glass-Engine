use super::{
    query::{search, RangeQuery, ScoredTrack},
    track::Track,
};

/// The full track catalog. Embedded at compile time from data/catalog.json
/// so the app is a single self-contained WASM bundle — no HTTP requests needed.
pub struct Catalog {
    tracks: Vec<Track>,
}

impl Catalog {
    /// Load from the compile-time embedded JSON.
    pub fn load() -> Self {
        let raw = include_str!("../../data/catalog.json");
        let tracks: Vec<Track> =
            serde_json::from_str(raw).expect("catalog.json is malformed");
        Self { tracks }
    }

    pub fn tracks(&self) -> &[Track] {
        &self.tracks
    }

    pub fn track_count(&self) -> usize {
        self.tracks.len()
    }

    /// Year range spanning the whole catalog.
    pub fn year_bounds(&self) -> (u16, u16) {
        let min = self.tracks.iter().map(|t| t.year).min().unwrap_or(1960);
        let max = self.tracks.iter().map(|t| t.year).max().unwrap_or(2030);
        (min, max)
    }

    /// Run a query and return ranked results.
    pub fn search(&self, query: &RangeQuery) -> Vec<ScoredTrack> {
        search(&self.tracks, query)
    }
}
