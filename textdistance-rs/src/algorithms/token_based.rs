//! Token-based string similarity algorithms.
//!
//! Algorithms that compare strings by treating them as bags/sets of tokens
//! (characters or q-grams) and computing set-theoretic measures.

use super::base::{
    counter, count_counter, intersect_counters, prepare_sequence, subtract_counter,
    union_counters, TextDistance, TextSimilarity,
};

// ---------------------------------------------------------------------------
// Jaccard
// ---------------------------------------------------------------------------

/// Jaccard similarity coefficient.
///
/// <https://en.wikipedia.org/wiki/Jaccard_index>
#[derive(Debug, Clone)]
pub struct Jaccard {
    pub qval: usize,
    pub as_set: bool,
}

impl Default for Jaccard {
    fn default() -> Self {
        Self { qval: 1, as_set: false }
    }
}

impl Jaccard {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Jaccard {
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
        let t1 = prepare_sequence(s1, self.qval);
        let t2 = prepare_sequence(s2, self.qval);
        let c1 = counter(&t1);
        let c2 = counter(&t2);
        let intersection = count_counter(&intersect_counters(&c1, &c2), self.as_set);
        let union = count_counter(&union_counters(&c1, &c2), self.as_set);
        if union == 0 {
            return 0.0;
        }
        intersection as f64 / union as f64
    }
}

// ---------------------------------------------------------------------------
// Sorensen (Dice)
// ---------------------------------------------------------------------------

/// Sørensen–Dice coefficient.
///
/// <https://en.wikipedia.org/wiki/S%C3%B8rensen%E2%80%93Dice_coefficient>
#[derive(Debug, Clone)]
pub struct Sorensen {
    pub qval: usize,
    pub as_set: bool,
}

impl Default for Sorensen {
    fn default() -> Self {
        Self { qval: 1, as_set: false }
    }
}

impl Sorensen {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Sorensen {
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
        let t1 = prepare_sequence(s1, self.qval);
        let t2 = prepare_sequence(s2, self.qval);
        let c1 = counter(&t1);
        let c2 = counter(&t2);
        let count = count_counter(&c1, self.as_set) + count_counter(&c2, self.as_set);
        let intersection = count_counter(&intersect_counters(&c1, &c2), self.as_set);
        if count == 0 {
            return 0.0;
        }
        2.0 * intersection as f64 / count as f64
    }
}

// ---------------------------------------------------------------------------
// Tversky
// ---------------------------------------------------------------------------

/// Tversky index.
///
/// <https://en.wikipedia.org/wiki/Tversky_index>
#[derive(Debug, Clone)]
pub struct Tversky {
    pub qval: usize,
    pub alpha: f64,
    pub beta: f64,
    pub as_set: bool,
}

impl Default for Tversky {
    fn default() -> Self {
        Self {
            qval: 1,
            alpha: 1.0,
            beta: 1.0,
            as_set: false,
        }
    }
}

impl Tversky {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Tversky {
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
        let t1 = prepare_sequence(s1, self.qval);
        let t2 = prepare_sequence(s2, self.qval);
        let c1 = counter(&t1);
        let c2 = counter(&t2);
        let intersection = count_counter(&intersect_counters(&c1, &c2), self.as_set) as f64;
        let s1_count = count_counter(&c1, self.as_set) as f64;
        let s2_count = count_counter(&c2, self.as_set) as f64;

        let result = intersection
            + self.alpha * (s1_count - intersection)
            + self.beta * (s2_count - intersection);
        if result == 0.0 {
            return 0.0;
        }
        intersection / result
    }
}

// ---------------------------------------------------------------------------
// Overlap
// ---------------------------------------------------------------------------

/// Overlap coefficient.
///
/// <https://en.wikipedia.org/wiki/Overlap_coefficient>
#[derive(Debug, Clone)]
pub struct Overlap {
    pub qval: usize,
    pub as_set: bool,
}

impl Default for Overlap {
    fn default() -> Self {
        Self { qval: 1, as_set: false }
    }
}

impl Overlap {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Overlap {
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
        let t1 = prepare_sequence(s1, self.qval);
        let t2 = prepare_sequence(s2, self.qval);
        let c1 = counter(&t1);
        let c2 = counter(&t2);
        let intersection = count_counter(&intersect_counters(&c1, &c2), self.as_set) as f64;
        let min_count = (count_counter(&c1, self.as_set) as f64)
            .min(count_counter(&c2, self.as_set) as f64);
        if min_count == 0.0 {
            return 0.0;
        }
        intersection / min_count
    }
}

// ---------------------------------------------------------------------------
// Cosine
// ---------------------------------------------------------------------------

/// Cosine similarity (Ochiai coefficient).
///
/// <https://en.wikipedia.org/wiki/Cosine_similarity>
#[derive(Debug, Clone)]
pub struct Cosine {
    pub qval: usize,
    pub as_set: bool,
}

impl Default for Cosine {
    fn default() -> Self {
        Self { qval: 1, as_set: false }
    }
}

impl Cosine {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Cosine {
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
        let t1 = prepare_sequence(s1, self.qval);
        let t2 = prepare_sequence(s2, self.qval);
        let c1 = counter(&t1);
        let c2 = counter(&t2);
        let intersection = count_counter(&intersect_counters(&c1, &c2), self.as_set) as f64;
        let s1_count = count_counter(&c1, self.as_set) as f64;
        let s2_count = count_counter(&c2, self.as_set) as f64;
        let prod = s1_count * s2_count;
        if prod == 0.0 {
            return 0.0;
        }
        intersection / prod.sqrt()
    }
}

// ---------------------------------------------------------------------------
// Tanimoto
// ---------------------------------------------------------------------------

/// Tanimoto distance (log2 of Jaccard coefficient).
///
/// <https://en.wikipedia.org/wiki/Jaccard_index#Tanimoto_similarity_and_distance>
#[derive(Debug, Clone)]
pub struct Tanimoto {
    pub qval: usize,
    pub as_set: bool,
}

impl Default for Tanimoto {
    fn default() -> Self {
        Self { qval: 1, as_set: false }
    }
}

impl Tanimoto {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextDistance for Tanimoto {
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        let jaccard = Jaccard {
            qval: self.qval,
            as_set: self.as_set,
        };
        let result = jaccard.similarity(s1, s2);
        if result == 0.0 {
            f64::NEG_INFINITY
        } else {
            result.log2()
        }
    }

    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        0.0 // Tanimoto range is (-inf, 0]
    }
}

// ---------------------------------------------------------------------------
// MongeElkan
// ---------------------------------------------------------------------------

/// Monge-Elkan similarity.
///
/// <https://www.academia.edu/200314>
#[derive(Debug, Clone)]
pub struct MongeElkan {
    pub symmetric: bool,
}

impl Default for MongeElkan {
    fn default() -> Self {
        Self { symmetric: false }
    }
}

impl MongeElkan {
    pub fn new() -> Self {
        Self::default()
    }

    fn calc_one_way(&self, s1: &str, s2: &str) -> f64 {
        let words1: Vec<&str> = s1.split_whitespace().collect();
        let words2: Vec<&str> = s2.split_whitespace().collect();
        if words1.is_empty() {
            return 0.0;
        }
        let dl = super::edit_based::DamerauLevenshtein::new();
        let mut total = 0.0;
        for w1 in &words1 {
            let mut max_sim = f64::NEG_INFINITY;
            for w2 in &words2 {
                let max_val = w1.chars().count().max(w2.chars().count()) as f64;
                let sim = max_val - dl.distance(w1, w2);
                if sim > max_sim {
                    max_sim = sim;
                }
            }
            total += max_sim;
        }
        total / words1.len() as f64 / words2.len().max(1) as f64
    }
}

impl TextSimilarity for MongeElkan {
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        let words: Vec<&str> = s1.split_whitespace().chain(s2.split_whitespace()).collect();
        if words.is_empty() {
            return 0.0;
        }
        let max_word_len = words.iter().map(|w| w.chars().count()).max().unwrap_or(0);
        max_word_len as f64
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return self.maximum(s1, s2);
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }
        if self.symmetric {
            let a = self.calc_one_way(s1, s2);
            let b = self.calc_one_way(s2, s1);
            (a + b) / 2.0
        } else {
            self.calc_one_way(s1, s2)
        }
    }
}

// ---------------------------------------------------------------------------
// Bag
// ---------------------------------------------------------------------------

/// Bag distance.
///
/// <http://www-db.disi.unibo.it/research/papers/SPIRE02.pdf>
#[derive(Debug, Clone, Default)]
pub struct Bag {
    pub qval: usize,
}

impl Bag {
    pub fn new() -> Self {
        Self { qval: 1 }
    }
}

impl TextDistance for Bag {
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 0.0;
        }
        let t1 = prepare_sequence(s1, self.qval);
        let t2 = prepare_sequence(s2, self.qval);
        let c1 = counter(&t1);
        let c2 = counter(&t2);
        let intersection = intersect_counters(&c1, &c2);
        let diff1 = count_counter(&subtract_counter(&c1, &intersection), false);
        let diff2 = count_counter(&subtract_counter(&c2, &intersection), false);
        diff1.max(diff2) as f64
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jaccard() {
        let j = Jaccard::new();
        assert_eq!(j.similarity("abc", "abc"), 1.0);
        assert!(j.similarity("abc", "def") == 0.0);
    }

    #[test]
    fn test_sorensen() {
        let s = Sorensen::new();
        assert_eq!(s.similarity("abc", "abc"), 1.0);
    }

    #[test]
    fn test_cosine() {
        let c = Cosine::new();
        assert_eq!(c.similarity("abc", "abc"), 1.0);
    }

    #[test]
    fn test_bag() {
        let b = Bag::new();
        assert_eq!(b.distance("abc", "abc"), 0.0);
        assert!(b.distance("abc", "abcdef") > 0.0);
    }

    #[test]
    fn test_tanimoto_identical() {
        let t = Tanimoto::new();
        // log2(1.0) = 0.0 for identical strings
        assert_eq!(t.distance("abc", "abc"), 0.0);
    }
}
