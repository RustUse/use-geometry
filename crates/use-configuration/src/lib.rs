#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_incidence::IncidenceStructure;

/// Basic point-line configuration metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeometricConfiguration {
    point_count: usize,
    line_count: usize,
    incidence_count: usize,
}

impl GeometricConfiguration {
    /// Creates a configuration descriptor with positive point and line counts.
    #[must_use]
    pub const fn new(
        point_count: usize,
        line_count: usize,
        incidence_count: usize,
    ) -> Option<Self> {
        if point_count > 0 && line_count > 0 {
            Some(Self {
                point_count,
                line_count,
                incidence_count,
            })
        } else {
            None
        }
    }

    /// Returns the point count.
    #[must_use]
    pub const fn point_count(self) -> usize {
        self.point_count
    }

    /// Returns the line count.
    #[must_use]
    pub const fn line_count(self) -> usize {
        self.line_count
    }

    /// Returns the incidence count.
    #[must_use]
    pub const fn incidence_count(self) -> usize {
        self.incidence_count
    }
}

/// Two rows of six labels used in double-six configurations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DoubleSix {
    first: [usize; 6],
    second: [usize; 6],
}

impl DoubleSix {
    /// Creates a double-six label arrangement.
    #[must_use]
    pub const fn new(first: [usize; 6], second: [usize; 6]) -> Self {
        Self { first, second }
    }

    /// Returns the standard label arrangement.
    #[must_use]
    pub const fn standard() -> Self {
        Self::new([0, 1, 2, 3, 4, 5], [6, 7, 8, 9, 10, 11])
    }

    /// Returns the first row of labels.
    #[must_use]
    pub const fn first(self) -> [usize; 6] {
        self.first
    }

    /// Returns the second row of labels.
    #[must_use]
    pub const fn second(self) -> [usize; 6] {
        self.second
    }

    /// Returns the paired line count.
    #[must_use]
    pub const fn pair_count(self) -> usize {
        6
    }
}

/// The Schlafli double-six configuration metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SchlafliDoubleSix {
    arrangement: DoubleSix,
    configuration: GeometricConfiguration,
}

impl SchlafliDoubleSix {
    /// Creates the standard Schlafli double-six metadata record.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            arrangement: DoubleSix::standard(),
            configuration: GeometricConfiguration {
                point_count: 12,
                line_count: 30,
                incidence_count: 60,
            },
        }
    }

    /// Returns the double-six label arrangement.
    #[must_use]
    pub const fn arrangement(self) -> DoubleSix {
        self.arrangement
    }

    /// Returns the point-line configuration counts.
    #[must_use]
    pub const fn configuration(self) -> GeometricConfiguration {
        self.configuration
    }

    /// Returns a sparse incidence structure carrying the standard counts.
    #[must_use]
    pub fn incidence_structure(self) -> IncidenceStructure {
        IncidenceStructure::new(
            self.configuration.point_count(),
            self.configuration.line_count(),
            Vec::new(),
        )
        .expect("standard double-six counts are positive")
    }
}

impl Default for SchlafliDoubleSix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::{DoubleSix, GeometricConfiguration, SchlafliDoubleSix};

    #[test]
    fn stores_schlafli_double_six_metadata() {
        let double_six = SchlafliDoubleSix::new();
        let structure = double_six.incidence_structure();

        assert_eq!(double_six.arrangement().pair_count(), 6);
        assert_eq!(double_six.configuration().point_count(), 12);
        assert_eq!(structure.line_count(), 30);
        assert_eq!(DoubleSix::standard().first(), [0, 1, 2, 3, 4, 5]);
        assert_eq!(GeometricConfiguration::new(0, 1, 0), None);
    }
}
