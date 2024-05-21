#![feature(btree_cursors)]
use std::collections::{BTreeMap, BinaryHeap};
use std::ops::Bound;

fn price_profit(price_gain: Vec<(u32, u32)>) -> BTreeMap<u32, BinaryHeap<u32>> {
    price_gain
        .into_iter()
        .filter_map(|(price, gains)| gains.checked_sub(price).map(|profit| (price, profit)))
        .fold(Default::default(), |mut map, (price, profit)| {
            map.entry(price).or_default().push(profit);
            map
        })
}

fn collapse_lower(capital: u32, price_profit: &mut BTreeMap<u32, BinaryHeap<u32>>) {
    let mut buff = BinaryHeap::new();

    loop {
        let mut cursor = price_profit.upper_bound_mut(Bound::Included(&capital));

        if let Some((k, v)) = cursor.peek_prev() {
            buff.append(v);
            let k = *k;
            price_profit.remove(&k);
        } else {
            break;
        }
    }

    price_profit.insert(capital, buff);
}

fn get_next_profit(capital: u32, price_profit: &mut BTreeMap<u32, BinaryHeap<u32>>) -> Option<u32> {
    // since we won't call this function with lower capital than we have now we can collapse all lower entries into one entry
    collapse_lower(capital, price_profit);
    let mut cursor = price_profit.upper_bound_mut(Bound::Included(&capital));

    if let Some((price, profits)) = cursor.peek_prev() {
        match profits.pop() {
            None => {
                let price = *price;
                price_profit.remove(&price);
                None
            }
            Some(profit) => Some(profit),
        }
    } else {
        None
    }
}

// returns final capital
pub fn solution(mut capital: u32, n: u32, price_gain: Vec<(u32, u32)>) -> u32 {
    let mut price_profit = price_profit(price_gain);

    for _ in 0..n {
        if let Some(profit) = get_next_profit(capital, &mut price_profit) {
            capital += profit;
        } else {
            break;
        }
    }

    capital
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_price_profit() {
        let price_gain = vec![
            (10, 20),
            (10, 20),
            (10, 30),
            (10, 5),
            (20, 30),
            (42, 12),
            (17, 88),
            (17, 12),
            (17, 28),
        ];

        let price_profit = price_profit(price_gain)
            .into_iter()
            .map(|(price, profits)| (price, profits.into_sorted_vec()))
            .collect::<Vec<_>>();

        assert_eq!(
            price_profit,
            vec![(10, vec![10, 10, 20]), (17, vec![11, 71]), (20, vec![10])]
        )
    }

    #[test]
    fn test_collapse_lower() {
        let price_gain = vec![
            (5, 1333),
            (7, 122),
            (10, 20),
            (10, 20),
            (10, 30),
            (10, 5),
            (20, 30),
            (42, 12),
            (17, 88),
            (17, 12),
            (17, 28),
        ];

        let mut price_profit = price_profit(price_gain);

        collapse_lower(10, &mut price_profit);

        let price_profit = price_profit
            .into_iter()
            .map(|(price, profits)| (price, profits.into_sorted_vec()))
            .collect::<Vec<_>>();

        assert_eq!(
            price_profit,
            vec![
                (10, vec![10, 10, 20, 115, 1328]),
                (17, vec![11, 71]),
                (20, vec![10])
            ]
        )
    }

    #[test]
    fn test_get_next_profit() {
        let price_gain = vec![
            (5, 1333),
            (7, 122),
            (10, 20),
            (10, 20),
            (10, 30),
            (10, 5),
            (20, 30),
            (42, 12),
            (17, 88),
            (17, 12),
            (17, 28),
        ];

        let mut price_profit = price_profit(price_gain);

        assert_eq!(get_next_profit(4, &mut price_profit), None);
        assert_eq!(get_next_profit(9, &mut price_profit), Some(1328));
        assert_eq!(get_next_profit(9, &mut price_profit), Some(115));
        assert_eq!(get_next_profit(10, &mut price_profit), Some(20));
        assert_eq!(get_next_profit(10, &mut price_profit), Some(10));
        assert_eq!(get_next_profit(10, &mut price_profit), Some(10));
        assert_eq!(get_next_profit(10, &mut price_profit), None);
        assert_eq!(get_next_profit(16, &mut price_profit), None);
        assert_eq!(get_next_profit(17, &mut price_profit), Some(71));
        assert_eq!(get_next_profit(18, &mut price_profit), Some(11));
        assert_eq!(get_next_profit(18, &mut price_profit), None);
        assert_eq!(get_next_profit(25, &mut price_profit), Some(10));
        assert_eq!(get_next_profit(99, &mut price_profit), None);
    }

    #[test]
    fn test_solution1() {
        let price_gain = vec![
            (10, 20),
            (10, 20),
            (10, 30),
            (10, 5),
            (20, 30),
            (42, 12),
            (17, 88),
            (17, 12),
            (17, 28),
        ];
        let capital = 10;
        let n = 3;

        assert_eq!(solution(capital, n, price_gain), 112)
    }

    #[test]
    fn test_solution2() {
        let price_gain = vec![(12, 122), (7, 9), (8888, 8899)];
        let capital = 10;
        let n = 3;

        assert_eq!(solution(capital, n, price_gain), 122);
    }

    #[test]
    fn test_solution3() {
        let price_gain = vec![(4, 5), (5, 6), (6, 7), (7, 8)];
        let capital = 5;
        let n = 5;

        assert_eq!(solution(capital, n, price_gain), 9)
    }

    #[test]
    fn test_solution4() {
        let price_gain = vec![
            (20, 10),
            (28, 29),
            (57, 14),
            (62, 36),
            (94, 84),
            (26, 45),
            (4, 40),
            (10, 5),
            (53, 64),
        ];
        let capital = 45;
        let n = 4;

        assert_eq!(solution(capital, n, price_gain), 112)
    }

    #[test]
    fn test_solution5() {
        let price_gain = vec![
            (36, 22),
            (39, 17),
            (72, 57),
            (36, 81),
            (98, 41),
            (70, 11),
            (15, 32),
            (54, 63),
            (59, 71),
            (32, 84),
            (8, 13),
            (48, 92),
            (67, 17),
        ];
        let capital = 28;
        let n = 6;

        assert_eq!(solution(capital, n, price_gain), 207)
    }

    #[test]
    fn test_solution6() {
        let price_gain = vec![
            (22, 87),
            (14, 3),
            (4, 74),
            (24, 64),
            (33, 31),
            (33, 76),
            (98, 26),
            (21, 16),
            (96, 90),
            (73, 96),
            (66, 39),
            (15, 3),
            (77, 40),
            (16, 59),
            (31, 74),
        ];
        let capital = 22;
        let n = 8;

        assert_eq!(solution(capital, n, price_gain), 349)
    }
}
