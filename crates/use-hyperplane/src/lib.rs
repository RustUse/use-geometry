#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_geometry_core::GeometryError;

/// An n-dimensional hyperplane represented by `coefficients.dot(point) + offset = 0`.
#[derive(Debug, Clone, PartialEq)]
pub struct Hyperplane {
    coefficients: Vec<f64>,
    offset: f64,
}

impl Hyperplane {
    /// Creates a validated hyperplane from finite coefficients and finite offset.
    ///
    /// # Errors
    ///
    /// Returns a [`GeometryError`] when coefficients are empty, non-finite, all zero,
    /// or when the offset is non-finite.
    pub fn try_new(coefficients: Vec<f64>, offset: f64) -> Result<Self, GeometryError> {
        if coefficients.is_empty() || coefficients.iter().all(|value| *value == 0.0) {
            return Err(GeometryError::ZeroDirectionVector);
        }

        for value in &coefficients {
            if !value.is_finite() {
                return Err(GeometryError::non_finite_component(
                    "Hyperplane",
                    "coefficient",
                    *value,
                ));
            }
        }

        if !offset.is_finite() {
            return Err(GeometryError::non_finite_component(
                "Hyperplane",
                "offset",
                offset,
            ));
        }

        Ok(Self {
            coefficients,
            offset,
        })
    }

    /// Returns the hyperplane dimension.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.coefficients.len()
    }

    /// Returns the coefficient vector.
    #[must_use]
    pub fn coefficients(&self) -> &[f64] {
        &self.coefficients
    }

    /// Returns the offset.
    #[must_use]
    pub const fn offset(&self) -> f64 {
        self.offset
    }

    /// Evaluates the hyperplane equation at `point` when dimensions match.
    #[must_use]
    pub fn evaluate(&self, point: &[f64]) -> Option<f64> {
        if point.len() != self.coefficients.len() {
            return None;
        }

        Some(
            self.coefficients
                .iter()
                .zip(point)
                .map(|(coefficient, coordinate)| coefficient * coordinate)
                .sum::<f64>()
                + self.offset,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::Hyperplane;
    use use_geometry_core::GeometryError;

    #[test]
    fn evaluates_hyperplanes() {
        let hyperplane = Hyperplane::try_new(vec![1.0, 0.0, 0.0], -2.0).expect("valid hyperplane");

        assert_eq!(hyperplane.dimension(), 3);
        assert_eq!(hyperplane.coefficients(), &[1.0, 0.0, 0.0]);
        assert_eq!(hyperplane.offset(), -2.0);
        assert_eq!(hyperplane.evaluate(&[2.0, 4.0, 5.0]), Some(0.0));
        assert_eq!(hyperplane.evaluate(&[2.0, 4.0]), None);
    }

    #[test]
    fn rejects_empty_or_zero_coefficients() {
        assert_eq!(
            Hyperplane::try_new(Vec::new(), 0.0),
            Err(GeometryError::ZeroDirectionVector)
        );
        assert_eq!(
            Hyperplane::try_new(vec![0.0, 0.0], 0.0),
            Err(GeometryError::ZeroDirectionVector)
        );
    }
}
