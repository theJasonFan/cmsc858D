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
- `wt` - with funcionality as specied here: https://rob-p.github.io/CMSC858D/assignments/02_homework_1
- `<name>_bench` - programs to time and benchmark succinct datastructures (usages in source)

## Examples:
See examples for `wt` usage in `examples/`.