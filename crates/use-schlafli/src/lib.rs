#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// A Schlafli symbol represented by its numeric entries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchlafliSymbol {
    entries: Vec<usize>,
}

impl SchlafliSymbol {
    /// Creates a symbol from positive entries.
    #[must_use]
    pub fn new(entries: Vec<usize>) -> Option<Self> {
        if !entries.is_empty() && entries.iter().all(|entry| *entry >= 2) {
            Some(Self { entries })
        } else {
            None
        }
    }

    /// Creates a regular polygon symbol `{p}`.
    #[must_use]
    pub fn polygon(p: usize) -> Option<Self> {
        if p >= 3 { Self::new(vec![p]) } else { None }
    }

    /// Creates a regular polyhedron symbol `{p, q}`.
    #[must_use]
    pub fn polyhedron(p: usize, q: usize) -> Option<Self> {
        if p >= 3 && q >= 3 {
            Self::new(vec![p, q])
        } else {
            None
        }
    }

    /// Creates a regular polychoron symbol `{p, q, r}`.
    #[must_use]
    pub fn polychoron(p: usize, q: usize, r: usize) -> Option<Self> {
        if p >= 3 && q >= 3 && r >= 3 {
            Self::new(vec![p, q, r])
        } else {
            None
        }
    }

    /// Returns the symbol entries.
    #[must_use]
    pub fn entries(&self) -> &[usize] {
        &self.entries
    }

    /// Returns the Coxeter rank conventionally associated with this symbol.
    #[must_use]
    pub fn rank(&self) -> usize {
        self.entries.len() + 1
    }

    /// Returns the regular polytope dimension conventionally associated with this symbol.
    #[must_use]
    pub fn dimension(&self) -> usize {
        self.entries.len()
    }
}

impl fmt::Display for SchlafliSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("{")?;

        for (index, entry) in self.entries.iter().enumerate() {
            if index > 0 {
                formatter.write_str(", ")?;
            }

            write!(formatter, "{entry}")?;
        }

        formatter.write_str("}")
    }
}

#[cfg(test)]
mod tests {
    use super::SchlafliSymbol;

    #[test]
    fn formats_schlafli_symbols() {
        let symbol = SchlafliSymbol::polychoron(3, 4, 3).expect("valid symbol");

        assert_eq!(symbol.entries(), &[3, 4, 3]);
        assert_eq!(symbol.rank(), 4);
        assert_eq!(symbol.dimension(), 3);
        assert_eq!(symbol.to_string(), "{3, 4, 3}");
        assert_eq!(SchlafliSymbol::polygon(2), None);
    }
}
