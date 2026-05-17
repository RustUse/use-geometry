#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/// A Johnson solid identifier from `J1` through `J92`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JohnsonSolidId(u8);

impl JohnsonSolidId {
    /// Creates an identifier for `J1` through `J92`.
    #[must_use]
    pub const fn new(number: u8) -> Option<Self> {
        if number >= 1 && number <= 92 {
            Some(Self(number))
        } else {
            None
        }
    }

    /// Returns the numeric part of the identifier.
    #[must_use]
    pub const fn number(self) -> u8 {
        self.0
    }

    /// Returns the canonical `J` label.
    #[must_use]
    pub fn label(self) -> String {
        format!("J{}", self.0)
    }
}

/// A Johnson solid descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct JohnsonSolid {
    id: JohnsonSolidId,
}

impl JohnsonSolid {
    /// Creates a Johnson solid descriptor.
    #[must_use]
    pub const fn new(id: JohnsonSolidId) -> Self {
        Self { id }
    }

    /// Returns the Johnson solid identifier.
    #[must_use]
    pub const fn id(self) -> JohnsonSolidId {
        self.id
    }

    /// Returns a selected common name when this crate carries one.
    #[must_use]
    pub const fn name(self) -> Option<&'static str> {
        match self.id.number() {
            1 => Some("square pyramid"),
            2 => Some("pentagonal pyramid"),
            3 => Some("triangular cupola"),
            4 => Some("square cupola"),
            5 => Some("pentagonal cupola"),
            6 => Some("pentagonal rotunda"),
            92 => Some("triangular hebesphenorotunda"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{JohnsonSolid, JohnsonSolidId};

    #[test]
    fn validates_johnson_solid_ids() {
        let id = JohnsonSolidId::new(1).expect("valid id");
        let solid = JohnsonSolid::new(id);

        assert_eq!(id.number(), 1);
        assert_eq!(id.label(), "J1");
        assert_eq!(solid.name(), Some("square pyramid"));
        assert_eq!(JohnsonSolidId::new(93), None);
    }
}
