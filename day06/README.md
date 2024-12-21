# Day06 Notes (README) for Advent of Code 2023

## Solutions
- Part1:
- Part2:

### Methods, General
-

### Errors Made
-

### Logistics
-

### Had to Lookup
-

### Needs
-



## Logging
# For a specific function/module in the day06 library
RUST_LOG=day06::support::subscriber=debug cargo run --bin check

# You can combine multiple filters with commas
RUST_LOG=check=debug,day06::support::subscriber=trace cargo run --bin check

# You can also use wildcards
RUST_LOG=check=debug,day06::support::*=trace cargo run --bin check
```

The format is generally:
- For binary code: `binary_name::module::submodule`
- For library code: `crate_name::module::submodule`

You can also use more complex directives like:
```bash
# Everything at info, but debug for specific paths
RUST_LOG=info,check=debug,day06::support=debug

# Everything at error, except specific modules at debug
RUST_LOG=error,check=debug,day06::support::subscriber=debug
