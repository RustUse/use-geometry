#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use core::fmt;

/// A lightweight Wythoff construction symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WythoffSymbol {
    notation: String,
}

impl WythoffSymbol {
    /// Creates a Wythoff symbol from non-empty notation.
    #[must_use]
    pub fn new(notation: impl Into<String>) -> Option<Self> {
        let notation = notation.into();

        if notation.trim().is_empty() {
            None
        } else {
            Some(Self { notation })
        }
    }

    /// Returns the notation string.
    #[must_use]
    pub fn notation(&self) -> &str {
        &self.notation
    }
}

impl fmt::Display for WythoffSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.notation)
    }
}

#[cfg(test)]
mod tests {
    use super::WythoffSymbol;

    #[test]
    fn stores_wythoff_notation() {
        let symbol = WythoffSymbol::new("3 | 4 2").expect("valid symbol");

        assert_eq!(symbol.notation(), "3 | 4 2");
        assert_eq!(symbol.to_string(), "3 | 4 2");
        assert_eq!(WythoffSymbol::new("   "), None);
    }
}
