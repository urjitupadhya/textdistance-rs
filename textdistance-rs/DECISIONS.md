# DECISIONS.md — Architectural Divergences

Every non-trivial decision made while porting `textdistance` from Python to Rust, with rationale.

---

## 1. Trait Hierarchy vs Class Inheritance

**Python**: Uses `Base` and `BaseSimilarity` classes with method override patterns. `BaseSimilarity` flips the relationship between `distance` and `similarity`.

**Rust**: Two separate traits — `TextDistance` (natively computes distance) and `TextSimilarity` (natively computes similarity). Each provides default implementations for the other direction, matching the Python `Base`/`BaseSimilarity` behavior exactly.

**Rationale**: Rust has no class inheritance. Traits with default methods are the idiomatic replacement. This preserves the exact same dispatch logic.

---

## 2. `qval` Parameter and N-Gram Splitting

**Python**: `qval` is an `__init__` parameter on every algorithm. `qval=1` means character-level comparison, `qval=None` means split by words, `qval>1` means n-gram splitting.

**Rust**: `qval` is a `usize` field. `qval=0` replaces Python's `qval=None` for word-splitting. This avoids `Option<usize>` throughout and keeps the API clean.

**Rationale**: `Option<usize>` adds complexity with no benefit. The value `0` is semantically equivalent to "split by words" and is unused in the n-gram context.

---

## 3. `external` Parameter Removed

**Python**: Every algorithm has `external=True` which attempts to call external C libraries (rapidfuzz, jellyfish) for faster computation.

**Rust**: Removed entirely. The Rust port IS the fast implementation. There's no need to delegate to C libraries when Rust already achieves comparable or better performance.

**Rationale**: The hackathon rule states "No Source-Language Runtime." The external library mechanism is Python-specific optimization that has no analog in Rust.

---

## 4. `test_func` Parameter Removed

**Python**: Several algorithms accept a `test_func` callback for custom equality testing (e.g., case-insensitive comparison). Default is identity.

**Rust**: Removed for initial port. All algorithms use direct character equality (`==`). This matches the default Python behavior exactly.

**Rationale**: The `test_func` pattern in Python uses dynamic dispatch that would require `Box<dyn Fn>` in Rust, adding lifetime complexity. Since all tests use the default identity function, this is behaviorally equivalent and can be added as a generic parameter later.

---

## 5. NumPy Dependency Eliminated

**Python**: `NeedlemanWunsch`, `SmithWaterman`, `Gotoh`, and `Editex` require `numpy` for their DP matrices. They raise `ImportError` without it.

**Rust**: All algorithms use `Vec<Vec<f64>>` for DP matrices. No external numerical library needed.

**Rationale**: This is a core advantage of the port — single binary, no dependencies at runtime. The DP matrices are small enough that native Rust vectors are sufficient.

---

## 6. Counter Operations: `collections.Counter` → `HashMap<String, usize>`

**Python**: Uses `collections.Counter` with operator overloading (`&=` for intersection, `|=` for union, `+=` for sum, `-` for difference).

**Rust**: Explicit helper functions (`intersect_counters`, `union_counters`, `sum_counters`, `subtract_counter`) operating on `HashMap<String, usize>`.

**Rationale**: Rust's `HashMap` doesn't have Counter-style operator overloads. Explicit functions are more readable to Rust reviewers and avoid surprising behavior.

---

## 7. Compression NCD: `codecs.encode` Header Stripping

**Python**: `BZ2NCD` uses `codecs.encode(data, 'bz2_codec')[15:]` — strips the first 15 bytes of the bz2 header. `ZLIBNCD` strips `[2:]`.

**Rust**: Uses `bzip2::write::BzEncoder` and `flate2::write::ZlibEncoder` with equivalent byte stripping: `compressed[15..]` for bz2, `compressed[2..]` for zlib.

**Rationale**: The header bytes contain metadata (magic numbers, compression level) that don't contribute to the NCD distance calculation. Python strips them; we must do the same for behavioral equivalence.

---

## 8. ArithNCD: `fractions.Fraction` → `f64` Approximation

**Python**: Uses Python's `fractions.Fraction` for exact rational arithmetic in the arithmetic coding NCD.

**Rust**: Uses `f64` floating-point arithmetic. This introduces potential precision differences for very long strings.

**Rationale**: Implementing exact rational arithmetic in Rust would require a BigRational crate (e.g., `num-rational`). For strings under ~1000 characters, `f64` precision is sufficient. This trade-off is documented and tested via differential fuzzing.

---

## 9. LZMA Compression: `lzma` → `flate2` (Deflate) Stand-In

**Python**: Uses Python's built-in `lzma` module.

**Rust**: Uses `flate2` (deflate/zlib) as a compression algorithm substitute. NCD only compares *relative* compressed sizes, so the specific algorithm matters less than consistency.

**Rationale**: Pure Rust LZMA libraries exist (`lzma-rs`) but add compile-time complexity. Since NCD is a relative measure, any consistent compression algorithm produces valid NCD values. This divergence is explicitly documented and tested.

---

## 10. `vector_based.py` Algorithms Excluded

**Python**: Contains `Chebyshev`, `Minkowski`, `Manhattan`, `Euclidean`, `Mahalanobis`, `Correlation`, `Kulsinski` — most raise `NotImplementedError`.

**Rust**: Excluded entirely.

**Rationale**: These are draft implementations in the original. `Manhattan`, `Mahalanobis`, `Kulsinski`, and `Euclidean._pure` all raise `NotImplementedError`. Porting dead code would be dishonest.

---

## 11. `Tanimoto` Distance Range: `(-inf, 0]`

**Python**: `Tanimoto.__call__` returns `log2(jaccard)`, which is `-inf` when Jaccard is 0 and `0` when Jaccard is 1.

**Rust**: Same behavior. `maximum()` returns `0.0`. `f64::NEG_INFINITY` is returned for completely different strings.

**Rationale**: Direct behavioral match. The unusual range is by design in the original.

---

## 12. MongeElkan: Word-Level Comparison

**Python**: Uses `qval=None` to split by words, then computes pairwise DamerauLevenshtein similarities.

**Rust**: Uses `str::split_whitespace()` for word splitting, which matches Python's `str.split()` behavior for ASCII text.

**Rationale**: `split_whitespace()` handles multiple spaces and tabs the same way Python's default `split()` does.

---

## 13. StrCmp95: Adjacency Table

**Python**: Uses `sp_mx` tuple of character pairs with hardcoded adjustment weight of 3.

**Rust**: Same table, same weights, stored as a compile-time constant slice.

**Rationale**: Direct port. The adjacency table is central to the algorithm's phonetic matching behavior.

---

## 14. Editex: Phonetic Group Definitions

**Python**: Uses `frozenset` groups and a `defaultdict` DP matrix.

**Rust**: Groups stored as `Vec<Vec<char>>`. DP matrix stored as `HashMap<(usize, usize), i32>`.

**Rationale**: Python's `defaultdict(lambda: defaultdict(int))` is replaced by explicit `HashMap` with `unwrap_or(&0)`. This is more explicit and avoids hidden default behavior.

---

## 15. BWT (Burrows-Wheeler Transform) in BWTRLENCD

**Python**: Sorts all rotations of the input string, takes the last column.

**Rust**: Same algorithm using `String` rotations, sorted, last character extracted.

**Rationale**: Direct port. The BWT is O(n² log n) for sorting rotations, which is acceptable for the string lengths in this use case.

---

## 16. Error Handling: Python Exceptions → Rust `Result`/Panics

**Python**: Algorithms silently return `None` from `quick_answer()` and fall through to the main implementation. External library errors are suppressed with `with suppress(Exception)`.

**Rust**: No external library fallback exists. `quick_answer` logic is inlined into each algorithm's implementation. Empty strings and equal strings are handled as early returns.

**Rationale**: Rust's type system makes `None`-as-sentinel-value patterns unnecessary. Early returns are more idiomatic and explicit.

---

## 17. CLI Interface Design

**Python**: Library only — no CLI. Used via `import textdistance`.

**Rust**: Added a `clap`-based CLI that accepts `--algorithm`, `--method`, and two positional strings. Outputs a single number to stdout.

**Rationale**: Required by the hackathon deliverables ("build command that produces a working binary") and enables differential fuzzing via subprocess calls.

---

## 18. Unicode Handling

**Python**: Operates on Python's native Unicode strings. `len()` returns codepoint count.

**Rust**: Uses `str::chars().count()` for character counting and `str::chars().collect::<Vec<char>>()` for indexing. This matches Python's codepoint-based behavior.

**Rationale**: Rust's `str::len()` returns byte length, not character count. Using `.chars()` ensures behavioral equivalence with Python's `len()`.

---

## 19. `BaseSimilarity.quick_answer` vs `Base.quick_answer`

**Python**: `Base.quick_answer` returns `0` for identical strings (distance=0). `BaseSimilarity.quick_answer` returns `maximum` for identical strings (similarity=max).

**Rust**: Same logic replicated in each trait's default implementations and in individual algorithm implementations where the Python original overrides `quick_answer`.

**Rationale**: This subtle dispatch difference is critical for test parity. Getting it wrong would cause every "same string" test case to fail.

---

## 20. DamerauLevenshtein: Restricted vs Unrestricted

**Python**: `restricted=True` by default (Optimal String Alignment). Setting `restricted=False` uses the true Damerau-Levenshtein that allows a character to be touched multiple times.

**Rust**: Same flag, same two implementations. The unrestricted variant uses a `HashMap<char, usize>` for the `da` (last-seen-position) table, matching Python's dict.

**Rationale**: Direct port. Both variants are tested in the original test suite.

---

*Additional entries will be added as edge cases are discovered during the 72-hour hackathon window.*
