# MAZE

This crate contains library for generating mazes and executable to generate them from CLI. Example of use: `cargo run 20 20`
It will print the maze representation.

Example:

```
██████████████████████████████████████████████████████████████
██😐██      ██                                  ██          ██
██  ██  ██  ██████  ██████  ██████████████████  ██  ██  ██  ██
██      ██      ██      ██  ██          ██  ██  ██  ██  ██  ██
██████████████  ██████  ██████  ██████  ██  ██  ██████  ██  ██
██          ██      ██              ██  ██  ██      ██  ██  ██
██  ██████  ██████  ██████████████  ██  ██  ██████  ██  ██  ██
██  ██  ██🪤    ██🪤        ██  ██  ██  ██              ██⭐██
██  ██  ██  ██████████████  ██  ██  ██  ██████  ██████████████
██  ██  ██              ██  ██  ██  ██      ██    🪤██      ██
██  ██  ██████████  ██████  ██  ██  ██████  ██████████  ██  ██
██      ██      ██          ██          ██              ██  ██
██████  ██████  ██████████████  ██████████  ██████████████  ██
██      ██      ██              ██      ██      ██      ██  ██
██  ██████  ██████  ██████████████  ██  ██████████  ██  ██  ██
██  ██    🎖            ██      ██  ██              ██  ██  ██
██  ██████████████████  ██████  ██  ██████████████████  ██  ██
██                  ██    🪤██  ██      ██              ██  ██
██████████████████  ██████  ██  ██████  ██████████  ██████  ██
██                  ██      ██🪤    ██          ██          ██
██  ██████████████████  ██████████  ██████████  ██████████  ██
██          ██                              ██      ██      ██
██████████  ██████████████████████████████████████  ██  ██████
██      ██  ██                          ██          ██      ██
██  ██████  ██  ██████████████████████  ██  ██████████████  ██
██          ██      ██      ██      ██  ██              ██  ██
██  ██████████  ██  ██  ██  ██  ██  ██  ██████  ██████  ██  ██
██      ██      ██  ██  ██  ██  ██  ██      ██      ██  ██  ██
██████  ██████████  ██  ██  ██  ██  ██████  ██████████  ██  ██
██                  ██  ██      ██                      ██  ██
██████████████████████████████████████████████████████████████
```

There may be inconsistencies with how emojis are displayed, so some rows may be slightly off in your terminal.

- The maze is constructed using Depth First Search. Starting point is (0, 0).
- Then it finds point on the border which is farthest from the start and descends to start, forming the intended path.
- Tile map is formed and wall tiles are put on their positions (between base cells whose abs diff is not 1 and grid between them to fill the gaps)
- Start and finish tiles are put in place
- Up to 5 trap tiles are generated. Maximum of 2 on the path for player not to die.
- 0 or 1 treasure tiles are generated not on the path and to be reachable

This solution is efficient because it has no generate maze once and there'll be no failed attempts. It takes `O(h * w)` time. All cells are reachable (including treasure🎖 and exit⭐).

Some lookups to (not) put things on the path take `O(log(length))` (`HashSet` is constructed before, and it takes `O(length)` time)

However, this maze can be a bit boring because there can be no loops or freestanding tiles and only one way to the finish because of the algorithm choice. There are many other ways to generate mazes.
