# Day02 Notes (README) for Advent of Code 2023

## Solutions
- Part1: 326 (`326`)
- Part2: 381 (`381`)

### Methods, General
- newtype pattern (e.g. to differentiate between origin values and diff-values)
- map_windows for sliding window (kernel convolution)
- working with those differencs ("discrete calculus") <-- though direct operation on raw values, recursively looking for alts, would have been simpler for part 2
- fuzzing (as there was a hard to find error case)

### Errors Made
- Taking element property to lgoical operation too soon made code hard to read (compiler solves anyway?)
- unstable map_windows requires special allowance in root, not local file
- edge case was poorly handled in a couple ways, and not directly tested

### Logistics
- New types and sum/addition/ reference sum operation
- outputting type flow in iterators

### Had to Lookup
- mapping with slices; Vec<Res<>> inversion
- quickcheck
- rand crate
- `.get()` vs `[_]` (actually off of separate traits, former being sealed; and latter deref's result)

### Needs
- Finer trace-log targeting. (cont'd)
- write tests for edge cases as we go -- accumulates too much uncertainty otherwise
- decomposing code for testability
