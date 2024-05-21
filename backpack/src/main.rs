use backpack::*;
use std::{fs::File, time::Instant};

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let path = match args[..] {
        [path] => path,
        _ => panic!("Unsupported number of arguments! Use: `cargo run input.csv"),
    };

    let now = Instant::now();

    let file = File::open(path).expect("Failed to open file");
    let input = csv::ReaderBuilder::new()
        .from_reader(file)
        .deserialize::<Row>()
        .map(|rec| rec.expect("Failed to deserialize csv row"))
        .collect::<Vec<_>>();

    const MAX_SIZE: u32 = 1024 * 1024; // 1 Mb

    let res = greedy(input, MAX_SIZE);

    let construction_time = now.elapsed().as_millis();

    let max_allocated_bytes = res.max_allocated_bytes;
    let total_fee = res.total_fee;
    let block_size = res.block_size;
    let ids = res.ids;
    let amount_of_transactions = ids.len();

    println!(
        "\nAmmount of transactions: {amount_of_transactions}
The block size: {block_size} bytes
Total extracted value: {total_fee}
Constructuib time: {construction_time} ms
Max memory used to store intermediate pre-calculations: {max_allocated_bytes}"
    );
}
