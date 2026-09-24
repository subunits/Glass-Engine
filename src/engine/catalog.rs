use super::{
    query::{search, RangeQuery, ScoredTrack},
    track::Track,
};

#[derive(Clone)]
pub struct Catalog {
    tracks: Vec<Track>,
}

impl Catalog {
    pub fn load() -> Self {
        let raw = include_str!("../../data/catalog.json");
        let tracks: Vec<Track> =
            serde_json::from_str(raw).expect("catalog.json is malformed");
        Self { tracks }
    }

    pub fn tracks(&self) -> &[Track] { &self.tracks }
    pub fn track_count(&self) -> usize { self.tracks.len() }

    pub fn year_bounds(&self) -> (u16, u16) {
        let min = self.tracks.iter().map(|t| t.year).min().unwrap_or(1960);
        let max = self.tracks.iter().map(|t| t.year).max().unwrap_or(2030);
        (min, max)
    }

    pub fn search(&self, query: &RangeQuery) -> Vec<ScoredTrack> {
        search(&self.tracks, query)
    }
}
