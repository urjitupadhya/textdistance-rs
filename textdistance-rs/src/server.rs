use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use crate::algorithms::base::{TextDistance, TextSimilarity};
use crate::*;

#[derive(Deserialize)]
pub struct CompareRequest {
    s1: String,
    s2: String,
}

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

fn sanitize(v: f64) -> f64 {
    if v.is_nan() || v.is_infinite() {
        0.0
    } else {
        v
    }
}

macro_rules! run_distance {
    ($cat:expr, $alg_name:expr, $alg:expr, $s1:expr, $s2:expr) => {
        CompareResult {
            category: $cat,
            algorithm: $alg_name,
            distance: sanitize($alg.distance($s1, $s2)),
            similarity: sanitize($alg.similarity($s1, $s2)),
            normalized_distance: sanitize($alg.normalized_distance($s1, $s2)),
            normalized_similarity: sanitize($alg.normalized_similarity($s1, $s2)),
        }
    };
}

macro_rules! run_similarity {
    ($cat:expr, $alg_name:expr, $alg:expr, $s1:expr, $s2:expr) => {
        CompareResult {
            category: $cat,
            algorithm: $alg_name,
            distance: sanitize($alg.distance($s1, $s2)),
            similarity: sanitize($alg.similarity($s1, $s2)),
            normalized_distance: sanitize($alg.normalized_distance($s1, $s2)),
            normalized_similarity: sanitize($alg.normalized_similarity($s1, $s2)),
        }
    };
}

// removed rayon

async fn compare_all(Json(payload): Json<CompareRequest>) -> impl IntoResponse {
    let s1 = payload.s1.clone();
    let s2 = payload.s2.clone();

    // Boxed closures so we can put them in a Vec and iterate in parallel.
    let tasks: Vec<Box<dyn Fn(&str, &str) -> CompareResult + Send + Sync>> = vec![
        // Edit-based (Distance)
        Box::new(|a, b| run_distance!("Edit", "Hamming", Hamming::new(), a, b)),
        Box::new(|a, b| run_distance!("Edit", "Levenshtein", Levenshtein::new(), a, b)),
        Box::new(|a, b| run_distance!("Edit", "DamerauLevenshtein", DamerauLevenshtein::new(), a, b)),
        // Edit-based (Similarity)
        Box::new(|a, b| run_similarity!("Edit", "Jaro", Jaro::new(), a, b)),
        Box::new(|a, b| run_similarity!("Edit", "JaroWinkler", JaroWinkler::new(), a, b)),
        Box::new(|a, b| run_similarity!("Edit", "StrCmp95", StrCmp95::new(), a, b)),
        Box::new(|a, b| run_similarity!("Edit", "NeedlemanWunsch", NeedlemanWunsch::new(), a, b)),
        Box::new(|a, b| run_similarity!("Edit", "SmithWaterman", SmithWaterman::new(), a, b)),
        Box::new(|a, b| run_similarity!("Edit", "Gotoh", Gotoh::new(), a, b)),
        Box::new(|a, b| run_similarity!("Edit", "MLIPNS", Mlipns::new(), a, b)),
        // Token-based (Similarity)
        Box::new(|a, b| run_similarity!("Token", "Jaccard", Jaccard::new(), a, b)),
        Box::new(|a, b| run_similarity!("Token", "Sorensen", Sorensen::new(), a, b)),
        Box::new(|a, b| run_similarity!("Token", "Tversky", Tversky::new(), a, b)),
        Box::new(|a, b| run_similarity!("Token", "Overlap", Overlap::new(), a, b)),
        Box::new(|a, b| run_similarity!("Token", "Cosine", Cosine::new(), a, b)),
        Box::new(|a, b| run_similarity!("Token", "MongeElkan", MongeElkan::new(), a, b)),
        // Token-based (Distance)
        Box::new(|a, b| run_distance!("Token", "Tanimoto", Tanimoto::new(), a, b)),
        Box::new(|a, b| run_distance!("Token", "Bag", Bag::new(), a, b)),
        // Sequence-based (Similarity)
        Box::new(|a, b| run_similarity!("Sequence", "LCSSeq", LCSSeq::new(), a, b)),
        Box::new(|a, b| run_similarity!("Sequence", "LCSStr", LCSStr::new(), a, b)),
        Box::new(|a, b| run_similarity!("Sequence", "RatcliffObershelp", RatcliffObershelp::new(), a, b)),
        // Compression-based (Distance)
        Box::new(|a, b| run_distance!("Compression", "ArithNCD", ArithNcd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "RLENCD", RleNcd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "BWTRLENCD", BwtRleNcd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "SqrtNCD", SqrtNcd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "EntropyNCD", EntropyNcd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "BZ2NCD", Bz2Ncd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "LZMANCD", LzmaNcd::new(), a, b)),
        Box::new(|a, b| run_distance!("Compression", "ZlibNCD", ZlibNcd::new(), a, b)),
        // Phonetic
        Box::new(|a, b| run_similarity!("Phonetic", "MRA", Mra::new(), a, b)),
        Box::new(|a, b| run_distance!("Phonetic", "Editex", Editex::new(), a, b)),
        // Simple
        Box::new(|a, b| run_similarity!("Simple", "Prefix", Prefix::new(), a, b)),
        Box::new(|a, b| run_similarity!("Simple", "Postfix", Postfix::new(), a, b)),
        Box::new(|a, b| run_distance!("Simple", "Length", Length::new(), a, b)),
        Box::new(|a, b| run_similarity!("Simple", "Identity", Identity::new(), a, b)),
        Box::new(|a, b| run_similarity!("Simple", "Matrix", Matrix::new(), a, b)),
    ];

    // Compute all 36 algorithms sequentially (these take < 1ms total)
    let results: Vec<CompareResult> = tasks.iter().map(|f| f(&s1, &s2)).collect();

    Json(CompareAllResponse { results })
}

pub async fn start_server(port: u16) {
    let api_routes = Router::new()
        .route("/compare_all", post(compare_all));

    // Serve the frontend static files if they exist
    let app = Router::new()
        .nest("/api", api_routes)
        .fallback_service(ServeDir::new("frontend/dist"))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("Starting SaaS API server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
