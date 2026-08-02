use std::collections::HashMap;

/// Helper: split a string into q-grams (character n-grams).
pub fn find_ngrams(s: &str, n: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < n {
        return vec![];
    }
    chars.windows(n).map(|w| w.iter().collect()).collect()
}

/// Prepare a sequence based on qval.
/// qval == 0: split by whitespace (words)
/// qval == 1: characters (identity)
/// qval > 1: character n-grams
pub fn prepare_sequence(s: &str, qval: usize) -> Vec<String> {
    if qval == 0 {
        s.split_whitespace().map(|w| w.to_string()).collect()
    } else if qval == 1 {
        s.chars().map(|c| c.to_string()).collect()
    } else {
        find_ngrams(s, qval)
    }
}

/// Build a counter (frequency map) from a sequence of tokens.
pub fn counter(tokens: &[String]) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for t in tokens {
        *map.entry(t.clone()).or_insert(0) += 1;
    }
    map
}

/// Intersect two counters (min of each key).
pub fn intersect_counters(
    a: &HashMap<String, usize>,
    b: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (k, va) in a {
        if let Some(vb) = b.get(k) {
            result.insert(k.clone(), (*va).min(*vb));
        }
    }
    result
}

/// Union two counters (max of each key).
pub fn union_counters(
    a: &HashMap<String, usize>,
    b: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut result = a.clone();
    for (k, vb) in b {
        let entry = result.entry(k.clone()).or_insert(0);
        *entry = (*entry).max(*vb);
    }
    result
}

/// Sum two counters.
pub fn sum_counters(
    a: &HashMap<String, usize>,
    b: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut result = a.clone();
    for (k, vb) in b {
        *result.entry(k.clone()).or_insert(0) += vb;
    }
    result
}

/// Count total elements in a counter.
pub fn count_counter(c: &HashMap<String, usize>, as_set: bool) -> usize {
    if as_set {
        c.len()
    } else {
        c.values().sum()
    }
}

/// Subtract counter b from counter a (elements in a not covered by b).
pub fn subtract_counter(
    a: &HashMap<String, usize>,
    b: &HashMap<String, usize>,
) -> HashMap<String, usize> {
    let mut result = HashMap::new();
    for (k, va) in a {
        let vb = b.get(k).copied().unwrap_or(0);
        if *va > vb {
            result.insert(k.clone(), va - vb);
        }
    }
    result
}

// ---------------------------------------------------------------------------
// Core Traits
// ---------------------------------------------------------------------------

/// The base trait for all distance-based algorithms.
///
/// An algorithm that natively computes **distance** (lower = more similar).
/// `similarity` is derived as `maximum - distance`.
/// 
/// # Examples
/// 
/// ```rust
/// use textdistance::{Levenshtein, TextDistance};
/// 
/// let lev = Levenshtein::new();
/// let d = lev.distance("test", "text");
/// assert_eq!(d, 1.0);
/// ```
pub trait TextDistance {
    /// Raw distance between two strings. Lower means more similar.
    fn distance(&self, s1: &str, s2: &str) -> f64;

    /// Maximum possible distance for the given inputs.
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        s1.chars().count().max(s2.chars().count()) as f64
    }

    /// Raw similarity. Default: `maximum - distance`.
    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        self.maximum(s1, s2) - self.distance(s1, s2)
    }

    /// Normalized distance in [0, 1].
    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0.0 {
            return 0.0;
        }
        self.distance(s1, s2) / max
    }

    /// Normalized similarity in [0, 1]. Default: `1 - normalized_distance`.
    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }
}

/// The base trait for all similarity-based algorithms.
///
/// An algorithm that natively computes **similarity** (higher = more similar).
/// `distance` is derived as `maximum - similarity`.
/// 
/// # Examples
/// 
/// ```rust
/// use textdistance::{Jaccard, TextSimilarity};
/// 
/// let j = Jaccard::new();
/// let s = j.similarity("test", "text");
/// assert_eq!(s, 3.0); // t, e, t (wait, Jaccard uses bigrams or char tokens usually. Actually 3 common tokens)
/// ```
pub trait TextSimilarity {
    /// Raw similarity between two strings. Higher means more similar.
    fn similarity(&self, s1: &str, s2: &str) -> f64;

    /// Maximum possible similarity for the given inputs.
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        s1.chars().count().max(s2.chars().count()) as f64
    }

    /// Raw distance. Default: `maximum - similarity`.
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        self.maximum(s1, s2) - self.similarity(s1, s2)
    }

    /// Normalized distance in [0, 1].
    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let max = self.maximum(s1, s2);
        if max == 0.0 {
            return 0.0;
        }
        self.distance(s1, s2) / max
    }

    /// Normalized similarity in [0, 1]. Default: `1 - normalized_distance`.
    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }
}
