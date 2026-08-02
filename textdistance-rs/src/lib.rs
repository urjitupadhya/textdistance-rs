/// textdistance — 30+ string distance/similarity algorithms in Rust.
///
/// A port of the Python `textdistance` library by life4.
/// Every algorithm implements the `TextDistance` trait providing
/// `distance`, `similarity`, `normalized_distance`, and `normalized_similarity`.

//! # TextDistance
//! 
//! `textdistance` is a high-performance, 100% safe Rust port of the popular Python `textdistance` library.
//! It provides 30+ string distance and similarity algorithms spanning Edit, Token, Sequence, Compression,
//! Phonetic, and Simple categories.
//! 
//! ## Features
//! - **Zero Dependencies:** Pure Rust implementations without C-extensions or external math libraries.
//! - **100% Behavioral Parity:** Algorithms are fuzz-tested against their Python counterparts to guarantee mathematical accuracy.
//! - **WebAssembly Support:** Compiles to WASM for seamless integration into browser-based applications.
//! 
//! ## Quick Start
//! ```rust
//! use textdistance::{Levenshtein, TextDistance, TextSimilarity};
//! 
//! let lev = Levenshtein::new();
//! assert_eq!(lev.distance("kitten", "sitting"), 3.0);
//! assert_eq!(lev.normalized_similarity("kitten", "sitting"), 0.5714285714285714);
//! ```

#![warn(missing_docs)]

pub mod algorithms;
pub mod server;
pub mod wasm;

// Re-export all algorithm structs at the crate root for convenience.
pub use algorithms::base::{TextDistance, TextSimilarity};
pub use algorithms::edit_based::{
    DamerauLevenshtein, Gotoh, Hamming, Jaro, JaroWinkler, Levenshtein, Mlipns,
    NeedlemanWunsch, SmithWaterman, StrCmp95,
};
pub use algorithms::token_based::{Bag, Cosine, Jaccard, MongeElkan, Overlap, Sorensen, Tanimoto, Tversky};
pub use algorithms::sequence_based::{LCSSeq, LCSStr, RatcliffObershelp};
pub use algorithms::compression_based::{
    ArithNcd, BwtRleNcd, Bz2Ncd, EntropyNcd, LzmaNcd, RleNcd, SqrtNcd, ZlibNcd,
};
pub use algorithms::phonetic::{Editex, Mra};
pub use algorithms::simple::{Identity, Length, Matrix, Postfix, Prefix};
