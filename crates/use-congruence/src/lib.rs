#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A non-negative tolerance for congruence-style comparisons.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CongruenceTolerance {
    value: f64,
}

impl CongruenceTolerance {
    /// Creates a finite, non-negative tolerance.
    #[must_use]
    pub const fn new(value: f64) -> Option<Self> {
        if value.is_finite() && value >= 0.0 {
            Some(Self { value })
        } else {
            None
        }
    }

    /// Returns the tolerance value.
    #[must_use]
    pub const fn value(self) -> f64 {
        self.value
    }

    /// Returns `true` when two lengths match within this tolerance.
    #[must_use]
    pub fn matches_lengths(self, left: f64, right: f64) -> bool {
        (left - right).abs() <= self.value
    }
}

#[cfg(test)]
mod tests {
    use super::CongruenceTolerance;

    #[test]
    fn compares_lengths_with_tolerance() {
        let tolerance = CongruenceTolerance::new(0.01).expect("valid tolerance");

        assert!(tolerance.matches_lengths(3.0, 3.005));
        assert!(!tolerance.matches_lengths(3.0, 3.02));
        assert_eq!(CongruenceTolerance::new(-1.0), None);
    }
}
