#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use use_point::Point2;

/// A crease classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CreaseKind {
    /// Mountain crease.
    Mountain,
    /// Valley crease.
    Valley,
    /// Flat crease.
    Flat,
}

/// A crease segment in a flat pattern.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Crease {
    start: Point2,
    end: Point2,
    kind: CreaseKind,
}

impl Crease {
    /// Creates a crease.
    #[must_use]
    pub const fn new(start: Point2, end: Point2, kind: CreaseKind) -> Self {
        Self { start, end, kind }
    }

    /// Returns the start point.
    #[must_use]
    pub const fn start(self) -> Point2 {
        self.start
    }

    /// Returns the end point.
    #[must_use]
    pub const fn end(self) -> Point2 {
        self.end
    }

    /// Returns the crease kind.
    #[must_use]
    pub const fn kind(self) -> CreaseKind {
        self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{Crease, CreaseKind};
    use use_point::Point2;

    #[test]
    fn stores_crease_lines() {
        let crease = Crease::new(
            Point2::origin(),
            Point2::new(1.0, 0.0),
            CreaseKind::Mountain,
        );

        assert_eq!(crease.start(), Point2::origin());
        assert_eq!(crease.end(), Point2::new(1.0, 0.0));
        assert_eq!(crease.kind(), CreaseKind::Mountain);
    }
}
