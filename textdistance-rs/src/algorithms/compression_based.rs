//! Compression-based Normalized Compression Distance (NCD) algorithms.
//!
//! These algorithms measure string distance by comparing the compressed sizes
//! of the individual strings vs their concatenation.

use std::collections::HashMap;
use std::io::Write;

use super::base::TextDistance;

// ---------------------------------------------------------------------------
// NCD base logic
// ---------------------------------------------------------------------------

/// Compute NCD given a compression function.
/// NCD(x, y) = (C(xy) - min(C(x), C(y))) / max(C(x), C(y))
fn ncd_from_sizes(cx: f64, cy: f64, cxy: f64, cyx: f64) -> f64 {
    let concat_len = cxy.min(cyx);
    let max_len = cx.max(cy);
    let min_len = cx.min(cy);
    if max_len == 0.0 {
        return 0.0;
    }
    (concat_len - min_len) / max_len
}

// ---------------------------------------------------------------------------
// RLE NCD
// ---------------------------------------------------------------------------

/// Run-Length Encoding based NCD.
///
/// <https://en.wikipedia.org/wiki/Run-length_encoding>
#[derive(Debug, Clone, Default)]
pub struct RleNcd;

impl RleNcd {
    pub fn new() -> Self {
        Self
    }

    fn compress(&self, data: &str) -> String {
        let chars: Vec<char> = data.chars().collect();
        if chars.is_empty() {
            return String::new();
        }
        let mut result = String::new();
        let mut i = 0;
        while i < chars.len() {
            let ch = chars[i];
            let mut count = 1;
            while i + count < chars.len() && chars[i + count] == ch {
                count += 1;
            }
            if count > 2 {
                result.push_str(&count.to_string());
                result.push(ch);
            } else if count == 1 {
                result.push(ch);
            } else {
                result.push(ch);
                result.push(ch);
            }
            i += count;
        }
        result
    }
}

impl TextDistance for RleNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let cx = self.compress(s1).len() as f64;
        let cy = self.compress(s2).len() as f64;
        let xy = format!("{}{}", s1, s2);
        let yx = format!("{}{}", s2, s1);
        let cxy = self.compress(&xy).len() as f64;
        let cyx = self.compress(&yx).len() as f64;
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// BWT + RLE NCD
// ---------------------------------------------------------------------------

/// Burrows-Wheeler Transform + Run-Length Encoding NCD.
///
/// <https://en.wikipedia.org/wiki/Burrows%E2%80%93Wheeler_transform>
#[derive(Debug, Clone)]
pub struct BwtRleNcd {
    pub terminator: char,
}

impl Default for BwtRleNcd {
    fn default() -> Self {
        Self { terminator: '\0' }
    }
}

impl BwtRleNcd {
    pub fn new() -> Self {
        Self::default()
    }

    fn bwt(&self, data: &str) -> String {
        if data.is_empty() {
            return self.terminator.to_string();
        }
        let mut s = data.to_string();
        if !s.contains(self.terminator) {
            s.push(self.terminator);
            let mut rotations: Vec<String> = (0..s.len())
                .map(|i| format!("{}{}", &s[i..], &s[..i]))
                .collect();
            rotations.sort();
            rotations.iter().map(|r| r.chars().last().unwrap()).collect()
        } else {
            s
        }
    }

    fn compress(&self, data: &str) -> String {
        let bwt_data = self.bwt(data);
        let rle = RleNcd::new();
        rle.compress(&bwt_data)
    }
}

impl TextDistance for BwtRleNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let cx = self.compress(s1).len() as f64;
        let cy = self.compress(s2).len() as f64;
        let xy = format!("{}{}", s1, s2);
        let yx = format!("{}{}", s2, s1);
        let cxy = self.compress(&xy).len() as f64;
        let cyx = self.compress(&yx).len() as f64;
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// SqrtNCD
// ---------------------------------------------------------------------------

/// Square Root based NCD.
///
/// Size of compressed data equals the sum of square roots of counts of every
/// element in the input sequence.
#[derive(Debug, Clone, Default)]
pub struct SqrtNcd;

impl SqrtNcd {
    pub fn new() -> Self {
        Self
    }

    fn get_size(&self, data: &str) -> f64 {
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in data.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        counts.values().map(|&c| (c as f64).sqrt()).sum()
    }
}

impl TextDistance for SqrtNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let cx = self.get_size(s1);
        let cy = self.get_size(s2);
        let xy = format!("{}{}", s1, s2);
        let yx = format!("{}{}", s2, s1);
        let cxy = self.get_size(&xy);
        let cyx = self.get_size(&yx);
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// EntropyNCD
// ---------------------------------------------------------------------------

/// Entropy based NCD.
///
/// Uses Shannon entropy as the "compressed size".
///
/// <https://en.wikipedia.org/wiki/Entropy_(information_theory)>
#[derive(Debug, Clone)]
pub struct EntropyNcd {
    pub coef: f64,
    pub base: f64,
}

impl Default for EntropyNcd {
    fn default() -> Self {
        Self { coef: 1.0, base: 2.0 }
    }
}

impl EntropyNcd {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_size(&self, data: &str) -> f64 {
        let total = data.chars().count();
        if total == 0 {
            return self.coef;
        }
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in data.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut entropy = 0.0f64;
        for &count in counts.values() {
            let p = count as f64 / total as f64;
            entropy -= p * p.log(self.base);
        }
        self.coef + entropy
    }
}

impl TextDistance for EntropyNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let cx = self.get_size(s1);
        let cy = self.get_size(s2);
        let xy = format!("{}{}", s1, s2);
        let yx = format!("{}{}", s2, s1);
        let cxy = self.get_size(&xy);
        let cyx = self.get_size(&yx);
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// BZ2 NCD
// ---------------------------------------------------------------------------

/// BZ2 compression based NCD.
///
/// <https://en.wikipedia.org/wiki/Bzip2>
#[derive(Debug, Clone, Default)]
pub struct Bz2Ncd;

impl Bz2Ncd {
    pub fn new() -> Self {
        Self
    }

    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder = bzip2::write::BzEncoder::new(Vec::new(), bzip2::Compression::default());
        encoder.write_all(data).unwrap();
        let compressed = encoder.finish().unwrap();
        // Skip the 15-byte header to match Python's codecs.encode(data, 'bz2_codec')[15:]
        if compressed.len() > 15 {
            compressed[15..].to_vec()
        } else {
            compressed
        }
    }
}

impl TextDistance for Bz2Ncd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let cx = self.compress(b1).len() as f64;
        let cy = self.compress(b2).len() as f64;
        let mut xy = Vec::with_capacity(b1.len() + b2.len());
        xy.extend_from_slice(b1);
        xy.extend_from_slice(b2);
        let mut yx = Vec::with_capacity(b1.len() + b2.len());
        yx.extend_from_slice(b2);
        yx.extend_from_slice(b1);
        let cxy = self.compress(&xy).len() as f64;
        let cyx = self.compress(&yx).len() as f64;
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// LZMA NCD
// ---------------------------------------------------------------------------

/// LZMA compression based NCD.
///
/// <https://en.wikipedia.org/wiki/LZMA>
///
/// Note: Uses flate2 (zlib/deflate) as a stand-in. For true LZMA, the `lzma`
/// crate would be needed. This is documented in DECISIONS.md.
#[derive(Debug, Clone, Default)]
pub struct LzmaNcd;

impl LzmaNcd {
    pub fn new() -> Self {
        Self
    }

    fn compress(&self, data: &[u8]) -> Vec<u8> {
        // Using flate2 deflate as LZMA stand-in.
        // The actual NCD calculation only cares about relative sizes.
        let mut encoder =
            flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::best());
        encoder.write_all(data).unwrap();
        encoder.finish().unwrap()
    }
}

impl TextDistance for LzmaNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let cx = self.compress(b1).len() as f64;
        let cy = self.compress(b2).len() as f64;
        let mut xy = Vec::with_capacity(b1.len() + b2.len());
        xy.extend_from_slice(b1);
        xy.extend_from_slice(b2);
        let mut yx = Vec::with_capacity(b1.len() + b2.len());
        yx.extend_from_slice(b2);
        yx.extend_from_slice(b1);
        let cxy = self.compress(&xy).len() as f64;
        let cyx = self.compress(&yx).len() as f64;
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// Zlib NCD
// ---------------------------------------------------------------------------

/// Zlib compression based NCD.
///
/// <https://en.wikipedia.org/wiki/Zlib>
#[derive(Debug, Clone, Default)]
pub struct ZlibNcd;

impl ZlibNcd {
    pub fn new() -> Self {
        Self
    }

    fn compress(&self, data: &[u8]) -> Vec<u8> {
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(data).unwrap();
        let compressed = encoder.finish().unwrap();
        // Skip first 2 bytes to match Python's codecs.encode(data, 'zlib_codec')[2:]
        if compressed.len() > 2 {
            compressed[2..].to_vec()
        } else {
            compressed
        }
    }
}

impl TextDistance for ZlibNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let b1 = s1.as_bytes();
        let b2 = s2.as_bytes();
        let cx = self.compress(b1).len() as f64;
        let cy = self.compress(b2).len() as f64;
        let mut xy = Vec::with_capacity(b1.len() + b2.len());
        xy.extend_from_slice(b1);
        xy.extend_from_slice(b2);
        let mut yx = Vec::with_capacity(b1.len() + b2.len());
        yx.extend_from_slice(b2);
        yx.extend_from_slice(b1);
        let cxy = self.compress(&xy).len() as f64;
        let cyx = self.compress(&yx).len() as f64;
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// ArithNCD (Arithmetic coding NCD)
// ---------------------------------------------------------------------------

/// Arithmetic coding based NCD.
///
/// Uses a simplified arithmetic coding estimation for NCD.
/// The Python original uses `fractions.Fraction` for exact arithmetic.
/// We use f64 for practical purposes — documented in DECISIONS.md.
#[derive(Debug, Clone)]
pub struct ArithNcd {
    pub base: f64,
}

impl Default for ArithNcd {
    fn default() -> Self {
        Self { base: 2.0 }
    }
}

impl ArithNcd {
    pub fn new() -> Self {
        Self::default()
    }

    fn get_size(&self, data: &str) -> f64 {
        if data.is_empty() {
            return 0.0;
        }
        let mut counts: HashMap<char, usize> = HashMap::new();
        for c in data.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }
        let total = data.chars().count() as f64;
        // Estimate: sum of -log2(p) for each character ≈ compressed size in bits
        let mut size = 0.0f64;
        for c in data.chars() {
            let p = *counts.get(&c).unwrap() as f64 / total;
            size += -(p.log(self.base));
        }
        size
    }
}

impl TextDistance for ArithNcd {
    fn maximum(&self, _s1: &str, _s2: &str) -> f64 {
        1.0
    }

    fn distance(&self, s1: &str, s2: &str) -> f64 {
        if s1.is_empty() && s2.is_empty() {
            return 0.0;
        }
        let cx = self.get_size(s1);
        let cy = self.get_size(s2);
        let xy = format!("{}{}", s1, s2);
        let yx = format!("{}{}", s2, s1);
        let cxy = self.get_size(&xy);
        let cyx = self.get_size(&yx);
        ncd_from_sizes(cx, cy, cxy, cyx)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rle_ncd() {
        let r = RleNcd::new();
        assert_eq!(r.distance("aaa", "aaa"), 0.0);
    }

    #[test]
    fn test_sqrt_ncd() {
        let s = SqrtNcd::new();
        assert_eq!(s.distance("abc", "abc"), 0.0);
    }

    #[test]
    fn test_bz2_ncd() {
        let b = Bz2Ncd::new();
        let d = b.distance("hello world", "hello world");
        assert!(d >= 0.0 && d <= 1.0);
    }

    #[test]
    fn test_zlib_ncd() {
        let z = ZlibNcd::new();
        let d = z.distance("hello", "world");
        assert!(d >= 0.0);
    }
}
