//! Phonetic string distance algorithms.
//!
//! Algorithms based on phonetic encoding of words.

use std::collections::HashMap;

use super::base::{TextDistance, TextSimilarity};

// ---------------------------------------------------------------------------
// MRA — Match Rating Approach
// ---------------------------------------------------------------------------

/// Western Airlines Match Rating Approach comparison rating.
///
/// <https://en.wikipedia.org/wiki/Match_rating_approach>
#[derive(Debug, Clone, Default)]
pub struct Mra;

impl Mra {
    pub fn new() -> Self {
        Self
    }

    fn calc_mra(&self, word: &str) -> String {
        if word.is_empty() {
            return String::new();
        }
        let word = word.to_uppercase();
        let chars: Vec<char> = word.chars().collect();

        // Keep first char, remove vowels from rest
        let mut result = String::new();
        result.push(chars[0]);
        for &c in &chars[1..] {
            if !matches!(c, 'A' | 'E' | 'I' | 'O' | 'U') {
                result.push(c);
            }
        }

        // Remove consecutive duplicates
        let chars: Vec<char> = result.chars().collect();
        let mut deduped = String::new();
        for (i, &c) in chars.iter().enumerate() {
            if i == 0 || c != chars[i - 1] {
                deduped.push(c);
            }
        }

        // Truncate to first 3 + last 3 if longer than 6
        if deduped.chars().count() > 6 {
            let chars: Vec<char> = deduped.chars().collect();
            let len = chars.len();
            chars[..3].iter().chain(chars[len - 3..].iter()).collect()
        } else {
            deduped
        }
    }
}

impl TextSimilarity for Mra {
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        let m1: Vec<char> = self.calc_mra(s1).chars().collect();
        let m2: Vec<char> = self.calc_mra(s2).chars().collect();
        m1.len().max(m2.len()) as f64
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let mut seq1: Vec<char> = self.calc_mra(s1).chars().collect();
        let mut seq2: Vec<char> = self.calc_mra(s2).chars().collect();

        let max_length = seq1.len().max(seq2.len());
        if seq1.len().abs_diff(seq2.len()) > 2 {
            return 0.0;
        }

        // Remove matching chars from front
        for _ in 0..2 {
            let minlen = seq1.len().min(seq2.len());
            let mut new1 = Vec::new();
            let mut new2 = Vec::new();
            for i in 0..minlen {
                if seq1[i] != seq2[i] {
                    new1.push(seq1[i]);
                    new2.push(seq2[i]);
                }
            }
            // Append remaining
            if seq1.len() > minlen {
                new1.extend_from_slice(&seq1[minlen..]);
            }
            if seq2.len() > minlen {
                new2.extend_from_slice(&seq2[minlen..]);
            }
            seq1 = new1;
            seq2 = new2;
        }

        let remaining = seq1.len().max(seq2.len());
        (max_length - remaining) as f64
    }
}

// ---------------------------------------------------------------------------
// Editex
// ---------------------------------------------------------------------------

/// Editex phonetic string distance.
///
/// <https://anhaidgroup.github.io/py_stringmatching/v0.3.x/Editex.html>
#[derive(Debug, Clone)]
pub struct Editex {
    pub local: bool,
    pub match_cost: i32,
    pub group_cost: i32,
    pub mismatch_cost: i32,
}

impl Default for Editex {
    fn default() -> Self {
        Self {
            local: false,
            match_cost: 0,
            group_cost: 1,
            mismatch_cost: 2,
        }
    }
}

impl Editex {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_groups() -> Vec<Vec<char>> {
        vec![
            vec!['A', 'E', 'I', 'O', 'U', 'Y'],
            vec!['B', 'P'],
            vec!['C', 'K', 'Q'],
            vec!['D', 'T'],
            vec!['L', 'R'],
            vec!['M', 'N'],
            vec!['G', 'J'],
            vec!['F', 'P', 'V'],
            vec!['S', 'X', 'Z'],
            vec!['C', 'S', 'Z'],
        ]
    }

    fn is_ungrouped(c: char) -> bool {
        c == 'H' || c == 'W'
    }

    fn is_grouped(c: char) -> bool {
        let groups = Self::get_groups();
        groups.iter().any(|g| g.contains(&c))
    }

    fn r_cost(&self, a: char, b: char) -> i32 {
        if a == b {
            return self.match_cost;
        }
        if !Self::is_grouped(a) || !Self::is_grouped(b) {
            return self.mismatch_cost;
        }
        let groups = Self::get_groups();
        for group in &groups {
            if group.contains(&a) && group.contains(&b) {
                return self.group_cost;
            }
        }
        self.mismatch_cost
    }

    fn d_cost(&self, a: char, b: char) -> i32 {
        if a != b && Self::is_ungrouped(a) {
            return self.group_cost;
        }
        self.r_cost(a, b)
    }
}

impl TextDistance for Editex {
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        (s1.chars().count().max(s2.chars().count()) as i32 * self.mismatch_cost) as f64
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 0.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return self.maximum(s1, s2);
        }

        let max_length = self.maximum(s1, s2);

        // Prepend space, uppercase
        let s1u = format!(" {}", s1.to_uppercase());
        let s2u = format!(" {}", s2.to_uppercase());
        let c1: Vec<char> = s1u.chars().collect();
        let c2: Vec<char> = s2u.chars().collect();
        let len1 = c1.len() - 1;
        let len2 = c2.len() - 1;

        let mut d: HashMap<(usize, usize), i32> = HashMap::new();
        d.insert((0, 0), 0);

        if !self.local {
            for i in 1..=len1 {
                let prev = *d.get(&(i - 1, 0)).unwrap_or(&0);
                d.insert((i, 0), prev + self.d_cost(c1[i - 1], c1[i]));
            }
        } else {
            for i in 1..=len1 {
                d.insert((i, 0), 0);
            }
        }
        for j in 1..=len2 {
            let prev = *d.get(&(0, j - 1)).unwrap_or(&0);
            d.insert((0, j), prev + self.d_cost(c2[j - 1], c2[j]));
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let v1 = *d.get(&(i - 1, j)).unwrap_or(&0) + self.d_cost(c1[i - 1], c1[i]);
                let v2 = *d.get(&(i, j - 1)).unwrap_or(&0) + self.d_cost(c2[j - 1], c2[j]);
                let v3 = *d.get(&(i - 1, j - 1)).unwrap_or(&0) + self.r_cost(c1[i], c2[j]);
                d.insert((i, j), v1.min(v2).min(v3));
            }
        }

        let distance = *d.get(&(len1, len2)).unwrap_or(&0);
        (distance as f64).min(max_length)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mra() {
        let m = Mra::new();
        assert!(m.similarity("Smith", "Smith") > 0.0);
    }

    #[test]
    fn test_editex() {
        let e = Editex::new();
        assert_eq!(e.distance("abc", "abc"), 0.0);
        assert!(e.distance("abc", "def") > 0.0);
    }
}
