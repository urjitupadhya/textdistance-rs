#!/usr/bin/env python3
"""
Differential fuzzing harness for textdistance Python vs Rust port.

Generates random string pairs, calls both implementations, and asserts
the results match within floating-point tolerance.

Usage:
    python fuzz/harness.py [--iterations 10000] [--timeout 60]
"""

import subprocess
import sys
import random
import string
import time
import json
import argparse

# Import the original Python library
import textdistance

# Algorithms to test and their CLI names
# Format: (python_instance, rust_cli_name, is_similarity_based)
ALGORITHMS = [
    (textdistance.hamming, "hamming", False),
    (textdistance.levenshtein, "levenshtein", False),
    (textdistance.damerau_levenshtein, "damerau-levenshtein", False),
    (textdistance.jaro, "jaro", True),
    (textdistance.jaro_winkler, "jaro-winkler", True),
    (textdistance.strcmp95, "strcmp95", True),
    (textdistance.mlipns, "mlipns", True),
    (textdistance.jaccard, "jaccard", True),
    (textdistance.sorensen, "sorensen", True),
    (textdistance.tversky, "tversky", True),
    (textdistance.overlap, "overlap", True),
    (textdistance.cosine, "cosine", True),
    (textdistance.bag, "bag", False),
    (textdistance.ratcliff_obershelp, "ratcliff-obershelp", True),
    (textdistance.prefix, "prefix", True),
    (textdistance.postfix, "postfix", True),
    (textdistance.identity, "identity", True),
    (textdistance.editex, "editex", False),
    (textdistance.mra, "mra", True),
]

RUST_BINARY = "target/release/textdistance"
EPSILON = 1e-6


def generate_random_string(max_len=20):
    """Generate a random ASCII string."""
    length = random.randint(0, max_len)
    return ''.join(random.choices(string.ascii_letters + string.digits, k=length))


def call_rust(algorithm: str, method: str, s1: str, s2: str) -> float:
    """Call the Rust binary and return the result."""
    result = subprocess.run(
        [RUST_BINARY, "-a", algorithm, "-m", method, s1, s2],
        capture_output=True, text=True, timeout=5,
    )
    if result.returncode != 0:
        raise RuntimeError(f"Rust binary failed: {result.stderr}")
    return float(result.stdout.strip())


def run_fuzz(iterations: int, timeout: int):
    """Run differential fuzzing."""
    start_time = time.time()
    total_tests = 0
    divergences = []
    log_entries = []

    print(f"Starting differential fuzz: {iterations} iterations, {timeout}s timeout")
    print(f"Testing {len(ALGORITHMS)} algorithms")
    print("-" * 60)

    for i in range(iterations):
        if time.time() - start_time > timeout:
            print(f"\nTimeout reached after {timeout}s")
            break

        s1 = generate_random_string()
        s2 = generate_random_string()

        for py_alg, rust_name, _ in ALGORITHMS:
            for method in ["distance", "similarity", "normalized-distance", "normalized-similarity"]:
                try:
                    # Get Python result
                    py_method = method.replace("-", "_")
                    py_result = getattr(py_alg, py_method)(s1, s2)

                    # Get Rust result
                    rust_result = call_rust(rust_name, method, s1, s2)

                    total_tests += 1

                    # Compare (handle inf/nan)
                    if py_result != py_result:  # NaN
                        if rust_result == rust_result:
                            divergences.append({
                                "algorithm": rust_name,
                                "method": method,
                                "s1": s1, "s2": s2,
                                "python": str(py_result),
                                "rust": str(rust_result),
                            })
                    elif abs(py_result) == float('inf'):
                        if py_result != rust_result:
                            divergences.append({
                                "algorithm": rust_name,
                                "method": method,
                                "s1": s1, "s2": s2,
                                "python": str(py_result),
                                "rust": str(rust_result),
                            })
                    elif abs(float(py_result) - rust_result) > EPSILON:
                        divergences.append({
                            "algorithm": rust_name,
                            "method": method,
                            "s1": s1, "s2": s2,
                            "python": str(py_result),
                            "rust": str(rust_result),
                        })

                except Exception as e:
                    log_entries.append(f"ERROR: {rust_name}.{method}('{s1}', '{s2}'): {e}")

        if (i + 1) % 100 == 0:
            elapsed = time.time() - start_time
            print(f"  [{i+1}/{iterations}] {total_tests} tests, "
                  f"{len(divergences)} divergences, {elapsed:.1f}s")

    elapsed = time.time() - start_time
    print("-" * 60)
    print(f"Completed: {total_tests} tests in {elapsed:.1f}s")
    print(f"Divergences: {len(divergences)}")

    # Write log
    log = {
        "total_tests": total_tests,
        "divergences": len(divergences),
        "elapsed_seconds": round(elapsed, 2),
        "algorithms_tested": len(ALGORITHMS),
        "divergence_details": divergences[:50],  # Cap at 50 for readability
        "errors": log_entries[:20],
    }

    with open("fuzz/log.json", "w") as f:
        json.dump(log, f, indent=2)

    # Also write plain text log
    with open("fuzz/log.txt", "w") as f:
        f.write(f"Differential Fuzz Log\n")
        f.write(f"====================\n")
        f.write(f"Total tests: {total_tests}\n")
        f.write(f"Divergences: {len(divergences)}\n")
        f.write(f"Elapsed: {elapsed:.2f}s\n")
        f.write(f"Algorithms: {len(ALGORITHMS)}\n\n")
        if divergences:
            f.write("Divergence details:\n")
            for d in divergences[:50]:
                f.write(f"  {d['algorithm']}.{d['method']}('{d['s1']}', '{d['s2']}'): "
                        f"py={d['python']}, rs={d['rust']}\n")
        else:
            f.write("No divergences found. All outputs match within epsilon.\n")

    return len(divergences)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Differential fuzz harness")
    parser.add_argument("--iterations", type=int, default=1000)
    parser.add_argument("--timeout", type=int, default=60)
    args = parser.parse_args()

    divergences = run_fuzz(args.iterations, args.timeout)
    sys.exit(1 if divergences > 0 else 0)
