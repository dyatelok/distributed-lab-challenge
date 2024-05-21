# Students's Capital

Usage `cargo run` or `cargo run -- --headless`
Example: `cat data/test.txt | cargo run -- --headless` to run test

Run all tests with `cargo test`

This implementation first stores all possible profits for each price. This is done using `BTreeMap<u32, BinaryHeap<u32>>`. Solution has to merge many `BinaryHeap`s to keep into about all possible profits that can be achieved with less than `c` capital in one `BuinaryHeap`. Average cost of merge operation is `O(n)`, so it's not a huge problem. Other structures that implement similar interfaces can be used - `BTreeSet` (`O(n+m)` merge cost) or pointer-based binary-heaps (`O(log(n)*log(m))` merge cost, but this will probably result in a cache nightmare, so it probably doesn't worth it, but it has to be benchmarked).
