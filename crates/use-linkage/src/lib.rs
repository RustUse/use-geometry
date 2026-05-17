#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A linkage joint identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JointId(pub usize);

/// A bar connecting two joints.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bar {
    start: usize,
    end: usize,
    length: f64,
}

impl Bar {
    /// Creates a bar with a positive finite length.
    #[must_use]
    pub const fn new(start: usize, end: usize, length: f64) -> Option<Self> {
        if start != end && length.is_finite() && length > 0.0 {
            Some(Self { start, end, length })
        } else {
            None
        }
    }

    /// Returns the endpoint joint indices.
    #[must_use]
    pub const fn endpoints(self) -> (usize, usize) {
        (self.start, self.end)
    }

    /// Returns the bar length.
    #[must_use]
    pub const fn length(self) -> f64 {
        self.length
    }
}

/// A linkage summary.
#[derive(Debug, Clone, PartialEq)]
pub struct Linkage {
    joint_count: usize,
    bars: Vec<Bar>,
}

impl Linkage {
    /// Creates a linkage with at least one joint.
    #[must_use]
    pub fn new(joint_count: usize, bars: Vec<Bar>) -> Option<Self> {
        if joint_count > 0 {
            Some(Self { joint_count, bars })
        } else {
            None
        }
    }

    /// Returns the joint count.
    #[must_use]
    pub const fn joint_count(&self) -> usize {
        self.joint_count
    }

    /// Returns the bars.
    #[must_use]
    pub fn bars(&self) -> &[Bar] {
        &self.bars
    }
}

#[cfg(test)]
mod tests {
    use super::{Bar, JointId, Linkage};

    #[test]
    fn stores_linkage_records() {
        let bar = Bar::new(0, 1, 2.0).expect("valid bar");
        let linkage = Linkage::new(4, vec![bar]).expect("valid linkage");

        assert_eq!(JointId(2).0, 2);
        assert_eq!(bar.endpoints(), (0, 1));
        assert_eq!(bar.length(), 2.0);
        assert_eq!(linkage.joint_count(), 4);
        assert_eq!(linkage.bars(), &[bar]);
    }
}
