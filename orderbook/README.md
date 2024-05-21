# Orderbook

Orderbook implementation for two currencies - UAH and USD in this case. Works by only applying FIFO step as it's said in problem statement. Top buyer (if multiple - the one who came first) sells to top seller (if multiple - the one who came first). Price is determined by buyer (he buys for the price he wants and seller may sell for better price). In real world this would be done differently and fees would be taken. But it's enough for this example.

Orderbook consists of 2 binary heaps to store sell and buy orders ascending and descending orders. Rust std [BinaryHeap](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html) was used.

Order is added using `O(1)~` time and for each matched order complexity is `O(log(n))`. This makes this implementation very efficient. Any other structure implementing interface of Priority Queue could have been used. Fibonacci Heap would be better because of it's `O(1)` decrease-key operation, but for this little application it'll probably worse in terms of absolute values.

Orderbook data structure is exposed by `lib.rs` and can be used in other crates. By using `cargo run --release` application can be run. It takes input from `StdIn` and puts output to `StdOut`.

Example of use is `cat data/test.txt | cargo run | save balance_changes.txt` (nushell)
