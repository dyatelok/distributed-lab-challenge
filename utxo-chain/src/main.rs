use std::collections::HashMap;
use utxo_chain::*;

fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    let args = args.iter().map(|s| &**s).collect::<Vec<_>>();

    let path = match args[..] {
        [path] => path,
        _ => panic!("Unsupported number of arguments. Use: `cargo run path`"),
    };

    let mut blockchain = std::fs::File::open(path).expect("Failed to open file");

    let mut blocks = Vec::new();

    while let Some(block) = Block::read_from(&mut blockchain) {
        blocks.push(block);
    }

    let transactions = blocks
        .into_iter()
        .flat_map(|block| block.transactions)
        .flat_map(|tx| {
            if tx.inputs.len() == 1 && tx.outputs.len() == 2 {
                Some((tx.id, tx.inputs[0].prev_hash))
            } else {
                None
            }
        })
        .collect();

    let heights = get_heights(transactions);

    let chain = get_longest_chain(&heights);

    println!("Max UTXO chain height: {}", chain.len());

    for (i, hash) in chain.iter().enumerate() {
        println!("{i:3}: {}", hash_str(hash));
    }
}

fn get_heights<T: Copy + Eq + Ord + std::hash::Hash>(
    transactions: Vec<(T, T)>,
) -> HashMap<T, (u32, Option<T>, T)> {
    let transactions_map = transactions
        .iter()
        .map(|(x, y)| (x, y))
        .collect::<HashMap<_, _>>();

    let mut heights = HashMap::new();

    transactions.iter().for_each(|(this_id, prev_hash)| {
        let mut this_id = *this_id;

        let (mut height, _, mut prev) = *heights.entry(this_id).or_insert((1, None, *prev_hash));

        while let Some(prev_id) = transactions_map.get(&prev) {
            let (this_height, this_next, this_prev) =
                heights.entry(prev).or_insert((0, Some(this_id), **prev_id));

            height += 1;
            if *this_height < height {
                *this_height = height;
                *this_next = Some(this_id);

                this_id = prev;
                prev = *this_prev;
            } else {
                break;
            }
        }
    });

    heights
}

fn get_longest_chain<T: Copy + Eq + Ord + std::hash::Hash>(
    heights: &HashMap<T, (u32, Option<T>, T)>,
) -> Vec<T> {
    let (max_height, id, next, prev) = heights
        .iter()
        .map(|(id, (max_height, next, prev))| (max_height, id, next, prev))
        .max()
        .unwrap();
    let mut path = Vec::with_capacity(*max_height as usize);

    path.push(*prev);
    path.push(*id);

    let mut next = *next;

    while let Some(next_inner) = next {
        path.push(next_inner);
        next = heights.get(&next_inner).and_then(|x| x.1);
    }

    path
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_heights() {
        let transactions = vec![(1, -1)];

        let mut heights = get_heights(transactions).into_iter().collect::<Vec<_>>();
        heights.sort();

        assert_eq!(heights, vec![(1, (1, None, -1)),]);

        let transactions = vec![(1, -1), (2, 1)];

        let mut heights = get_heights(transactions).into_iter().collect::<Vec<_>>();
        heights.sort();

        assert_eq!(heights, vec![(1, (2, Some(2), -1)), (2, (1, None, 1))]);

        let transactions = vec![
            (1, 2),
            (10, 2),
            (2, 4),
            (3, 4),
            (4, 7),
            (6, 7),
            (7, -1),
            (11, 8),
            (5, 8),
            (8, 9),
            (12, 9),
            (9, -2),
        ];

        let mut heights = get_heights(transactions).into_iter().collect::<Vec<_>>();
        heights.sort();

        assert_eq!(
            heights,
            vec![
                (1, (1, None, 2)),
                (2, (2, Some(1), 4)),
                (3, (1, None, 4)),
                (4, (3, Some(2), 7)),
                (5, (1, None, 8)),
                (6, (1, None, 7)),
                (7, (4, Some(4), -1)),
                (8, (2, Some(11), 9)),
                (9, (3, Some(8), -2)),
                (10, (1, None, 2)),
                (11, (1, None, 8)),
                (12, (1, None, 9)),
            ]
        );
    }

    #[test]
    fn test_het_longest_chain() {
        let transactions = vec![
            (1, 2),
            (10, 2),
            (2, 4),
            (3, 4),
            (4, 7),
            (6, 7),
            (7, -1),
            (11, 8),
            (5, 8),
            (8, 9),
            (12, 9),
            (9, -2),
        ];

        let heights = get_heights(transactions);
        let path = get_longest_chain(&heights);

        assert_eq!(path, vec![-1, 7, 4, 2, 1])
    }
}
