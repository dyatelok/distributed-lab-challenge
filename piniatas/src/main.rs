use piniatas::*;
use std::io::stdin;

fn main() {
    let mut buff = String::new();
    stdin().read_line(&mut buff).expect("Failed to read line");

    let piniatas = buff
        .split_whitespace()
        .map(|num| num.parse())
        .collect::<Result<Vec<_>, _>>()
        .expect("Failed to parse input");

    let answer = piniatas_solve(&piniatas);

    println!("{answer}")
}
