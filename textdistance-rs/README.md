# textdistance-rs

> *"How similar are two strings? It depends on how you ask."*

---

## The Story

It started with a deceptively simple question: **how do you measure the distance between two words?**

"kitten" and "sitting" — are they close? By one metric, it takes 3 edits to transform one into the other. By another, they share 57% similarity. By a phonetic measure, they sound nothing alike. By a compression-based metric, they compress very differently. **The answer changes completely depending on which lens you use.**

The Python library [`textdistance`](https://github.com/life4/textdistance) solved this beautifully — 36 algorithms, one unified interface. But Python has a price: it leans on C extensions (`rapidfuzz`, `jellyfish`, `numpy`) to get anywhere near fast. Remove those, and it crawls. Add them, and you've imported half the scientific computing stack just to check if two names match.

So we asked the natural follow-up: **what if you threw Python away and rewrote the whole thing in Rust?**

No Python runtime. No C extensions. No NumPy. No dependencies you didn't choose yourself.

Just pure, safe Rust — running 36 algorithms in microseconds, compiling to a single binary, and deploying as a SaaS API with a live dashboard. **That is this project.**

---

## What We Built

### 1. The Core Library
A pure-Rust port of `textdistance`. Every algorithm implements the same two traits:

```rust
pub trait TextDistance {
    fn distance(&self, s1: &str, s2: &str) -> f64;
    fn similarity(&self, s1: &str, s2: &str) -> f64;
    fn normalized_distance(&self, s1: &str, s2: &str) -> f64;
    fn normalized_similarity(&self, s1: &str, s2: &str) -> f64;
}
```

You swap algorithms without changing your code. Levenshtein today, Jaro-Winkler tomorrow, Compression-based NCD next week — same four methods, always.

### 2. The SaaS API
An [Axum](https://github.com/tokio-rs/axum) web server that exposes all 36 algorithms over a single POST endpoint. Rust computes the math in under a millisecond and returns clean JSON. No Python. No Node. Just `cargo run`.

### 3. The Live Dashboard
A React + Vite dashboard that sends your two strings to the API and renders every algorithm's score in real-time — color-coded, categorized, and beautiful.

---

## The 36 Algorithms

### Edit-Based (10)
How many character-level operations does it take to transform one string into another?

| Algorithm | What it measures |
|---|---|
| **Hamming** | Substitutions at each position (same-length strings) |
| **Levenshtein** | Insertions, deletions, substitutions |
| **Damerau-Levenshtein** | Same as Levenshtein, plus transpositions |
| **Jaro** | Transpositions with character matching window |
| **Jaro-Winkler** | Jaro, with a bonus for matching prefixes |
| **StrCmp95** | US Census Bureau's string comparison standard |
| **Needleman-Wunsch** | Global sequence alignment (bioinformatics) |
| **Smith-Waterman** | Local sequence alignment (bioinformatics) |
| **Gotoh** | Affine gap penalties for alignment |
| **MLIPNS** | Mismatch-tolerant similarity |

### Token-Based (8)
What happens if you compare the *sets* of characters or n-grams instead of positions?

| Algorithm | What it measures |
|---|---|
| **Jaccard** | Intersection over union of token sets |
| **Sørensen-Dice** | Weighted Jaccard for biostatistics |
| **Tversky** | Asymmetric Jaccard (α, β weights) |
| **Overlap** | Intersection over the smaller set |
| **Cosine** | Vector angle between token frequency vectors |
| **Tanimoto** | Bitwise Jaccard |
| **Monge-Elkan** | Best-match token similarity |
| **Bag** | Multiset difference |

### Sequence-Based (3)
What's the longest common thread between the two strings?

| Algorithm | What it measures |
|---|---|
| **LCS Subsequence** | Longest common subsequence |
| **LCS Substring** | Longest common contiguous substring |
| **Ratcliff-Obershelp** | Recursive matching block similarity |

### Compression-Based (8)
If two strings are similar, their concatenation compresses well. Compression-based NCD captures this.

| Algorithm | Compressor used |
|---|---|
| **ArithNCD** | Arithmetic coding |
| **RLENCD** | Run-length encoding |
| **BWTRLENCD** | Burrows-Wheeler + RLE |
| **SqrtNCD** | Square-root based estimation |
| **EntropyNCD** | Shannon entropy |
| **BZ2NCD** | bzip2 |
| **LZMANCD** | Deflate (LZMA stand-in) |
| **ZlibNCD** | zlib |

### Phonetic (2)
Sound the same? These algorithms test phonetic proximity.

| Algorithm | What it measures |
|---|---|
| **MRA** | Match Rating Approach (name matching) |
| **Editex** | Edit distance weighted by phonetic similarity |

### Simple (5)
Sometimes the answer is obvious.

| Algorithm | What it measures |
|---|---|
| **Prefix** | Shared starting characters |
| **Postfix** | Shared ending characters |
| **Length** | Difference in string length |
| **Identity** | Exact match (1.0 or 0.0) |
| **Matrix** | Substitution matrix similarity |

---

## Run It Locally

### Backend (Rust API)
```bash
# Terminal 1 — build and start the API server
cd textdistance-rs
cargo run --release -- --serve
# Listening on http://0.0.0.0:3000
```

### Frontend (React Dashboard)
```bash
# Terminal 2 — start the live dashboard
cd textdistance-rs/frontend
npm install
npm run dev
# Open http://localhost:5173
```

### CLI
```bash
# Classic Levenshtein
./target/release/textdistance -a levenshtein "kitten" "sitting"
# => 3

# Normalized similarity
./target/release/textdistance -a levenshtein -m normalized-similarity "kitten" "sitting"
# => 0.5714...

# Jaro-Winkler
./target/release/textdistance -a jaro-winkler -m similarity "MARTHA" "MARHTA"
# => 0.9611...
```

### Library
```rust
use textdistance::{Levenshtein, JaroWinkler};
use textdistance::algorithms::base::{TextDistance, TextSimilarity};

let lev = Levenshtein::new();
assert_eq!(lev.distance("kitten", "sitting"), 3.0);
assert_eq!(lev.normalized_similarity("kitten", "sitting"), 0.5714285714285714);

let jw = JaroWinkler::new();
assert!((jw.similarity("MARTHA", "MARHTA") - 0.9611).abs() < 0.01);
```

---

## Architecture

```
textdistance-rs/
├── src/
│   ├── main.rs                 ← CLI (clap)
│   ├── lib.rs                  ← Public API + re-exports
│   ├── server.rs               ← Axum SaaS API server
│   ├── wasm.rs                 ← WebAssembly bindings
│   └── algorithms/
│       ├── base.rs             ← TextDistance / TextSimilarity traits
│       ├── edit_based.rs       ← 10 algorithms
│       ├── token_based.rs      ← 8 algorithms
│       ├── sequence_based.rs   ← 3 algorithms
│       ├── compression_based.rs ← 8 algorithms
│       ├── phonetic.rs         ← 2 algorithms
│       └── simple.rs           ← 5 algorithms
├── frontend/
│   └── src/App.tsx             ← React live dashboard
├── fuzz/
│   └── harness.py              ← Differential fuzzer vs Python
├── DECISIONS.md                ← 20 documented architectural decisions
└── Dockerfile
```

---

## Why Rust?

Because Python's `textdistance` library is brilliant — but it needs C extensions to be fast. Strip those away and it falls back to pure-Python implementations that are anywhere from **10× to 100× slower**.

Rust gives you C-level performance by default, with a type system that prevents entire categories of bugs at compile time. No GIL. No memory leaks. No runtime surprises. This port runs all 36 algorithms in under a millisecond on typical strings — no warm-up, no JIT, no tricks.

It's not a toy. It's a library you could actually ship.

---

## Hackathon Context

Built for the **Port Mortem Hackathon** — Track D: Python → Rust.

The spirit of the track: take something that exists in Python and understand it deeply enough to rewrite it from scratch in Rust. Not just translating syntax — understanding *why* each algorithm works, what its edge cases are, and how to express those constraints in Rust's type system.

That's the story. 36 algorithms. One binary. Zero Python.

---

## License

MIT — same as the original `textdistance` library by life4.
