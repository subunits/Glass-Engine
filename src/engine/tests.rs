#[cfg(test)]
mod tests {
    use crate::engine::query::{search, RangeQuery};
    use crate::engine::track::{DimRange, Dimensions, Genre, Track};

    fn make_track(id: u32, joy: f32, sorrow: f32, year: u16, genre: Genre) -> Track {
        Track {
            id,
            title: format!("Track {id}"),
            year,
            duration_secs: 300,
            genre,
            dims: Dimensions { joy, sorrow, intensity: 0.5, density: 0.5, velocity: 0.5 },
            note: None,
        }
    }

    fn sample_catalog() -> Vec<Track> {
        vec![
            make_track(1, 0.9, 0.1, 1980, Genre::Chamber),   // very joyful
            make_track(2, 0.1, 0.9, 1985, Genre::Opera),     // very sorrowful
            make_track(3, 0.5, 0.5, 2000, Genre::FilmScore), // neutral
            make_track(4, 0.8, 0.2, 1990, Genre::Chamber),   // mostly joyful
        ]
    }

    #[test]
    fn search_returns_closest_to_target() {
        let tracks = sample_catalog();
        let q = RangeQuery {
            joy: DimRange { lo: 0.0, hi: 1.0 },
            sorrow: DimRange { lo: 0.0, hi: 1.0 },
            ..RangeQuery::default()
        };
        // Targeting pure joy: should rank track 1 first
        let target_q = RangeQuery {
            joy: DimRange { lo: 0.8, hi: 1.0 },
            sorrow: DimRange { lo: 0.0, hi: 0.2 },
            ..q
        };
        let results = search(&tracks, &target_q);
        assert!(!results.is_empty());
        assert_eq!(results[0].track.id, 1);
    }

    #[test]
    fn genre_filter_excludes_non_matching() {
        let tracks = sample_catalog();
        let q = RangeQuery {
            genre_filter: Some(Genre::Opera),
            ..RangeQuery::default()
        };
        let results = search(&tracks, &q);
        assert!(results.iter().all(|r| r.track.genre == Genre::Opera));
    }

    #[test]
    fn year_range_filter_works() {
        let tracks = sample_catalog();
        let q = RangeQuery {
            year_range: (1984, 1992),
            ..RangeQuery::default()
        };
        let results = search(&tracks, &q);
        assert!(results.iter().all(|r| r.track.year >= 1984 && r.track.year <= 1992));
        // tracks 2 and 4 are in range; track 1 (1980) and 3 (2000) are not
        let ids: Vec<u32> = results.iter().map(|r| r.track.id).collect();
        assert!(ids.contains(&2));
        assert!(ids.contains(&4));
        assert!(!ids.contains(&1));
        assert!(!ids.contains(&3));
    }

    #[test]
    fn top_k_limits_results() {
        let tracks = sample_catalog();
        let q = RangeQuery { top_k: 2, ..RangeQuery::default() };
        let results = search(&tracks, &q);
        assert!(results.len() <= 2);
    }

    #[test]
    fn scores_are_normalised_0_to_1() {
        let tracks = sample_catalog();
        let results = search(&tracks, &RangeQuery::default());
        for r in &results {
            assert!(r.score >= 0.0 && r.score <= 1.0, "score out of range: {}", r.score);
        }
        // Best match should be score 1.0 (distance 0 from itself)
        if let Some(top) = results.first() {
            assert!((top.score - 1.0).abs() < 1e-5);
        }
    }

    #[test]
    fn dim_range_midpoint() {
        let r = DimRange { lo: 0.2, hi: 0.8 };
        assert!((r.midpoint() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn weighted_dist_sq_zero_for_same_point() {
        let d = Dimensions { joy: 0.3, sorrow: 0.7, intensity: 0.5, density: 0.2, velocity: 0.9 };
        assert!(d.weighted_dist_sq(&d, &Dimensions::UNIT_WEIGHT) < 1e-10);
    }
}
