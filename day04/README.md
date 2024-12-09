# Day04 Notes (README) for Advent of Code 2023

## Solutions
- Part1: 2560 (`2_560`)
- Part2:

### Methods, General
- StateMachine Search
  - (much more involved than needed, but exploring the process)
- 'matrix' rotation
- also added a slightly more elaborate errostate

### Errors Made
- state_machine state were easy to drop in the transition graph
- match arm ordering for 'readability' impacted logical ordering and attention to it
- while playful in intention, representing the board states with an enum *first* made visualizing and debugging unnecessarily frictive
  - implementing core logic that one will need to test & debug, and *then* exploring enums and other possible perfs will be much more efficient
  - i.e. we want to emphasize ease of access and visualization early on, over a 'correct' skeleton
    - for example: allowing excess input states would have made example tests easier
    - methods to diff raw input and counted input would likely saved time in net

### Logistics
- idx shuffling awkward (though not more than expected, really)

### Had to Lookup
- tracing `#[instrument(level = Level::STATE)]`
- misassumed I'd have access to private fields when defining an impl remotely

### Needs
- on-the-fly tracing-subscriber customization
- better log targeting
