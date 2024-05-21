use std::io::{stdin, stdout, Write};
use students_capital::solution;

fn prompt_int(message: &str, headless: bool) -> u32 {
    loop {
        if !headless {
            stdout()
                .write_all(message.as_bytes())
                .expect("Failed to write");
            stdout().flush().expect("Failed to flush");
        }

        let mut buff = String::new();
        let _ = stdin().read_line(&mut buff).expect("Failed to read line");
        if let Ok(num) = buff.trim_end().parse::<u32>() {
            return num;
        }
    }
}

fn prompt_vec_with_size(message: &str, size: usize, headless: bool) -> Vec<u32> {
    loop {
        if !headless {
            stdout()
                .write_all(message.as_bytes())
                .expect("Failed to write");
            stdout().flush().expect("Failed to flush");
        }

        let mut buff = String::new();
        let _ = stdin().read_line(&mut buff).expect("Failed to read line");

        if let Ok(vec) = buff
            .split_whitespace()
            .map(|s| s.parse::<u32>())
            .collect::<Result<Vec<_>, _>>()
        {
            if vec.len() == size {
                return vec;
            }
        }
    }
}

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let headless = match args[..] {
        [] => false,
        ["--headless"] => true,
        _ => {
            panic!("Unsupported number of arguments! Use: `cargo run` or `cargo run -- --headless`")
        }
    };

    let k = prompt_int("Insert number of laptops K: ", headless);
    let prices = prompt_vec_with_size("Insert vec of prices (len = k): ", k as usize, headless);
    let gains = prompt_vec_with_size("Insert vec of gains (len = k): ", k as usize, headless);
    let capital = prompt_int("Insert initial amount of capital C: ", headless);
    let n = prompt_int("Insert maximal number of repairs N: ", headless);

    let prices_gains = prices.into_iter().zip(gains).collect();

    let final_capital = solution(capital, n, prices_gains);

    if !headless {
        print!("Capital at the end of summer: ");
    }
    print!("{final_capital}");
}
