//! Edit-based string distance algorithms.
//!
//! Algorithms that measure distance by counting the minimum edit operations
//! (insertions, deletions, substitutions, transpositions) needed to transform
//! one sequence into another.

use super::base::{TextDistance, TextSimilarity};

// ---------------------------------------------------------------------------
// Hamming
// ---------------------------------------------------------------------------

/// Hamming distance: number of positions where corresponding characters differ.
///
/// <https://en.wikipedia.org/wiki/Hamming_distance>
#[derive(Debug, Clone)]
pub struct Hamming {
    /// If true, truncate the longer sequence to the length of the shorter one.
    pub truncate: bool,
}

impl Default for Hamming {
    fn default() -> Self {
        Self { truncate: false }
    }
}

impl Hamming {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_truncate(truncate: bool) -> Self {
        Self { truncate }
    }
}

impl TextDistance for Hamming {
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();

        if self.truncate {
            let min_len = c1.len().min(c2.len());
            c1[..min_len]
                .iter()
                .zip(&c2[..min_len])
                .filter(|(a, b)| a != b)
                .count() as f64
        } else {
            let max_len = c1.len().max(c2.len());
            let mut diff = 0usize;
            for i in 0..max_len {
                let a = c1.get(i);
                let b = c2.get(i);
                if a != b {
                    diff += 1;
                }
            }
            diff as f64
        }
    }
}

// ---------------------------------------------------------------------------
// Levenshtein
// ---------------------------------------------------------------------------

/// Levenshtein distance: minimum edit operations (insert, delete, substitute).
///
/// <https://en.wikipedia.org/wiki/Levenshtein_distance>
#[derive(Debug, Clone, Default)]
pub struct Levenshtein;

impl Levenshtein {
    pub fn new() -> Self {
        Self
    }
}

impl TextDistance for Levenshtein {
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let rows = c1.len() + 1;
        let cols = c2.len() + 1;

        let mut cur: Vec<usize> = (0..cols).collect();

        for r in 1..rows {
            let mut prev = cur.clone();
            cur[0] = r;
            for c in 1..cols {
                let deletion = prev[c] + 1;
                let insertion = cur[c - 1] + 1;
                let cost = if c1[r - 1] == c2[c - 1] { 0 } else { 1 };
                let edit = prev[c - 1] + cost;
                cur[c] = deletion.min(insertion).min(edit);
            }
            prev = cur.clone();
            let _ = prev; // suppress unused warning
        }
        cur[cols - 1] as f64
    }
}

// ---------------------------------------------------------------------------
// Damerau-Levenshtein
// ---------------------------------------------------------------------------

/// Damerau-Levenshtein distance: Levenshtein + transpositions.
///
/// <https://en.wikipedia.org/wiki/Damerau%E2%80%93Levenshtein_distance>
#[derive(Debug, Clone)]
pub struct DamerauLevenshtein {
    /// If true, use the restricted (optimal string alignment) variant.
    /// If false, use the unrestricted variant.
    pub restricted: bool,
}

impl Default for DamerauLevenshtein {
    fn default() -> Self {
        Self { restricted: true }
    }
}

impl DamerauLevenshtein {
    pub fn new() -> Self {
        Self::default()
    }

    fn restricted_distance(&self, s1: &str, s2: &str) -> usize {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        // Use a HashMap for the DP matrix with (i, j) keys, matching Python's dict approach.
        use std::collections::HashMap;
        let mut d: HashMap<(i64, i64), usize> = HashMap::new();

        for i in -1..=(len1 as i64) {
            d.insert((i, -1), (i + 1) as usize);
        }
        for j in -1..=(len2 as i64) {
            d.insert((-1, j), (j + 1) as usize);
        }

        for i in 0..len1 {
            for j in 0..len2 {
                let cost = if c1[i] == c2[j] { 0 } else { 1 };
                let ii = i as i64;
                let jj = j as i64;

                let val = (*d.get(&(ii - 1, jj)).unwrap() + 1)
                    .min(*d.get(&(ii, jj - 1)).unwrap() + 1)
                    .min(*d.get(&(ii - 1, jj - 1)).unwrap() + cost);
                d.insert((ii, jj), val);

                // transposition
                if i == 0 || j == 0 {
                    continue;
                }
                if c1[i] != c2[j - 1] {
                    continue;
                }
                if c1[i - 1] != c2[j] {
                    continue;
                }
                let trans = *d.get(&(ii - 2, jj - 2)).unwrap() + cost;
                let cur = *d.get(&(ii, jj)).unwrap();
                d.insert((ii, jj), cur.min(trans));
            }
        }
        *d.get(&(len1 as i64 - 1, len2 as i64 - 1)).unwrap()
    }

    fn unrestricted_distance(&self, s1: &str, s2: &str) -> usize {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        use std::collections::HashMap;
        let mut d: HashMap<(i64, i64), usize> = HashMap::new();
        let mut da: HashMap<char, usize> = HashMap::new();

        let maxdist = len1 + len2;
        d.insert((-1, -1), maxdist);

        for i in 0..=len1 {
            d.insert((i as i64, -1), maxdist);
            d.insert((i as i64, 0), i);
        }
        for j in 0..=len2 {
            d.insert((-1, j as i64), maxdist);
            d.insert((0, j as i64), j);
        }

        for i in 1..=len1 {
            let mut db: usize = 0;
            for j in 1..=len2 {
                let i1 = *da.get(&c2[j - 1]).unwrap_or(&0) as i64;
                let j1 = db as i64;
                let cost = if c1[i - 1] == c2[j - 1] {
                    db = j;
                    0usize
                } else {
                    1usize
                };

                let ii = i as i64;
                let jj = j as i64;

                let sub = *d.get(&(ii - 1, jj - 1)).unwrap() + cost;
                let ins = *d.get(&(ii, jj - 1)).unwrap() + 1;
                let del = *d.get(&(ii - 1, jj)).unwrap() + 1;
                let trans_cost = *d.get(&(i1 - 1, j1 - 1)).unwrap()
                    + (i as i64 - i1) as usize
                    - 1
                    + (j as i64 - j1) as usize;

                d.insert((ii, jj), sub.min(ins).min(del).min(trans_cost));
            }
            da.insert(c1[i - 1], i);
        }
        *d.get(&(len1 as i64, len2 as i64)).unwrap()
    }
}

impl TextDistance for DamerauLevenshtein {
    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 0.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return s1.chars().count().max(s2.chars().count()) as f64;
        }
        if self.restricted {
            self.restricted_distance(s1, s2) as f64
        } else {
            self.unrestricted_distance(s1, s2) as f64
        }
    }
}

// ---------------------------------------------------------------------------
// Jaro / Jaro-Winkler
// ---------------------------------------------------------------------------

/// Jaro-Winkler similarity.
///
/// <https://en.wikipedia.org/wiki/Jaro%E2%80%93Winkler_distance>
#[derive(Debug, Clone)]
pub struct JaroWinkler {
    pub winklerize: bool,
    pub long_tolerance: bool,
    pub prefix_weight: f64,
}

impl Default for JaroWinkler {
    fn default() -> Self {
        Self {
            winklerize: true,
            long_tolerance: false,
            prefix_weight: 0.1,
        }
    }
}

impl JaroWinkler {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for JaroWinkler {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return 1.0;
        }
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let s1_len = c1.len();
        let s2_len = c2.len();

        if s1_len == 0 || s2_len == 0 {
            return 0.0;
        }

        let min_len = s1_len.min(s2_len);
        let search_range = if (s1_len.max(s2_len) / 2) > 0 {
            (s1_len.max(s2_len) / 2) - 1
        } else {
            0
        };

        let mut s1_flags = vec![false; s1_len];
        let mut s2_flags = vec![false; s2_len];

        // Count & flag matched pairs
        let mut common_chars: usize = 0;
        for i in 0..s1_len {
            let low = if i > search_range { i - search_range } else { 0 };
            let hi = (i + search_range).min(s2_len - 1);
            for j in low..=hi {
                if !s2_flags[j] && c2[j] == c1[i] {
                    s1_flags[i] = true;
                    s2_flags[j] = true;
                    common_chars += 1;
                    break;
                }
            }
        }

        if common_chars == 0 {
            return 0.0;
        }

        // Count transpositions
        let mut k = 0usize;
        let mut trans_count = 0usize;
        for i in 0..s1_len {
            if s1_flags[i] {
                let mut j = k;
                while j < s2_len {
                    if s2_flags[j] {
                        k = j + 1;
                        if c1[i] != c2[j] {
                            trans_count += 1;
                        }
                        break;
                    }
                    j += 1;
                }
            }
        }
        trans_count /= 2;

        let cc = common_chars as f64;
        let mut weight = cc / s1_len as f64 + cc / s2_len as f64
            + (cc - trans_count as f64) / cc;
        weight /= 3.0;

        if !self.winklerize {
            return weight;
        }
        if weight <= 0.7 {
            return weight;
        }

        // Winkler modification: boost for common prefix (up to 4 chars)
        let j = min_len.min(4);
        let mut i = 0;
        while i < j && c1[i] == c2[i] {
            i += 1;
        }
        if i > 0 {
            weight += i as f64 * self.prefix_weight * (1.0 - weight);
        }

        // Long string adjustment
        if !self.long_tolerance || min_len <= 4 {
            return weight;
        }
        if common_chars <= i + 1 || 2 * common_chars < min_len + i {
            return weight;
        }
        let tmp = (common_chars - i - 1) as f64 / (s1_len + s2_len - i * 2 + 2) as f64;
        weight += (1.0 - weight) * tmp;
        weight
    }
}

/// Jaro similarity (Jaro-Winkler without the Winkler boost).
#[derive(Debug, Clone, Default)]
pub struct Jaro;

impl Jaro {
    pub fn new() -> Self {
        Self
    }
}

impl TextSimilarity for Jaro {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let jw = JaroWinkler {
            winklerize: false,
            long_tolerance: false,
            prefix_weight: 0.1,
        };
        jw.similarity(s1, s2)
    }
}

// ---------------------------------------------------------------------------
// Needleman-Wunsch
// ---------------------------------------------------------------------------

/// Needleman-Wunsch global alignment score.
///
/// <https://en.wikipedia.org/wiki/Needleman%E2%80%93Wunsch_algorithm>
#[derive(Debug, Clone)]
pub struct NeedlemanWunsch {
    pub gap_cost: f64,
}

impl Default for NeedlemanWunsch {
    fn default() -> Self {
        Self { gap_cost: 1.0 }
    }
}

impl NeedlemanWunsch {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for NeedlemanWunsch {
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        s1.chars().count().max(s2.chars().count()) as f64
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        let mut dist_mat = vec![vec![0.0f64; len2 + 1]; len1 + 1];

        for i in 0..=len1 {
            dist_mat[i][0] = -(i as f64 * self.gap_cost);
        }
        for j in 0..=len2 {
            dist_mat[0][j] = -(j as f64 * self.gap_cost);
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let sim = if c1[i - 1] == c2[j - 1] { 1.0 } else { 0.0 };
                let match_score = dist_mat[i - 1][j - 1] + sim;
                let delete = dist_mat[i - 1][j] - self.gap_cost;
                let insert = dist_mat[i][j - 1] - self.gap_cost;
                dist_mat[i][j] = match_score.max(delete).max(insert);
            }
        }
        dist_mat[len1][len2]
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        -1.0 * self.similarity(s1, s2)
    }

    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.chars().count().max(s2.chars().count()) as f64;
        let minimum = -max_len * self.gap_cost;
        let maximum = max_len;
        if maximum == 0.0 {
            return 0.0;
        }
        (self.distance(s1, s2) - minimum) / (maximum - minimum)
    }

    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        let max_len = s1.chars().count().max(s2.chars().count()) as f64;
        let minimum = -max_len * self.gap_cost;
        let maximum = max_len;
        if maximum == 0.0 {
            return 1.0;
        }
        (self.similarity(s1, s2) - minimum) / (maximum * 2.0)
    }
}

// ---------------------------------------------------------------------------
// Smith-Waterman
// ---------------------------------------------------------------------------

/// Smith-Waterman local alignment score.
///
/// <https://en.wikipedia.org/wiki/Smith%E2%80%93Waterman_algorithm>
#[derive(Debug, Clone)]
pub struct SmithWaterman {
    pub gap_cost: f64,
}

impl Default for SmithWaterman {
    fn default() -> Self {
        Self { gap_cost: 1.0 }
    }
}

impl SmithWaterman {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for SmithWaterman {
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        s1.chars().count().min(s2.chars().count()) as f64
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        if s1 == s2 {
            return self.maximum(s1, s2);
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        let mut dist_mat = vec![vec![0.0f64; len2 + 1]; len1 + 1];

        for i in 1..=len1 {
            for j in 1..=len2 {
                let sim = if c1[i - 1] == c2[j - 1] { 1.0 } else { 0.0 };
                let match_score = dist_mat[i - 1][j - 1] + sim;
                let delete = dist_mat[i - 1][j] - self.gap_cost;
                let insert = dist_mat[i][j - 1] - self.gap_cost;
                dist_mat[i][j] = 0.0f64.max(match_score).max(delete).max(insert);
            }
        }
        dist_mat[len1][len2]
    }
}

// ---------------------------------------------------------------------------
// Gotoh
// ---------------------------------------------------------------------------

/// Gotoh score: Needleman-Wunsch with affine gap penalties.
///
/// <https://www.cs.umd.edu/class/spring2003/cmsc838t/papers/gotoh1982.pdf>
#[derive(Debug, Clone)]
pub struct Gotoh {
    pub gap_open: f64,
    pub gap_ext: f64,
}

impl Default for Gotoh {
    fn default() -> Self {
        Self {
            gap_open: 1.0,
            gap_ext: 0.4,
        }
    }
}

impl Gotoh {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Gotoh {
    fn maximum(&self, s1: &str, s2: &str) -> f64 {
        s1.chars().count().min(s2.chars().count()) as f64
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let c1: Vec<char> = s1.chars().collect();
        let c2: Vec<char> = s2.chars().collect();
        let len1 = c1.len();
        let len2 = c2.len();

        let mut d = vec![vec![0.0f64; len2 + 1]; len1 + 1];
        let mut p = vec![vec![0.0f64; len2 + 1]; len1 + 1];
        let mut q = vec![vec![0.0f64; len2 + 1]; len1 + 1];

        p[0][0] = f64::NEG_INFINITY;
        q[0][0] = f64::NEG_INFINITY;

        for i in 1..=len1 {
            d[i][0] = f64::NEG_INFINITY;
            p[i][0] = -self.gap_open - self.gap_ext * (i as f64 - 1.0);
            q[i][0] = f64::NEG_INFINITY;
            if i <= len2 {
                // intentionally skipped for len2+1 bound check
            }
        }
        // Set q[i][1] = -gap_open for all i
        for i in 1..=len1 {
            if 1 <= len2 {
                q[i][1] = -self.gap_open;
            }
        }
        for j in 1..=len2 {
            d[0][j] = f64::NEG_INFINITY;
            p[0][j] = f64::NEG_INFINITY;
            q[0][j] = -self.gap_open - self.gap_ext * (j as f64 - 1.0);
        }
        // Set p[1][j] = -gap_open for all j
        for j in 1..=len2 {
            if 1 <= len1 {
                p[1][j] = -self.gap_open;
            }
        }

        for i in 1..=len1 {
            for j in 1..=len2 {
                let sim = if c1[i - 1] == c2[j - 1] { 1.0 } else { 0.0 };
                d[i][j] = (d[i - 1][j - 1] + sim)
                    .max(p[i - 1][j - 1] + sim)
                    .max(q[i - 1][j - 1] + sim);
                p[i][j] = (d[i - 1][j] - self.gap_open).max(p[i - 1][j] - self.gap_ext);
                q[i][j] = (d[i][j - 1] - self.gap_open).max(q[i][j - 1] - self.gap_ext);
            }
        }

        d[len1][len2].max(p[len1][len2]).max(q[len1][len2])
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        -1.0 * self.similarity(s1, s2)
    }

    fn normalized_distance(&self, s1: &str, s2: &str) -> f64 {
        let min_len = s1.chars().count().min(s2.chars().count()) as f64;
        let minimum = -min_len;
        let maximum = min_len;
        if maximum == 0.0 {
            return 0.0;
        }
        (self.distance(s1, s2) - minimum) / (maximum - minimum)
    }

    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64 {
        1.0 - self.normalized_distance(s1, s2)
    }
}

// ---------------------------------------------------------------------------
// StrCmp95
// ---------------------------------------------------------------------------

/// strcmp95 similarity.
///
/// <http://cpansearch.perl.org/src/SCW/Text-JaroWinkler-0.1/strcmp95.c>
#[derive(Debug, Clone)]
pub struct StrCmp95 {
    pub long_strings: bool,
}

impl Default for StrCmp95 {
    fn default() -> Self {
        Self { long_strings: false }
    }
}

impl StrCmp95 {
    pub fn new() -> Self {
        Self::default()
    }
}

const SP_MX: &[(&str, &str)] = &[
    ("A", "E"), ("A", "I"), ("A", "O"), ("A", "U"), ("B", "V"), ("E", "I"),
    ("E", "O"), ("E", "U"), ("I", "O"), ("I", "U"), ("O", "U"), ("I", "Y"),
    ("E", "Y"), ("C", "G"), ("E", "F"), ("W", "U"), ("W", "V"), ("X", "K"),
    ("S", "Z"), ("X", "S"), ("Q", "C"), ("U", "V"), ("M", "N"), ("L", "I"),
    ("Q", "O"), ("P", "R"), ("I", "J"), ("2", "Z"), ("5", "S"), ("8", "B"),
    ("1", "I"), ("1", "L"), ("0", "O"), ("0", "Q"), ("C", "K"), ("G", "J"),
];

fn in_range(c: char) -> bool {
    let code = c as u32;
    code > 0 && code < 91
}

impl TextSimilarity for StrCmp95 {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn similarity(&self, s1: &str, s2: &str) -> f64 {
        let s1: Vec<char> = s1.trim().to_uppercase().chars().collect();
        let s2: Vec<char> = s2.trim().to_uppercase().chars().collect();

        if s1 == s2 {
            return 1.0;
        }
        if s1.is_empty() || s2.is_empty() {
            return 0.0;
        }

        let len_s1 = s1.len();
        let len_s2 = s2.len();

        use std::collections::HashMap;
        let mut adjwt: HashMap<(char, char), i32> = HashMap::new();
        for &(c1, c2) in SP_MX {
            let a = c1.chars().next().unwrap();
            let b = c2.chars().next().unwrap();
            adjwt.insert((a, b), 3);
            adjwt.insert((b, a), 3);
        }

        let (search_range_init, minv) = if len_s1 > len_s2 {
            (len_s1, len_s2)
        } else {
            (len_s2, len_s1)
        };

        let mut s1_flag = vec![0i32; search_range_init];
        let mut s2_flag = vec![0i32; search_range_init];
        let search_range = if search_range_init / 2 > 0 {
            search_range_init / 2 - 1
        } else {
            0
        };

        // Count matched pairs
        let mut num_com = 0usize;
        let yl1 = if len_s2 > 0 { len_s2 - 1 } else { 0 };
        for i in 0..len_s1 {
            let lowlim = if i > search_range { i - search_range } else { 0 };
            let hilim = (i + search_range).min(yl1);
            for j in lowlim..=hilim {
                if s2_flag[j] == 0 && s2[j] == s1[i] {
                    s2_flag[j] = 1;
                    s1_flag[i] = 1;
                    num_com += 1;
                    break;
                }
            }
        }

        if num_com == 0 {
            return 0.0;
        }

        // Count transpositions
        let mut k = 0usize;
        let mut n_trans = 0usize;
        for i in 0..len_s1 {
            if s1_flag[i] == 0 {
                continue;
            }
            for j in k..len_s2 {
                if s2_flag[j] != 0 {
                    k = j + 1;
                    if s1[i] != s2[j] {
                        n_trans += 1;
                    }
                    break;
                }
            }
        }
        n_trans /= 2;

        // Adjust for similarities in unmatched characters
        let mut n_simi = 0i32;
        if minv > num_com {
            for i in 0..len_s1 {
                if s1_flag[i] != 0 {
                    continue;
                }
                if !in_range(s1[i]) {
                    continue;
                }
                for j in 0..len_s2 {
                    if s2_flag[j] != 0 {
                        continue;
                    }
                    if !in_range(s2[j]) {
                        continue;
                    }
                    if let Some(&adj) = adjwt.get(&(s1[i], s2[j])) {
                        n_simi += adj;
                        s2_flag[j] = 2;
                        break;
                    }
                }
            }
        }
        let num_sim = n_simi as f64 / 10.0 + num_com as f64;

        // Main weight
        let mut weight = num_sim / len_s1 as f64 + num_sim / len_s2 as f64
            + (num_com as f64 - n_trans as f64) / num_com as f64;
        weight /= 3.0;

        if weight <= 0.7 {
            return weight;
        }

        // Common prefix (up to 4 chars)
        let j = minv.min(4);
        let mut i = 0usize;
        for (sc1, sc2) in s1.iter().zip(s2.iter()) {
            if i >= j {
                break;
            }
            if sc1 != sc2 {
                break;
            }
            if sc1.is_ascii_digit() {
                break;
            }
            i += 1;
        }
        if i > 0 {
            weight += i as f64 * 0.1 * (1.0 - weight);
        }

        // Long string adjustment
        if !self.long_strings {
            return weight;
        }
        if minv <= 4 {
            return weight;
        }
        if num_com <= i + 1 || 2 * num_com < minv + i {
            return weight;
        }
        if s1[0].is_ascii_digit() {
            return weight;
        }
        let res = (num_com - i - 1) as f64 / (len_s1 + len_s2 - i * 2 + 2) as f64;
        weight += (1.0 - weight) * res;
        weight
    }
}

// ---------------------------------------------------------------------------
// MLIPNS
// ---------------------------------------------------------------------------

/// MLIPNS similarity.
///
/// <http://www.sial.iias.spb.su/files/386-386-1-PB.pdf>
#[derive(Debug, Clone)]
pub struct Mlipns {
    pub threshold: f64,
    pub maxmismatches: usize,
}

impl Default for Mlipns {
    fn default() -> Self {
        Self {
            threshold: 0.25,
            maxmismatches: 2,
        }
    }
}

impl Mlipns {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TextSimilarity for Mlipns {
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

        let mut ham = Hamming::new().distance(s1, s2);
        let mut maxlen = s1.chars().count().max(s2.chars().count()) as f64;
        let mut mismatches = 0usize;

        while mismatches <= self.maxmismatches {
            if maxlen == 0.0 {
                return 1.0;
            }
            if 1.0 - (maxlen - ham) / maxlen <= self.threshold {
                return 1.0;
            }
            mismatches += 1;
            ham -= 1.0;
            maxlen -= 1.0;
        }

        if maxlen == 0.0 {
            return 1.0;
        }
        0.0
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hamming() {
        let h = Hamming::new();
        assert_eq!(h.distance("karolin", "kathrin"), 3.0);
        assert_eq!(h.distance("", ""), 0.0);
        assert_eq!(h.distance("a", ""), 1.0);
    }

    #[test]
    fn test_levenshtein() {
        let l = Levenshtein::new();
        assert_eq!(l.distance("kitten", "sitting"), 3.0);
        assert_eq!(l.distance("", ""), 0.0);
        assert_eq!(l.distance("abc", "abc"), 0.0);
    }

    #[test]
    fn test_damerau_levenshtein() {
        let dl = DamerauLevenshtein::new();
        assert_eq!(dl.distance("CA", "ABC"), 3.0);
        assert_eq!(dl.distance("", ""), 0.0);
    }

    #[test]
    fn test_jaro_winkler() {
        let jw = JaroWinkler::new();
        let sim = jw.similarity("MARTHA", "MARHTA");
        assert!((sim - 0.9611).abs() < 0.01);
    }

    #[test]
    fn test_jaro() {
        let j = Jaro::new();
        let sim = j.similarity("MARTHA", "MARHTA");
        assert!((sim - 0.9444).abs() < 0.01);
    }

    #[test]
    fn test_needleman_wunsch() {
        let nw = NeedlemanWunsch::new();
        let sim = nw.similarity("GATTACA", "GCATGCU");
        assert!(sim > 0.0);
    }

    #[test]
    fn test_smith_waterman() {
        let sw = SmithWaterman::new();
        let sim = sw.similarity("GATTACA", "GCATGCU");
        assert!(sim > 0.0);
    }

    #[test]
    fn test_mlipns() {
        let m = Mlipns::new();
        assert_eq!(m.similarity("abc", "abc"), 1.0);
    }
}
