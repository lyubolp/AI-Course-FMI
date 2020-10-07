## Frogs task

Consider the following task. There are `2n+1` leaves on a river. There are `n` frogs on left leaves (let's call them `L-frogs`), and `n` frogs on the right leaves (let's call them `R-frogs`). The initial position can be visualized in the following way (n=3): `1 1 1 0 -1 -1 -1` (1 corresponds with `L-frogs`, -1 for `R-frogs`, 0 is empty leaf)

`L-frogs` can jump 1 or 2 leaves to the right, if the leaf is free. `R-frogs` can jump 1 or 2 leaves to the left, if the leaf is free.

The goal of the task is to print all the steps from the initial position to the following position - all the `R-frogs` should be in the left leaves and the `L-frogs` should be in the right leaves.

For example: `n=3`

Initial position: `1 1 1 _ -1 -1 -1`
End position: `-1 -1 -1 0 1 1 1`

Output should be:

```
[1, 1, 1, 0, -1, -1, -1]
[1, 1, 1, -1, 0, -1, -1]
[1, 1, 0, -1, 1, -1, -1]
[1, 0, 1, -1, 1, -1, -1]
[1, -1, 1, 0, 1, -1, -1]
[1, -1, 1, -1, 1, 0, -1]
[1, -1, 1, -1, 1, -1, 0]
[1, -1, 1, -1, 0, -1, 1]
[1, -1, 0, -1, 1, -1, 1]
[0, -1, 1, -1, 1, -1, 1]
[-1, 0, 1, -1, 1, -1, 1]
[-1, -1, 1, 0, 1, -1, 1]
[-1, -1, 1, -1, 1, 0, 1]
[-1, -1, 1, -1, 0, 1, 1]
[-1, -1, 0, -1, 1, 1, 1]
[-1, -1, -1, 0, 1, 1, 1]
```

Parameters: `1 <= n <= 100`

