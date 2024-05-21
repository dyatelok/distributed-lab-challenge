# Website Analytics

Example of usage: `cargo r -r data/day1.csv data/day2.csv`

This solution is very efficient and only takes `O(n + m)` time, where `n` and `m` are numbers of rows in files. It works by constructing `HashMap` from `day1` (takes `O(n)` time because insert time is `O(1)~*`) and then it looks up every element from `day2` in `day1`, which takes `O(m)` time because every single lookup is `O(1)~`

Complexity reference: [Maps performance](https://doc.rust-lang.org/std/collections/index.html#maps)
