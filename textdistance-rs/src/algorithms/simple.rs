//! Simple string distance/similarity algorithms.
//!
//! Basic comparisons: prefix, postfix, length, identity, matrix.

use super::base::{TextDistance, TextSimilarity};

// ---------------------------------------------------------------------------
// Prefix
// ---------------------------------------------------------------------------

/// Prefix similarity: length of common prefix.
#[derive(Debug, Clone, Default)]
pub struct Prefix;

impl Prefix {
    pub fn new() -> Self {
        Self
    }

    /// Get the common prefix string.
    pub fn prefix(&self, s1: &str, s2: &str) -> String {
        s1.chars()
            .zip(s2.chars())
            .take_while(|(a, b)| a == b)
            .map(|(a, _)| a)
            .collect()
    }
}

impl TextSimilarity for Prefix {
    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.prefix(s1, s2).chars().count() as f64
    }
}

// ---------------------------------------------------------------------------
// Postfix
// ---------------------------------------------------------------------------

/// Postfix similarity: length of common suffix.
#[derive(Debug, Clone, Default)]
pub struct Postfix;

impl Postfix {
    pub fn new() -> Self {
        Self
    }

    /// Get the common postfix string.
    pub fn postfix(&self, s1: &str, s2: &str) -> String {
        let r1: String = s1.chars().rev().collect();
        let r2: String = s2.chars().rev().collect();
        let prefix = Prefix::new();
        let result: String = prefix.prefix(&r1, &r2);
        result.chars().rev().collect()
    }
}

impl TextSimilarity for Postfix {
    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.postfix(s1, s2).chars().count() as f64
    }
}

// ---------------------------------------------------------------------------
// Length
// ---------------------------------------------------------------------------

/// Length distance: absolute difference of string lengths.
#[derive(Debug, Clone, Default)]
pub struct Length;

impl Length {
    pub fn new() -> Self {
        Self
    }
}

impl TextDistance for Length {
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        let l1 = s1.chars().count();
        let l2 = s2.chars().count();
        (l1.max(l2) - l1.min(l2)) as f64
    }
}

// ---------------------------------------------------------------------------
// Identity
// ---------------------------------------------------------------------------

/// Identity similarity: 1 if strings are equal, 0 otherwise.
#[derive(Debug, Clone, Default)]
pub struct Identity;

impl Identity {
    pub fn new() -> Self {
        Self
    }
}

impl TextSimilarity for Identity {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 { 1.0 } else { 0.0 }
    }
}

// ---------------------------------------------------------------------------
// Matrix
// ---------------------------------------------------------------------------

/// Matrix similarity: configurable match/mismatch costs.
#[derive(Debug, Clone)]
pub struct Matrix {
    pub mismatch_cost: f64,
    pub match_cost: f64,
}

impl Default for Matrix {
    fn default() -> Self {
        Self {
            mismatch_cost: 0.0,
            match_cost: 1.0,
        }
    }
}

impl Matrix {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Matrix {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        self.match_cost
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            self.match_cost
        } else {
            self.mismatch_cost
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prefix() {
        let p = Prefix::new();
        assert_eq!(p.prefix("abc", "abd"), "ab");
        assert_eq!(p.similarity("abcdef", "abcxyz"), 3.0);
    }

    #[test]
    fn test_postfix() {
        let p = Postfix::new();
        assert_eq!(p.postfix("abc", "xbc"), "bc");
        assert_eq!(p.similarity("xyzabc", "123abc"), 3.0);
    }

    #[test]
    fn test_length() {
        let l = Length::new();
        assert_eq!(l.distance("abc", "abcde"), 2.0);
        assert_eq!(l.distance("abc", "abc"), 0.0);
    }

    #[test]
    fn test_identity() {
        let id = Identity::new();
        assert_eq!(id.similarity("abc", "abc"), 1.0);
        assert_eq!(id.similarity("abc", "def"), 0.0);
    }

    #[test]
    fn test_matrix() {
        let m = Matrix::new();
        assert_eq!(m.similarity("abc", "abc"), 1.0);
        assert_eq!(m.similarity("abc", "def"), 0.0);
    }
}
