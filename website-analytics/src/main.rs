use serde::Deserialize;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
};

#[derive(Debug, Deserialize)]
struct Record {
    user_id: u32,
    product_id: u32,
    #[allow(unused)]
    timestamp: u32,
}

fn join(v: impl IntoIterator<Item = u32>) -> String {
    v.into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let (day1, day2) = match args[..] {
        [day1, day2] => (day1, day2),
        _ => panic!("Unsupported number of arguments! Use: `cargo run day1.csv day2.csv"),
    };

    let day1 = File::open(day1).expect("Failed to read file");
    let day1 = csv::ReaderBuilder::new()
        .from_reader(day1)
        .deserialize::<Record>()
        .map(|rec| rec.expect("Failed to deserialize csv row"))
        .fold(HashMap::new(), |mut map, rec| {
            #[allow(clippy::unwrap_or_default)]
            let v = map.entry(rec.user_id).or_insert(HashSet::new());
            v.insert(rec.product_id);
            map
        });

    let day2 = File::open(day2).expect("Failed to read file");
    let mut rdr = csv::ReaderBuilder::new().from_reader(day2);
    let day2 = rdr
        .deserialize::<Record>()
        .map(|rec| rec.expect("Failed to deserialize csv row"))
        .map(|rec| (rec.user_id, rec.product_id));

    let mut both_days = HashSet::new();
    let mut only_second_day = HashSet::new();

    day2.for_each(|rec| {
        if let Some(visited_first_day) = day1.get(&rec.0) {
            both_days.insert(rec.0);

            if !visited_first_day.contains(&rec.1) {
                only_second_day.insert(rec.0);
            }
        } else {
            only_second_day.insert(rec.0);
        }
    });

    let both_days = join(both_days);
    let only_second_day = join(only_second_day);

    println!("both_days: {both_days}");
    println!("only_second_day: {only_second_day}");
}
