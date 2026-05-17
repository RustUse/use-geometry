#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_angle::Angle;

/// A rigid fold around a crease index.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RigidFold {
    crease_index: usize,
    angle: Angle,
}

impl RigidFold {
    /// Creates a rigid fold.
    #[must_use]
    pub const fn new(crease_index: usize, angle: Angle) -> Self {
        Self {
            crease_index,
            angle,
        }
    }

    /// Returns the crease index.
    #[must_use]
    pub const fn crease_index(self) -> usize {
        self.crease_index
    }

    /// Returns the fold angle.
    #[must_use]
    pub const fn angle(self) -> Angle {
        self.angle
    }
}

/// A sequence of rigid folds.
#[derive(Debug, Clone, PartialEq)]
pub struct RigidFoldSequence {
    folds: Vec<RigidFold>,
}

impl RigidFoldSequence {
    /// Creates a rigid-fold sequence.
    #[must_use]
    pub const fn new(folds: Vec<RigidFold>) -> Self {
        Self { folds }
    }

    /// Returns the folds.
    #[must_use]
    pub fn folds(&self) -> &[RigidFold] {
        &self.folds
    }

    /// Returns the fold count.
    #[must_use]
    pub fn fold_count(&self) -> usize {
        self.folds.len()
    }
}

#[cfg(test)]
mod tests {
    use super::{RigidFold, RigidFoldSequence};
    use use_angle::Angle;

    #[test]
    fn stores_rigid_fold_sequences() {
        let fold = RigidFold::new(0, Angle::from_degrees(45.0));
        let sequence = RigidFoldSequence::new(vec![fold]);

        assert_eq!(fold.crease_index(), 0);
        assert_eq!(fold.angle().degrees(), 45.0);
        assert_eq!(sequence.fold_count(), 1);
        assert_eq!(sequence.folds(), &[fold]);
    }
}
