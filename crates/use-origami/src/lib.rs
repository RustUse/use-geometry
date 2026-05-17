#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A compact origami model summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrigamiModel {
    crease_count: usize,
    face_count: usize,
}

impl OrigamiModel {
    /// Creates an origami model summary.
    #[must_use]
    pub const fn new(crease_count: usize, face_count: usize) -> Option<Self> {
        if face_count > 0 {
            Some(Self {
                crease_count,
                face_count,
            })
        } else {
            None
        }
    }

    /// Returns the crease count.
    #[must_use]
    pub const fn crease_count(self) -> usize {
        self.crease_count
    }

    /// Returns the face count.
    #[must_use]
    pub const fn face_count(self) -> usize {
        self.face_count
    }
}

#[cfg(test)]
mod tests {
    use super::OrigamiModel;

    #[test]
    fn stores_origami_model_counts() {
        let model = OrigamiModel::new(8, 4).expect("valid model");

        assert_eq!(model.crease_count(), 8);
        assert_eq!(model.face_count(), 4);
        assert_eq!(OrigamiModel::new(8, 0), None);
    }
}
