# Benchmark Methodology

## What Was Measured

We benchmark the **Rust port** against the **original Python textdistance** library on the same set of string pairs, measuring:

| Metric | Description |
|---|---|
| **p99 latency** | 99th percentile single-call latency |
| **Throughput** | Operations per second |
| **RSS** | Resident Set Size (peak memory usage) |
| **Startup time** | Time from process launch to first result |

## How It Was Measured

### Hardware
- **CPU**: (fill at benchmark time)
- **RAM**: (fill at benchmark time)
- **OS**: (fill at benchmark time)

### Workload
- **10,000 string pairs** generated deterministically from a fixed seed (seed=42).
- String lengths: 1–100 characters, ASCII alphanumeric.
- Same pairs used for both Python and Rust.

### Tools
- **criterion** (Rust) for micro-benchmarks of individual algorithms. This generates high-fidelity HTML reports with confidence intervals and regression detection.
- **time** / `/usr/bin/time -v` for RSS measurement.

### Commands

```bash
# Run the Criterion benchmark suite
cargo bench

# Open the HTML report
open target/criterion/report/index.html
```

### Confounders

- Python startup time (~30ms) is included in Python benchmarks. This favors Rust in startup-sensitive workloads.
- Python's `textdistance` may delegate to C extensions (`rapidfuzz`, `jellyfish`) when installed. We benchmark with `external=False` to compare pure Python vs pure Rust.
- Process spawning overhead in the CLI benchmark is amortized over 10,000 pairs.

## Results

See `bench/results.json` for raw data.
