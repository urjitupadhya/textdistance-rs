use clap::{Parser, ValueEnum};
use textdistance::algorithms::base::{TextDistance, TextSimilarity};

#[derive(Debug, Clone, ValueEnum)]
enum Algorithm {
    // Edit-based
    Hamming,
    Levenshtein,
    DamerauLevenshtein,
    Jaro,
    JaroWinkler,
    Strcmp95,
    NeedlemanWunsch,
    SmithWaterman,
    Gotoh,
    Mlipns,
    // Token-based
    Jaccard,
    Sorensen,
    Tversky,
    Overlap,
    Cosine,
    Tanimoto,
    MongeElkan,
    Bag,
    // Sequence-based
    Lcsseq,
    Lcsstr,
    RatcliffObershelp,
    // Compression-based
    ArithNcd,
    RleNcd,
    BwtRleNcd,
    SqrtNcd,
    EntropyNcd,
    Bz2Ncd,
    LzmaNcd,
    ZlibNcd,
    // Phonetic
    Mra,
    Editex,
    // Simple
    Prefix,
    Postfix,
    Length,
    Identity,
    Matrix,
}

#[derive(Debug, Clone, ValueEnum)]
enum Method {
    Distance,
    Similarity,
    NormalizedDistance,
    NormalizedSimilarity,
}

/// textdistance — 30+ string distance/similarity algorithms in Rust.
///
/// A port of the Python `textdistance` library.
#[derive(Parser, Debug)]
#[command(name = "textdistance", version, about)]
struct Cli {
    /// Start the SaaS API server
    #[arg(long)]
    serve: bool,

    /// Port to serve on
    #[arg(long, default_value_t = 3000)]
    port: u16,

    /// The algorithm to use.
    #[arg(short, long)]
    algorithm: Option<Algorithm>,

    /// The method to call.
    #[arg(short, long, default_value = "distance")]
    method: Method,

    /// First string.
    s1: Option<String>,

    /// Second string.
    s2: Option<String>,
}

/// Helper macro: call the right method on either TextDistance or TextSimilarity
macro_rules! call_distance {
    ($alg:expr, $method:expr, $s1:expr, $s2:expr) => {
        match $method {
            Method::Distance => $alg.distance($s1, $s2),
            Method::Similarity => $alg.similarity($s1, $s2),
            Method::NormalizedDistance => $alg.normalized_distance($s1, $s2),
            Method::NormalizedSimilarity => $alg.normalized_similarity($s1, $s2),
        }
    };
}

macro_rules! call_similarity {
    ($alg:expr, $method:expr, $s1:expr, $s2:expr) => {
        match $method {
            Method::Distance => $alg.distance($s1, $s2),
            Method::Similarity => $alg.similarity($s1, $s2),
            Method::NormalizedDistance => $alg.normalized_distance($s1, $s2),
            Method::NormalizedSimilarity => $alg.normalized_similarity($s1, $s2),
        }
    };
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    if cli.serve {
        textdistance::server::start_server(cli.port).await;
        return;
    }

    let algorithm = cli.algorithm.expect("Algorithm is required in CLI mode");
    let s1 = cli.s1.expect("s1 is required in CLI mode");
    let s2 = cli.s2.expect("s2 is required in CLI mode");
    let s1 = &s1;
    let s2 = &s2;

    let result = match algorithm {
        // Edit-based (TextDistance trait)
        Algorithm::Hamming => call_distance!(textdistance::Hamming::new(), cli.method, s1, s2),
        Algorithm::Levenshtein => call_distance!(textdistance::Levenshtein::new(), cli.method, s1, s2),
        Algorithm::DamerauLevenshtein => call_distance!(textdistance::DamerauLevenshtein::new(), cli.method, s1, s2),

        // Edit-based (TextSimilarity trait)
        Algorithm::Jaro => call_similarity!(textdistance::Jaro::new(), cli.method, s1, s2),
        Algorithm::JaroWinkler => call_similarity!(textdistance::JaroWinkler::new(), cli.method, s1, s2),
        Algorithm::Strcmp95 => call_similarity!(textdistance::StrCmp95::new(), cli.method, s1, s2),
        Algorithm::NeedlemanWunsch => call_similarity!(textdistance::NeedlemanWunsch::new(), cli.method, s1, s2),
        Algorithm::SmithWaterman => call_similarity!(textdistance::SmithWaterman::new(), cli.method, s1, s2),
        Algorithm::Gotoh => call_similarity!(textdistance::Gotoh::new(), cli.method, s1, s2),
        Algorithm::Mlipns => call_similarity!(textdistance::Mlipns::new(), cli.method, s1, s2),

        // Token-based (TextSimilarity trait)
        Algorithm::Jaccard => call_similarity!(textdistance::Jaccard::new(), cli.method, s1, s2),
        Algorithm::Sorensen => call_similarity!(textdistance::Sorensen::new(), cli.method, s1, s2),
        Algorithm::Tversky => call_similarity!(textdistance::Tversky::new(), cli.method, s1, s2),
        Algorithm::Overlap => call_similarity!(textdistance::Overlap::new(), cli.method, s1, s2),
        Algorithm::Cosine => call_similarity!(textdistance::Cosine::new(), cli.method, s1, s2),
        Algorithm::MongeElkan => call_similarity!(textdistance::MongeElkan::new(), cli.method, s1, s2),
        Algorithm::Mra => call_similarity!(textdistance::Mra::new(), cli.method, s1, s2),

        // Token-based (TextDistance trait)
        Algorithm::Tanimoto => call_distance!(textdistance::Tanimoto::new(), cli.method, s1, s2),
        Algorithm::Bag => call_distance!(textdistance::Bag::new(), cli.method, s1, s2),

        // Sequence-based (TextSimilarity trait)
        Algorithm::Lcsseq => call_similarity!(textdistance::LCSSeq::new(), cli.method, s1, s2),
        Algorithm::Lcsstr => call_similarity!(textdistance::LCSStr::new(), cli.method, s1, s2),
        Algorithm::RatcliffObershelp => call_similarity!(textdistance::RatcliffObershelp::new(), cli.method, s1, s2),

        // Compression-based (TextDistance trait)
        Algorithm::ArithNcd => call_distance!(textdistance::ArithNcd::new(), cli.method, s1, s2),
        Algorithm::RleNcd => call_distance!(textdistance::RleNcd::new(), cli.method, s1, s2),
        Algorithm::BwtRleNcd => call_distance!(textdistance::BwtRleNcd::new(), cli.method, s1, s2),
        Algorithm::SqrtNcd => call_distance!(textdistance::SqrtNcd::new(), cli.method, s1, s2),
        Algorithm::EntropyNcd => call_distance!(textdistance::EntropyNcd::new(), cli.method, s1, s2),
        Algorithm::Bz2Ncd => call_distance!(textdistance::Bz2Ncd::new(), cli.method, s1, s2),
        Algorithm::LzmaNcd => call_distance!(textdistance::LzmaNcd::new(), cli.method, s1, s2),
        Algorithm::ZlibNcd => call_distance!(textdistance::ZlibNcd::new(), cli.method, s1, s2),

        // Phonetic (TextDistance trait for Editex)
        Algorithm::Editex => call_distance!(textdistance::Editex::new(), cli.method, s1, s2),

        // Simple
        Algorithm::Prefix => call_similarity!(textdistance::Prefix::new(), cli.method, s1, s2),
        Algorithm::Postfix => call_similarity!(textdistance::Postfix::new(), cli.method, s1, s2),
        Algorithm::Length => call_distance!(textdistance::Length::new(), cli.method, s1, s2),
        Algorithm::Identity => call_similarity!(textdistance::Identity::new(), cli.method, s1, s2),
        Algorithm::Matrix => call_similarity!(textdistance::Matrix::new(), cli.method, s1, s2),
    };

    println!("{}", result);
}
