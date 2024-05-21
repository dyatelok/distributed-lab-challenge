# Backpack

This is an example of classical 1/0 Knapsack problem. There are several ways to solve this problem - using dynamic programming, branch and bound, etc.

However, in this case with capacity of `1024 * 1024 ≈ 10^6` and amount of elements up to `10^5` (and possible more) it'll take `10^11` memory cells for temporary storage in case of dynamic programming to store temporary calculations for block recreation and approximately the same amount CPU operations (this is a memory bound task). And for branch and bound with possible amounts of elements up to `10^5` and worst case complexity of `O(n^2)` it makes no since.

And since we don't need an exact solution for this problem we can use suboptimal, but fast and good-enough greedy algorithm, which sorts elements by `fee/vbytes` and takes what it can. This results in pretty good results.

Example: `cargo run --release data/test.csv`
