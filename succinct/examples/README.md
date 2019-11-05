# Examples

Build binaries to run the following examples in respective directories.

## `dna/`
Build wavelet tree over chromosome 01 of S. cerevisae (yeast).

    $wt build chr01.fsa chr01_wt.out

The serialized wavelet tree is **smaller** than the file containing the DNA sequence.

## `tomorrow/`
Example usage of `$wt`:

- `$wt build input.txt wt.out` to build and serialize wavelet tree
- `$wt rank wt.out rank.txt` to issue rank queries
- `$wt access wt.out access.txt` to issue access queries
- `$wt select wt.out select.txt` to issue rank queries