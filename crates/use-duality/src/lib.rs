#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A named dual pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Dual {
    primal: &'static str,
    dual: &'static str,
}

impl Dual {
    /// Creates a dual pair.
    #[must_use]
    pub const fn new(primal: &'static str, dual: &'static str) -> Self {
        Self { primal, dual }
    }

    /// Returns the primal name.
    #[must_use]
    pub const fn primal(self) -> &'static str {
        self.primal
    }

    /// Returns the dual name.
    #[must_use]
    pub const fn dual(self) -> &'static str {
        self.dual
    }
}

/// Metadata describing a duality relationship.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Duality {
    pair: Dual,
    involutive: bool,
}

impl Duality {
    /// Creates a duality descriptor.
    #[must_use]
    pub const fn new(pair: Dual, involutive: bool) -> Self {
        Self { pair, involutive }
    }

    /// Returns the dual pair.
    #[must_use]
    pub const fn pair(self) -> Dual {
        self.pair
    }

    /// Returns whether applying the duality twice returns to the starting family.
    #[must_use]
    pub const fn is_involutive(self) -> bool {
        self.involutive
    }
}

#[cfg(test)]
mod tests {
    use super::{Dual, Duality};

    #[test]
    fn stores_duality_metadata() {
        let pair = Dual::new("cube", "octahedron");
        let duality = Duality::new(pair, true);

        assert_eq!(duality.pair().primal(), "cube");
        assert_eq!(duality.pair().dual(), "octahedron");
        assert!(duality.is_involutive());
    }
}
