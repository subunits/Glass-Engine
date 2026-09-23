use serde::{Deserialize, Serialize};
use super::track::{DimRange, Dimensions, Genre, Track};

// ── RangeQuery ───────────────────────────────────────────────────────────────

/// What the slider panel produces. Each emotional dimension is a DimRange;
/// tracks outside any range are excluded before scoring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RangeQuery {
    pub joy:          DimRange,
    pub sorrow:       DimRange,
    pub intensity:    DimRange,
    pub density:      DimRange,
    pub velocity:     DimRange,
    pub year_range:   (u16, u16),
    pub genre_filter: Option<Genre>,
    /// How many results to return (default 10)
    pub top_k:        usize,
    /// Per-dimension importance weights (default all 1.0)
    pub weights:      Dimensions,
}

impl Default for RangeQuery {
    fn default() -> Self {
        Self {
            joy:          DimRange::full(),
            sorrow:       DimRange::full(),
            intensity:    DimRange::full(),
            density:      DimRange::full(),
            velocity:     DimRange::full(),
            year_range:   (1960, 2030),
            genre_filter: None,
            top_k:        10,
            weights:      Dimensions::UNIT_WEIGHT,
        }
    }
}

impl RangeQuery {
    /// Collapse each DimRange to its midpoint to get the search target.
    fn target(&self) -> Dimensions {
        Dimensions {
            joy:       self.joy.midpoint(),
            sorrow:    self.sorrow.midpoint(),
            intensity: self.intensity.midpoint(),
            density:   self.density.midpoint(),
            velocity:  self.velocity.midpoint(),
        }
    }

    /// Returns true if the track passes all hard filters.
    fn passes_filters(&self, track: &Track) -> bool {
        let d = &track.dims;
        self.joy.contains(d.joy)
            && self.sorrow.contains(d.sorrow)
            && self.intensity.contains(d.intensity)
            && self.density.contains(d.density)
            && self.velocity.contains(d.velocity)
            && track.year >= self.year_range.0
            && track.year <= self.year_range.1
            && self
                .genre_filter
                .as_ref()
                .map_or(true, |g| &track.genre == g)
    }

    /// Score a track: lower = better match.
    pub fn score(&self, track: &Track) -> f32 {
        let target = self.target();
        track.dims.weighted_dist_sq(&target, &self.weights).sqrt()
    }
}

// ── ScoredTrack ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoredTrack {
    pub track: Track,
    /// Normalised similarity score: 1.0 = perfect match, 0.0 = max distance
    pub score: f32,
    /// Raw weighted distance (lower = better)
    pub dist:  f32,
}

// ── search ───────────────────────────────────────────────────────────────────

/// Run a query against a slice of tracks. Returns up to `query.top_k` results
/// sorted by ascending distance (best first).
pub fn search(tracks: &[Track], query: &RangeQuery) -> Vec<ScoredTrack> {
    let mut scored: Vec<(f32, &Track)> = tracks
        .iter()
        .filter(|t| query.passes_filters(t))
        .map(|t| (query.score(t), t))
        .collect();

    scored.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    scored.truncate(query.top_k);

    // Normalise: map [0, max_dist] → [1, 0]
    let max_dist = scored.last().map(|(d, _)| *d).unwrap_or(1.0).max(f32::EPSILON);

    scored
        .into_iter()
        .map(|(dist, track)| ScoredTrack {
            track: track.clone(),
            score: 1.0 - (dist / max_dist),
            dist,
        })
        .collect()
}
