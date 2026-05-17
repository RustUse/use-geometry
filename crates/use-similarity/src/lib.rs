#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A positive finite scale ratio used for geometric similarity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimilarityRatio {
    scale: f64,
}

impl SimilarityRatio {
    /// Creates a positive finite similarity ratio.
    #[must_use]
    pub const fn new(scale: f64) -> Option<Self> {
        if scale.is_finite() && scale > 0.0 {
            Some(Self { scale })
        } else {
            None
        }
    }

    /// Returns the scale factor.
    #[must_use]
    pub const fn scale(self) -> f64 {
        self.scale
    }

    /// Applies the ratio to a length.
    #[must_use]
    pub fn apply_length(self, length: f64) -> f64 {
        length * self.scale
    }
}

#[cfg(test)]
mod tests {
    use super::SimilarityRatio;

    #[test]
    fn scales_lengths() {
        let ratio = SimilarityRatio::new(2.0).expect("valid ratio");

        assert_eq!(ratio.scale(), 2.0);
        assert_eq!(ratio.apply_length(3.0), 6.0);
        assert_eq!(SimilarityRatio::new(0.0), None);
    }
}
