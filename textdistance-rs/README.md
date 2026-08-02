# textdistance-rs

> **30+ string distance/similarity algorithms, ported from Python to Rust.**
>
> A Port Mortem hackathon submission (Track D: Python → Rust).

## What This Is

A complete Rust port of [life4/textdistance](https://github.com/life4/textdistance) — a Python library providing 30+ string distance and similarity algorithms under a unified interface.

**Original**: Python library with optional C-extension acceleration via `rapidfuzz`, `jellyfish`, `numpy`.
**This port**: Single Rust binary. No Python runtime. No C extensions. No numpy.

## Algorithms Implemented

### Edit-Based (10)
Hamming, Levenshtein, Damerau-Levenshtein, Jaro, Jaro-Winkler, StrCmp95, Needleman-Wunsch, Smith-Waterman, Gotoh, MLIPNS

### Token-Based (8)
Jaccard, Sørensen-Dice, Tversky, Overlap, Cosine, Tanimoto, Monge-Elkan, Bag

### Sequence-Based (3)
Longest Common Subsequence, Longest Common Substring, Ratcliff-Obershelp

### Compression-Based (8)
Arithmetic NCD, RLE NCD, BWT+RLE NCD, Sqrt NCD, Entropy NCD, BZ2 NCD, LZMA NCD, Zlib NCD

### Phonetic (2)
MRA (Match Rating Approach), Editex

### Simple (5)
Prefix, Postfix, Length, Identity, Matrix

## Build

```bash
cargo build --release
```

## Usage (CLI)

```bash
# Distance
./target/release/textdistance -a levenshtein "kitten" "sitting"
# => 3

# Normalized similarity
./target/release/textdistance -a levenshtein -m normalized-similarity "kitten" "sitting"
# => 0.5714...

# Any algorithm, any method
./target/release/textdistance -a jaro-winkler -m similarity "MARTHA" "MARHTA"
# => 0.9611...
```

## Usage (Library)

```rust
use textdistance::{Levenshtein, Hamming, JaroWinkler};
use textdistance::algorithms::base::{TextDistance, TextSimilarity};

let lev = Levenshtein::new();
assert_eq!(lev.distance("kitten", "sitting"), 3.0);

let jw = JaroWinkler::new();
assert!((jw.similarity("MARTHA", "MARHTA") - 0.9611).abs() < 0.01);
```

## Test

```bash
cargo test
```

## Differential Fuzz

```bash
# Build first
cargo build --release
# Run differential fuzzer (requires Python + textdistance installed)
python3 fuzz/harness.py --iterations 1000 --timeout 60
```

## Docker

```bash
docker build -t textdistance-rs .
docker run textdistance-rs textdistance -a levenshtein "hello" "world"
```

## Project Structure

```
textdistance-rs/
├── README.md
├── DECISIONS.md           ← 20 architectural decisions with rationale
├── Dockerfile
├── Cargo.toml
├── .port-mortem.toml
├── src/
│   ├── main.rs            ← CLI (clap)
│   ├── lib.rs             ← Public API
│   └── algorithms/
│       ├── base.rs        ← TextDistance / TextSimilarity traits
│       ├── edit_based.rs  ← 10 algorithms
│       ├── token_based.rs ← 8 algorithms
│       ├── sequence_based.rs ← 3 algorithms
│       ├── compression_based.rs ← 8 algorithms
│       ├── phonetic.rs    ← 2 algorithms
│       └── simple.rs      ← 5 algorithms
├── fuzz/
│   └── harness.py         ← Differential fuzzer
├── bench/
│   └── methodology.md     ← Benchmark methodology
└── tests/
    └── original/          ← Original Python test suite (hashed at kickoff)
```

## License

MIT — same as the original.
