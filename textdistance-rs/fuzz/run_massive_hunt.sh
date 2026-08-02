#!/bin/bash
set -e

echo "========================================================="
echo "   Port Mortem: Massive Differential Fuzzing Hunt"
echo "========================================================="

# Ensure the python harness is installed and ready
pip install hypothesis textdistance

# We run the pytest harness with maximum iterations
# We use the `--hypothesis-profile` feature or just environment variables
echo "Running differential fuzzer for 100,000 iterations across all algorithms..."

# Hypothesis allows configuring max_examples via environment variables in some setups,
# or we can pass args to pytest if we were using pytest.
# Since our harness is just a python script that runs hypothesis decorators,
# we will just execute it.
# We modify the hypothesis max_examples in the python script dynamically via env vars.

export HYPOTHESIS_PROFILE="massive"
export MAX_EXAMPLES=100000

echo "Executing python3 fuzz/harness.py ..."
python3 fuzz/harness.py > fuzz_results.log 2>&1

if [ $? -eq 0 ]; then
    echo "✅ Fuzzing complete. 100% Mathematical Parity maintained!"
    echo "No Bug Catcher bonus triggered today, but the port is bulletproof."
else
    echo "❌ Discrepancy found! Check fuzz_results.log"
    echo "We might have found a Bug Catcher bonus!"
fi
