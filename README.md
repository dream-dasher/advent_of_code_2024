# [Advent of Code 2024](https://adventofcode.com/2024)

<!--toc:start-->
- [Advent of Code 2024](#advent-of-code-2024httpsadventofcodecom2024)
  - [Day 1: Historian Hysteria](#day-1-historian-hysteriahttpsadventofcodecom2024day1-code-takeawaysday01readmemd)
  - [Day 2: Red-Nosed Reports](#day-2-red-nosed-reportshttpsadventofcodecom2024day2-code-takeawaysday02readmemd)
  - [Day 3: Mull It Over](#day-3-mull-it-overhttpsadventofcodecom2024day3)
  - [Day 4: Ceres Search](#day-4-ceres-searchhttpsadventofcodecom2024day4)
  - [Day 5: Print Queue](#day-5-print-queuehttpsadventofcodecom2024day5)
  - [Day 6: Guard Gallivant](#day-6-guard-gallivanthttpsadventofcodecom2024day6)
  - [Day 7: Bridge Repair](#day-7-bridge-repairhttpsadventofcodecom2024day7)
  - [Day 8: Resonant Collinearity](#day-8-resonant-collinearityhttpsadventofcodecom2024day8)
  - [Day 9: ____](#day-9-httpsadventofcodecom2024day8)
<!--toc:end-->


## Day 1: [Historian Hysteria](https://adventofcode.com/2024/day/1) : [code takeaways](day01/README.md)
- sort lists -> pairwise distance
- freq maps -> val * freq * freq

## Day 2: [Red-Nosed Reports](https://adventofcode.com/2024/day/2) : [code takeaways](day02/README.md)
- P1: Kernel Convolution,
- P2: + Solution via Differences

```
Raw  :  1  2   1 3
Diffs: *  1  -1 2 *
Sum  :      0  1

Raw  :  1 2 6  5
Diffs: * 1 4 -1 *
Sum  :    5 3

Raw  :  1 2 6 7
Diffs: * 1 4 1 *
Sum  :    5 5

Raw  :  9   1  2
Diffs: *  -8  1  *
Sum  :   x -7
```
**NOTE**: recursive solution over raw input with forking on ‘bad transition’: simpler code
          (just wanted to do it via the diffs)

## Day 3: [Mull It Over](https://adventofcode.com/2024/day/3) : [code takeaways](day03/README.md)
- P1: pattern match & extract
```
regex: mul\((\d+),(\d+)\)
```

## Day 4: [Ceres Search](https://adventofcode.com/2024/day/4) : [code takeaways](day04/README.md)
- P1: pattern match, mult directions
```
rc

00 10 20 30
01 11 21 31
02 12 22 32

03 13 23 33


00
01 10
02 11 20

across,row : [+1, 0]    new start: 00 [0, +1] 0_MAX
down, cal  : [0, +1]    new start: 00 [+1, 0] MAX_0
up-rt, diag: [+1, -1]   new start: 00 [0, +1] 0_MAX [+1, 0] MAX_MAX
```

## Day 5: [Print Queue](https://adventofcode.com/2024/day/5) : [----]()
- P1: checks against partial order
```
a < b
a < d
c < x
d < f
l < b

l  b  <--- awkward to represent without a graph
[a{b[df]}]
[cx]


l -- b -- d -- f
a --<
     a
c -- x
```
|**id** |**<** |**>**| **~**|
| :---: | :--: | :-: | :--: |
|a      |b,d   |     |      |
|b      |      |a,l  |      |
|d      |f     |a    |      |
|c      |x     |     |      |
|x      |      |c    |      |
|f      |      |d    |      |
|l      |b     |     |      |

## Day 6: [Guard Gallivant](https://adventofcode.com/2024/day/6) : [----]()
P1: agent simulation (optional tensor multiplication)
```
Simulation while checking for location + direction repetition (3D statespace)
For speed (if dealing with many positions) one could pre-calculate "trap regions"
e.g.

T is a trap region: {<,T,T,>}  # . .
                       V,^     T # .
                               T . .
                               T . .
                               _____

As having state V || ^ in any of those positions will lead to confinement to the region.
Notes:
- Not all lead-ins to a Trap are necessarily hit  (may be separate head and repetition areas)
- We can also look at valid transitions into a Trap region
  - e.g. In the above, at some distance the below are trap lead-in states:

T is a trap region: {<,T,T,>}  # . V
                       V,^     T # V
                               T < (<,V)
                               T < (<,V)
                               _____
```
This can also be represented as Matrix Multiplication.  Specifically a 4D “binary tensor”
(binary for populate-able values .. we can do whatever with non-populate-able values)
(I’m not seeing any nice algebra, that’s not just a hidden dimension, that allows for a 3D Tensor. - e.g. no id element in direction group)

but… still you could have a 4d element rep, but that’ just a hidden tensor dimension in this case
< V ^ >

Could make a bunch of redundant elements to force a group, but awkward
`(x,_) * (_,_x) = (x,_x)`
`(x,_) * (_,_y) = (_,_y)`


Implementation-wise: a different matrix for each directional state with transitions …
```
1D (2dirs) maze version (3D Tensor):

<,> : dimensions represents reversal iff that direction
    ... note this may be "non-linear" ... as it's value dependent ...?
1: represents same direction


< [. . . . # . . . .]     > [. . . . # . . . .]
[. > 1     x              [.         x
 .     1   x               . 1       x
 .       1 x               .   1     x
 .         x               .     1 < x
 # x x x x x x x x x       # x x x x x x x x x
 .         x > 1           .         x
 .         x     1         .         x 1
 .         x       1       .         x   1
 .]        x               .]        x     1 <

<
{
{R,1,0,0,0,0,0,0,0},
{0,0,1,0,0,0,0,0,0},
{0,0,0,1,0,0,0,0,0},
{0,0,0,0,0,0,0,0,0},
{0,0,0,0,0,0,0,0,0},
{0,0,0,0,0,R,1,0,0},
{0,0,0,0,0,0,0,1,0},
{0,0,0,0,0,0,0,0,1},
{0,0,0,0,0,0,0,0,0}
}
```

if we needed to calculate paths for many positions then we could pre calculate n-steps via matrix multiplications. — given sparsity it may not be so bad …
If we save the matrix at each step (a vector of 4D points) then as soon as we see a repetition (across steps) then that


## Day 7: [Bridge Repair](https://adventofcode.com/2024/day/7) : [----]()
- P1: combinatorial

```
operators: * +
190 == 10 _ 19
21037 !=  9 _ 7 _ 18 _ 13
292 ==  11 _ 6 _ 16 _ 20
```
```
n elements -> n-1 operators -> (n-1)^2 options
```
If we wanted to avoid brute force:
 - we could look for multiplication sets that bound first
   - shrink to sequencs of mults that are within bounds
   - then test addition
We could do this recursively -- pulling back from bound edges.

We could also just start with all * and then, in parallel and recursively, replace a * by a + at each step
This allows immediate rejections if UNDER bounds
(and vice versa if starting from bottom)

  ```
  Rejecting whole branch if UNDER bounds at any node.
  0 * * * *
  1 +
  1   +
  1     +
  1       +
```
Naive + Blind parallelization would result in a lot of redundant work.
(Note: I'm **NOT** certain memoization would be helpful given how cheap the operations are.)
¿Nice way to coordinate branching to prevent redundant descent?
e.g.
```
wasteful:

0    * * * *
1a   +
2a.  + +      <---- same
2a...

1b     +
2b.  + +      <---- same
2b...
1...
```

**Solution**: only descend to one side -
(yes: this is esentialy factorial multiplication :)
Now, if we want non-cooperative parallelism. we don't introduce redundancy
```
0   | * * * *  |
1a  | +        |

1b  |   +      |
2b  | + +      |

1c  |     +    |
2c. | +   +    |
2c: |   + +    |

1d  |       +  |
2d. | +     +  |
2d: |   +   +  |
2d; |     + +  |
```
We can still kill a branch if it's UNDER bounds at any point.
It's not optimal in terms of bounds detection, but it should be correct.

I'm not sure whether we'd get much advantage from doing relative product comparison -- which would allow more early branch kills.
If the number of operations were large enough it would benefit.  (But they seem relatively short from a quick peak at input.)

**Other optimizations**:
- we're doing a lot of semi-small problems
  - of course we can parallelize each one (really eating up any parallelization utility of the above solution approach actually 🤷)
  - chances of precise repetition of subsequences seems low (but we could store all possible values)
  - we could ... round and bound ... allowing reuse of pruning information broadly ... not sure if worth, but doable

**Caution**:
- need to check max requested value and what it fits in -- (I think they all fit in u64)
- if u64 holds desired value, then any pair will be within u128 ... but it would require pair-wise bounds checking... I suppose that's fine
  - could also do saturating mul or 'erring -- I imagine using u128 would be faster, but have never compared

## Day 8: [Resonant Collinearity](https://adventofcode.com/2024/day/8) : [----]()
- P1: tuples of distances at each point, marking section that have the 2x:x ratio for certain indices ("insideness" my also need to be checked)
  - (**perf**: just calculate "anti-node distances" and check for inclusion & overlap)
**Questions**:
  - distance is Manhattan, I think
  - the problems explicitly says there are "for any pair ... there are **two** [special locations], one on either side of them" ... but this is untrue generally. Ignoring discretizatoin there would be **four**: two inside and two outside. (where special is "in - line" an with "2:1" distance)
  - if we have to check "insideness" then this isn't just the trivial distance marking and reading
    - **NOTE**: because of discreteization: there will only be inside nodes if there
    - **solution** (if needed): the outside distances will always be larger than the inside distances, so we can just calculate those
**Perf.**: we can also just calculate the distances where "anti-nodes" can exist from the raw list of elements and then just test for inclusion and overlap -- will be faster.
No inner "anti-nodes":
(no 2:1 ratios)
```
 # 1  2  3  4  5  6  7  8  9
 9  8  7  6  5  4  3  2  1  #
```
Inner "anti-nodes":
`d = x + 2x`  
i.e. `d ∈ {3n}` (1/3 of distances)  
```
(3)  |#  1  2  3|
     |3  2  1  #|
         *  *
(6)  |#  1  2  3  4  5  6|
     |6  5  4  3  2  1  #|
            *     *
(9)  |#  1  2  3  4  5  6  7  8  9|
     |9  8  7  6  5  4  3  2  1  #|
               *        *
(12) |#  1  2  3  4  5  6  7  8  9  0  1  2|
     |2  1  0  9  8  7  6  5  4  3  2  1  #|
                  *           *
```

Outer "anti-nodes":
 `d = x + x`  
 i.e. `d ∈ {n}` (all distances)
**NOTE**: there will be no further "anti-nodes" due to the share distance offset of all further locations
```
(1)           1 |#  1|
              2 |1  #|
              *
(2)        2  1 |#  1  2|
           4  3 |2  1  #|
           *
(3)     3  2  1 |#  1  2  3|
        6  5  4 |3  2  1  #|
        *
(4)  4  3  2  1 |#  1  2  3  4|
     8  7  6  5 |4  3  2  1  #|
     *
```

## Day 9: [____](https://adventofcode.com/2024/day/8) : [----]()
