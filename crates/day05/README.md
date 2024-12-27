# Day05 Notes (README) for Advent of Code 2023

## Solutions
- Part1: 5268 (`5_268`)
- Part2: 5799 (`5_799`)

### Methods, General
- NewTypes, HashMaps&Sets, property_checks
- pub static OnceLock<comparison basis>, Ord+PartialOrd
  - Passing a reference to the page ordering structure and using that in `.cmp()` would likely be better in general.  But this was reasonable in its case.

### Errors Made
- Not an error, but passing a reference to the ordering basis struct in pages (with Eq/PartialEq defined for it) would be a more generally useful pattern. (...depends a bit) Optioned if there's a non-gaaaruanteed completeness to its state.

### Logistics
- Nicer ways to generate NewType structs for testing would be helpful.

### Had to Lookup
- `Ord` & `PartialOrd`

### Needs
-
