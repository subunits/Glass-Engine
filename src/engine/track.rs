use serde::{Deserialize, Serialize};

// ── Genre ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Genre {
    Opera,
    FilmScore,
    Chamber,
    Orchestral,
    Keyboard,
    Choral,
    Other,
}

impl Genre {
    pub fn label(&self) -> &'static str {
        match self {
            Genre::Opera      => "Opera",
            Genre::FilmScore  => "Film Score",
            Genre::Chamber    => "Chamber",
            Genre::Orchestral => "Orchestral",
            Genre::Keyboard   => "Keyboard",
            Genre::Choral     => "Choral",
            Genre::Other      => "Other",
        }
    }

    pub fn all() -> &'static [Genre] {
        &[
            Genre::Opera,
            Genre::FilmScore,
            Genre::Chamber,
            Genre::Orchestral,
            Genre::Keyboard,
            Genre::Choral,
            Genre::Other,
        ]
    }
}

// ── Dimensions ───────────────────────────────────────────────────────────────

/// A point (or direction) in the 5-dimensional emotional/structural space.
/// All values are normalised to [0.0, 1.0].
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Dimensions {
    pub joy:       f32,
    pub sorrow:    f32,
    pub intensity: f32,
    pub density:   f32,
    pub velocity:  f32,
}

impl Dimensions {
    pub const NEUTRAL: Self = Self {
        joy:       0.5,
        sorrow:    0.5,
        intensity: 0.5,
        density:   0.5,
        velocity:  0.5,
    };

    pub const UNIT_WEIGHT: Self = Self {
        joy:       1.0,
        sorrow:    1.0,
        intensity: 1.0,
        density:   1.0,
        velocity:  1.0,
    };

    /// Weighted squared Euclidean distance to another point.
    pub fn weighted_dist_sq(&self, other: &Dimensions, weights: &Dimensions) -> f32 {
        let d = |a: f32, b: f32, w: f32| w * (a - b).powi(2);
        d(self.joy,       other.joy,       weights.joy)
            + d(self.sorrow,    other.sorrow,    weights.sorrow)
            + d(self.intensity, other.intensity, weights.intensity)
            + d(self.density,   other.density,   weights.density)
            + d(self.velocity,  other.velocity,  weights.velocity)
    }
}

// ── DimRange ─────────────────────────────────────────────────────────────────

/// The original Glass Engine used two-handled sliders — each dimension has a
/// lo and hi endpoint that the user can drag independently. The search target
/// is the midpoint, and tracks outside [lo, hi] are excluded.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DimRange {
    pub lo: f32, // 0.0–1.0
    pub hi: f32, // 0.0–1.0
}

impl DimRange {
    pub fn full() -> Self { Self { lo: 0.0, hi: 1.0 } }

    pub fn midpoint(&self) -> f32 {
        (self.lo + self.hi) * 0.5
    }

    pub fn contains(&self, v: f32) -> bool {
        v >= self.lo && v <= self.hi
    }
}

impl Default for DimRange {
    fn default() -> Self { Self::full() }
}

// ── Track ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id:            u32,
    pub title:         String,
    pub year:          u16,
    pub duration_secs: u32,
    pub genre:         Genre,
    pub dims:          Dimensions,
    /// Optional short annotation shown in the UI
    pub note:          Option<String>,
}

impl Track {
    /// Human-readable duration string, e.g. "4:32"
    pub fn duration_str(&self) -> String {
        let m = self.duration_secs / 60;
        let s = self.duration_secs % 60;
        format!("{m}:{s:02}")
    }
}
