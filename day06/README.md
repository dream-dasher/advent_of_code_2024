# Day06 Notes (README) for Advent of Code 2023

## Solutions

- Part1: 4711 (`4_711`)
- Part2:

### Methods, General

- New types, checked math, some validity checks only on instantiation (burntsushi unwrap style),
- Direct simulation (more work and less efficient, but worked nicely touched some visualization library work I wanted to do; mostly done outside of this project)
- Traits to simplify type-specific 'algebras' (operations)

### Errors Made

- Lots of pausing over how to deal with 2 hard and soft bounds on top of newtype ~indirection.
  - simply defining the math/interaction traits from the outset would be the simplest
  - trying to lean too much on auto-derives and then work around what they could give ultimately used time
- Tracing resulted in a 1_000x increase in runtime for Part1_full (3.3sec to 3.2ms)
  - I had two settings (1 for TracingErrors & one for general Tracing)
  - Tracing Errors had been set to `TRACE` by default and was not controlled by EnvFilter
  - _both_ Error-Subscriber and general-Subscriber needed to be >= `WARN`
    - presumably because `INFO` is the default `#[instrument]` span level
  - Never calling a subscriber results in similar gains to scrubing all tracing code by feature flag
  - Setting to `WARN` results in about 0.7ms more time. (Unsure if that scales much or is mostly fixed.)
    - Scrubing tracing code, but still starting subscriber results in similar time increase, suggesting its mostly a fixed setup cost (though it could also be tracing/log info from other crates)
  - Refactored code so I could pass trace info via clap, and skip setting up subscriber if both levels set to off:
    - double `OFF`          :     1.5ms avg.
    - double `WARN || ERROR`:     1.9ms avg.
    - double `INFO`         : 2_600.0ms avg.

```zsh
argo clean
time cargo build --release --bin day06
hyperfine --warmup=1 --shell=none --parameter-list log trace,debug,info,warn,error,off './target/release/day06 1 full --log={log} --error-log={log}' --export-markdown=.output/profiling/day06_hyperfine_logparameterscan.md
```

| Command                                    |      Mean [s] | Min [s] | Max [s] |         Relative |
| :----------------------------------------- | ------------: | ------: | ------: | ---------------: |
| `day06 1 full (--log && error-log= trace)` | 2.628 ± 0.023 |   2.597 |   2.675 | 1700.35 ± 130.29 |
| `day06 1 full (--log && error-log= debug)` | 2.629 ± 0.018 |   2.601 |   2.664 | 1701.45 ± 130.05 |
| `day06 1 full (--log && error-log= info)`  | 2.527 ± 0.022 |   2.498 |   2.561 | 1635.51 ± 125.26 |
| `day06 1 full (--log && error-log= warn)`  | 0.002 ± 0.000 |   0.002 |   0.003 |      1.24 ± 0.12 |
| `day06 1 full (--log && error-log= error)` | 0.002 ± 0.000 |   0.002 |   0.003 |      1.23 ± 0.11 |
| `day06 1 full (--log && error-log= off)`   | 0.002 ± 0.000 |   0.001 |   0.004 |             1.00 |

```zsh
Summary
  ./target/release/day06 1 full --log=off --error-log=off ran
    1.23 ± 0.11 times faster than ./target/release/day06 1 full --log=error --error-log=error
    1.24 ± 0.12 times faster than ./target/release/day06 1 full --log=warn --error-log=warn
 1635.51 ± 125.26 times faster than ./target/release/day06 1 full --log=info --error-log=info
 1700.35 ± 130.29 times faster than ./target/release/day06 1 full --log=trace --error-log=trace
 1701.45 ± 130.05 times faster than ./target/release/day06 1 full --log=debug --error-log=debug
```

### Logistics

- The more time efficient methods (e.g. just working on paths with obstacles) would have been faster, but, intentioanlly, I chose direct, step-by-step simulation as I wanted to look some visualization approaches.

### Had to Lookup

- many variations on addition (e.g. _checked_, _borrowed_, _saturating_)
- terminal methods for clearing screen and pausing (waiting on input)
  - methods for setting up an running egui (but this was mostly a parllel work route that wasn't brought back here)

### Needs

-

## Logging

# For a specific function/module in the day06 library

RUST_LOG=day06::support::subscriber=debug cargo run --bin check

# You can combine multiple filters with commas

RUST_LOG=check=debug,day06::support::subscriber=trace cargo run --bin check

# You can also use wildcards

RUST_LOG=check=debug,day06::support::\*=trace cargo run --bin check

````

The format is generally:
- For binary code: `binary_name::module::submodule`
- For library code: `crate_name::module::submodule`

You can also use more complex directives like:
```bash
# Everything at info, but debug for specific paths
RUST_LOG=info,check=debug,day06::support=debug

# Everything at error, except specific modules at debug
RUST_LOG=error,check=debug,day06::support::subscriber=debug
````
