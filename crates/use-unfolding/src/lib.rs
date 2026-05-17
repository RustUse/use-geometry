#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A single unfolding step between adjacent faces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnfoldingStep {
    source_face: usize,
    target_face: usize,
}

impl UnfoldingStep {
    /// Creates an unfolding step.
    #[must_use]
    pub const fn new(source_face: usize, target_face: usize) -> Self {
        Self {
            source_face,
            target_face,
        }
    }

    /// Returns the source and target face indices.
    #[must_use]
    pub const fn faces(self) -> (usize, usize) {
        (self.source_face, self.target_face)
    }
}

/// A sequence of unfolding steps.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnfoldingPlan {
    steps: Vec<UnfoldingStep>,
}

impl UnfoldingPlan {
    /// Creates an unfolding plan.
    #[must_use]
    pub const fn new(steps: Vec<UnfoldingStep>) -> Self {
        Self { steps }
    }

    /// Returns the steps.
    #[must_use]
    pub fn steps(&self) -> &[UnfoldingStep] {
        &self.steps
    }

    /// Returns the step count.
    #[must_use]
    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{UnfoldingPlan, UnfoldingStep};

    #[test]
    fn stores_unfolding_plans() {
        let step = UnfoldingStep::new(0, 1);
        let plan = UnfoldingPlan::new(vec![step]);

        assert_eq!(step.faces(), (0, 1));
        assert_eq!(plan.step_count(), 1);
        assert_eq!(plan.steps(), &[step]);
    }
}
