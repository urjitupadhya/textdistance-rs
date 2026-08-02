use wasm_bindgen::prelude::*;
use crate::algorithms::base::{TextDistance, TextSimilarity};
use crate::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct CompareResult {
    category: &'static str,
    algorithm: &'static str,
    distance: f64,
    similarity: f64,
    normalized_distance: f64,
    normalized_similarity: f64,
}

#[derive(Serialize)]
pub struct CompareAllResponse {
    results: Vec<CompareResult>,
}

macro_rules! run_distance {
    ($cat:expr, $alg_name:expr, $alg:expr, $s1:expr, $s2:expr) => {
        CompareResult {
            category: $cat,
            algorithm: $alg_name,
            distance: $alg.distance($s1, $s2),
            similarity: $alg.similarity($s1, $s2),
            normalized_distance: $alg.normalized_distance($s1, $s2),
            normalized_similarity: $alg.normalized_similarity($s1, $s2),
        }
    };
}

macro_rules! run_similarity {
    ($cat:expr, $alg_name:expr, $alg:expr, $s1:expr, $s2:expr) => {
        CompareResult {
            category: $cat,
            algorithm: $alg_name,
            distance: $alg.distance($s1, $s2),
            similarity: $alg.similarity($s1, $s2),
            normalized_distance: $alg.normalized_distance($s1, $s2),
            normalized_similarity: $alg.normalized_similarity($s1, $s2),
        }
    };
}

#[wasm_bindgen]
pub fn compare_all_wasm(s1: &str, s2: &str) -> JsValue {
    let mut results = Vec::new();

    // Edit-based (Distance)
    results.push(run_distance!("Edit", "Hamming", Hamming::new(), s1, s2));
    results.push(run_distance!("Edit", "Levenshtein", Levenshtein::new(), s1, s2));
    results.push(run_distance!("Edit", "DamerauLevenshtein", DamerauLevenshtein::new(), s1, s2));

    // Edit-based (Similarity)
    results.push(run_similarity!("Edit", "Jaro", Jaro::new(), s1, s2));
    results.push(run_similarity!("Edit", "JaroWinkler", JaroWinkler::new(), s1, s2));
    results.push(run_similarity!("Edit", "StrCmp95", StrCmp95::new(), s1, s2));
    results.push(run_similarity!("Edit", "NeedlemanWunsch", NeedlemanWunsch::new(), s1, s2));
    results.push(run_similarity!("Edit", "SmithWaterman", SmithWaterman::new(), s1, s2));
    results.push(run_similarity!("Edit", "Gotoh", Gotoh::new(), s1, s2));
    results.push(run_similarity!("Edit", "MLIPNS", Mlipns::new(), s1, s2));

    // Token-based (Similarity)
    results.push(run_similarity!("Token", "Jaccard", Jaccard::new(), s1, s2));
    results.push(run_similarity!("Token", "Sorensen", Sorensen::new(), s1, s2));
    results.push(run_similarity!("Token", "Tversky", Tversky::new(), s1, s2));
    results.push(run_similarity!("Token", "Overlap", Overlap::new(), s1, s2));
    results.push(run_similarity!("Token", "Cosine", Cosine::new(), s1, s2));
    results.push(run_similarity!("Token", "MongeElkan", MongeElkan::new(), s1, s2));

    // Token-based (Distance)
    results.push(run_distance!("Token", "Tanimoto", Tanimoto::new(), s1, s2));
    results.push(run_distance!("Token", "Bag", Bag::new(), s1, s2));

    // Sequence-based (Similarity)
    results.push(run_similarity!("Sequence", "LCSSeq", LCSSeq::new(), s1, s2));
    results.push(run_similarity!("Sequence", "LCSStr", LCSStr::new(), s1, s2));
    results.push(run_similarity!("Sequence", "RatcliffObershelp", RatcliffObershelp::new(), s1, s2));

    // Compression-based (Distance)
    results.push(run_distance!("Compression", "ArithNCD", ArithNcd::new(), s1, s2));
    results.push(run_distance!("Compression", "RLENCD", RleNcd::new(), s1, s2));
    results.push(run_distance!("Compression", "BWTRLENCD", BwtRleNcd::new(), s1, s2));
    results.push(run_distance!("Compression", "SqrtNCD", SqrtNcd::new(), s1, s2));
    results.push(run_distance!("Compression", "EntropyNCD", EntropyNcd::new(), s1, s2));
    results.push(run_distance!("Compression", "BZ2NCD", Bz2Ncd::new(), s1, s2));
    results.push(run_distance!("Compression", "LZMANCD", LzmaNcd::new(), s1, s2));
    results.push(run_distance!("Compression", "ZlibNCD", ZlibNcd::new(), s1, s2));

    // Phonetic
    results.push(run_similarity!("Phonetic", "MRA", Mra::new(), s1, s2));
    results.push(run_distance!("Phonetic", "Editex", Editex::new(), s1, s2));

    // Simple
    results.push(run_similarity!("Simple", "Prefix", Prefix::new(), s1, s2));
    results.push(run_similarity!("Simple", "Postfix", Postfix::new(), s1, s2));
    results.push(run_distance!("Simple", "Length", Length::new(), s1, s2));
    results.push(run_similarity!("Simple", "Identity", Identity::new(), s1, s2));
    results.push(run_similarity!("Simple", "Matrix", Matrix::new(), s1, s2));

    let response = CompareAllResponse { results };
    serde_wasm_bindgen::to_value(&response).unwrap()
}
