# The Longest UTXO chain

This app parses raw bitcoin blocks, extracts transactions, filters transactions that have one input and two outputs and finds the longest UTXO chain that consists of them.

Parts are exposed by `lib.rs` and can be used in other crates.

All tests can be run using `cargo test`

Example of use: `cargo run --release data/1M.dat`

Example data has been taken from [blocktools](https://github.com/tenthirtyone/blocktools) repository on github.
