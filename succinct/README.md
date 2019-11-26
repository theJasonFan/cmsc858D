# Succinct data structures (in Rust)

## Build
Build Crate with:

    cargo build --release

Build, extract, and zip binaries with (binaries built to `\bin`):

    sh build_zip_bins.sh

## Test
Run tests with:

    cargo test

## Key Structs

- `BitVec` - bit vector class that supports `get_int` and `set_int` to get/set words (up to 32 bits) at specified indicies
- `IntVec` - bit-packed integer vector with arbitrary word size
- `RankSupport` - Bit vector with supported constant time `rank` and log time `select` operations.
- `WT` - Wavelet tree that supports constant time `rank`, `access` operations, and log time `select`.

## Binaries:

The following binaries are built and released in `bin.zip`.
- `wt` - with funcionality as specied here: https://rob-p.github.io/CMSC858D/assignments/02_homework_1
- `<name>_bench` - programs to time and benchmark succinct datastructures (usages in source)
- `bf build <key_file> <fpr> <n distinct keys> <output>`, builds a bloom filter with maximum FPR `fpr` with the given number of expected keys. The bloom filter inserts new-line seperated strings from `key_file` and is then serialized to `output`.
- `bf query <bloom_filter> <queries>`, loads serialized `bloom_filter` from disk, queries newline separated queries from `queries`, and outputs results to standard output.

## Examples:
See examples for `wt` usage in `examples/`.