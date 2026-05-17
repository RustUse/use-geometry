#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_angle::Angle;

/// A fold assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoldAssignment {
    /// A mountain fold.
    Mountain,
    /// A valley fold.
    Valley,
    /// An unassigned fold.
    Unassigned,
}

/// A fold assignment with an angle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FoldState {
    assignment: FoldAssignment,
    angle: Angle,
}

impl FoldState {
    /// Creates a fold state.
    #[must_use]
    pub const fn new(assignment: FoldAssignment, angle: Angle) -> Self {
        Self { assignment, angle }
    }

    /// Returns the assignment.
    #[must_use]
    pub const fn assignment(self) -> FoldAssignment {
        self.assignment
    }

    /// Returns the angle.
    #[must_use]
    pub const fn angle(self) -> Angle {
        self.angle
    }
}

#[cfg(test)]
mod tests {
    use super::{FoldAssignment, FoldState};
    use use_angle::Angle;

    #[test]
    fn stores_fold_states() {
        let fold = FoldState::new(FoldAssignment::Mountain, Angle::from_degrees(90.0));

        assert_eq!(fold.assignment(), FoldAssignment::Mountain);
        assert_eq!(fold.angle().degrees(), 90.0);
    }
}
