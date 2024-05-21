#![feature(lazy_cell)]

pub fn piniatas_solve(piniatas: &[u32]) -> u32 {
    let len = piniatas.len();
    let mut memo = vec![vec![None; len]; len];

    piniatas_helper(&mut memo, piniatas, 0, len as i32 - 1, true, true)
}

fn piniatas_helper(
    memo: &mut Vec<Vec<Option<u32>>>,
    piniatas: &[u32],
    start: i32,
    end: i32,
    pre_start: bool,
    pre_end: bool,
) -> u32 {
    if start >= end || start < 0 || end >= piniatas.len() as i32 {
        return 0;
    }

    memo[start as usize][end as usize].unwrap_or_else(|| {
        let res = (start..=end)
            .map(|split| {
                let mut candies_split = num_candies(piniatas, split as usize);

                if split == start && !pre_start {
                    candies_split = 0;
                }
                if split == end && !pre_end {
                    candies_split = 0;
                }

                let candies_left =
                    piniatas_helper(memo, piniatas, start, split - 1, pre_start, false);
                let candies_right = piniatas_helper(memo, piniatas, split + 1, end, false, pre_end);

                candies_left + candies_split + candies_right
            })
            .max()
            .unwrap();

        memo[start as usize][end as usize] = Some(res);

        res
    })
}

fn num_candies(piniatas: &[u32], i: usize) -> u32 {
    piniatas.get(i.overflowing_add(!0).0).unwrap_or(&1)
        * piniatas.get(i).unwrap()
        * piniatas.get(i + 1).unwrap_or(&1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::*;

    fn calculate(mut piniatas: Vec<u32>, sequence: &[usize]) -> u32 {
        let mut candies = 0;

        for i in sequence {
            candies += num_candies(&piniatas, *i);
            piniatas[*i] = 0;
        }

        candies
    }

    fn dumm_solver(piniatas: Vec<u32>) -> (u32, Vec<usize>) {
        let seq_len = piniatas.len();

        (0..seq_len)
            .permutations(seq_len)
            .map(|seq| (calculate(piniatas.clone(), &seq), seq))
            .max()
            .unwrap()
    }

    #[test]
    fn test_solution() {
        let test = vec![97, 40, 28, 60, 45];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![21, 18, 69, 61, 54, 63];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![24, 3, 23, 54, 69, 5, 50, 8, 11];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![9, 57, 77, 69];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![72, 36, 8];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![27, 18, 75];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![0, 85, 82, 95];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);

        let test = vec![7, 13, 19, 19, 7, 9, 1];
        assert_eq!(piniatas_solve(&test), dumm_solver(test).0);
    }

    // use rand::Rng;
    // fn rand_vec(len: usize) -> Vec<u32> {
    //     let mut vec = Vec::with_capacity(len);
    //     let mut rng = rand::thread_rng();

    //     (0..len).for_each(|_| {
    //         vec.push(rng.gen_range(1..20));
    //     });

    //     vec
    // }

    // #[test]
    // fn rangom_test() {
    //     for _ in 0..100 {
    //         let test = rand_vec(8);
    //         assert_eq!(piniatas(test.clone()), dumm_solver(dbg!(test)).0);
    //     }
    // }
}
