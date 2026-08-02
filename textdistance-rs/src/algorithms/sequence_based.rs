//! Sequence-based string similarity algorithms.
//!
//! Algorithms based on longest common subsequence / substring matching.

use super::base::TextSimilarity;

// ---------------------------------------------------------------------------
// LCSSeq — Longest Common Subsequence
// ---------------------------------------------------------------------------

/// Longest Common Subsequence similarity.
///
/// <https://en.wikipedia.org/wiki/Longest_common_subsequence_problem>
#[derive(Debug, Clone, Default)]
pub struct LCSSeq;

impl LCSSeq {
    pub fn new() -> Self {
        Self
    }

    /// Compute the LCS string using dynamic programming.
    pub fn lcs(&self, s1: &str, s2: &str) -> String {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        let mut lengths = vec![vec![0usize; len2 + 1]; len1 + 1];

        for i in 0..len1 {
            for j in 0..len2 {
                if c1[i] == c2[j] {
                    lengths[i + 1][j + 1] = lengths[i][j] + 1;
                } else {
                    lengths[i + 1][j + 1] = lengths[i + 1][j].max(lengths[i][j + 1]);
                }
            }
        }

        // Backtrack to find the actual subsequence
        let mut result = Vec::new();
        let mut i = len1;
        let mut j = len2;
        while i != 0 && j != 0 {
            if lengths[i][j] == lengths[i - 1][j] {
                i -= 1;
            } else if lengths[i][j] == lengths[i][j - 1] {
                j -= 1;
            } else {
                result.push(c1[i - 1]);
                i -= 1;
                j -= 1;
            }
        }
        result.reverse();
        result.into_iter().collect()
    }
}

impl TextSimilarity for LCSSeq {
    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.lcs(s1, s2).chars().count() as f64
    }
}

// ---------------------------------------------------------------------------
// LCSStr — Longest Common Substring
// ---------------------------------------------------------------------------

/// Longest Common Substring similarity.
#[derive(Debug, Clone, Default)]
pub struct LCSStr;

impl LCSStr {
    pub fn new() -> Self {
        Self
    }

    /// Compute the longest common substring.
    pub fn lcs_str(&self, s1: &str, s2: &str) -> String {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        if len1 == 0 || len2 == 0 {
            return String::new();
        }

        let mut longest = 0;
        let mut end_pos = 0;
        let mut lengths = vec![vec![0usize; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                if c1[i - 1] == c2[j - 1] {
                    lengths[i][j] = lengths[i - 1][j - 1] + 1;
                    if lengths[i][j] > longest {
                        longest = lengths[i][j];
                        end_pos = i;
                    }
                }
            }
        }

        c1[(end_pos - longest)..end_pos].iter().collect()
    }
}

impl TextSimilarity for LCSStr {
    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.lcs_str(s1, s2).chars().count() as f64
    }
}

// ---------------------------------------------------------------------------
// RatcliffObershelp
// ---------------------------------------------------------------------------

/// Ratcliff-Obershelp (Gestalt Pattern Matching) similarity.
///
/// <https://en.wikipedia.org/wiki/Gestalt_Pattern_Matching>
#[derive(Debug, Clone, Default)]
pub struct RatcliffObershelp;

impl RatcliffObershelp {
    pub fn new() -> Self {
        Self
    }

    fn find(&self, s1: &str, s2: &str) -> usize {
        let lcs = LCSStr::new();
        let subseq = lcs.lcs_str(s1, s2);
        let length = subseq.chars().count();
        if length == 0 {
            return 0;
        }

        let pos1 = s1.find(&subseq).unwrap_or(0);
        let pos2 = s2.find(&subseq).unwrap_or(0);

        let before1 = &s1[..pos1];
        let before2 = &s2[..pos2];
        let after1 = &s1[pos1 + subseq.len()..];
        let after2 = &s2[pos2 + subseq.len()..];

        self.find(before1, before2) + length + self.find(after1, after2)
    }
}

impl TextSimilarity for RatcliffObershelp {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }
        let ecount = s1.chars().count() + s2.chars().count();
        if ecount == 0 {
            return 0.0;
        }
        2.0 * self.find(s1, s2) as f64 / ecount as f64
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lcsseq() {
        let l = LCSSeq::new();
        assert_eq!(l.lcs("AGCAT", "GAC"), "GAC");
        assert_eq!(l.similarity("abc", "abc"), 3.0);
    }

    #[test]
    fn test_lcsstr() {
        let l = LCSStr::new();
        assert_eq!(l.lcs_str("abcdef", "zbcdf"), "bcd");
        assert_eq!(l.similarity("abc", "abc"), 3.0);
    }

    #[test]
    fn test_ratcliff() {
        let r = RatcliffObershelp::new();
        assert_eq!(r.similarity("abc", "abc"), 1.0);
        assert!(r.similarity("abc", "def") == 0.0);
    }
}
