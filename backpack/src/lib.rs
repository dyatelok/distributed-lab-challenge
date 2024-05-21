use serde::Deserialize;
use std::mem::size_of;

#[derive(Debug, Deserialize)]
pub struct Row {
    tx_id: u32,
    tx_size: u32,
    tx_fee: u32,
}

impl Row {
    #[allow(unused)]
    fn from(tx_id: u32, tx_size: u32, tx_fee: u32) -> Self {
        Self {
            tx_id,
            tx_size,
            tx_fee,
        }
    }
}

#[derive(Clone, Copy, Debug)]
struct Transaction {
    id: u32,
    size: u32,
    fee: u32,
    fee_per_vbyte: f32,
}

impl Ord for Transaction {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.fee_per_vbyte.total_cmp(&other.fee_per_vbyte).reverse()
    }
}

impl PartialOrd for Transaction {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Transaction {}

impl From<Row> for Transaction {
    fn from(value: Row) -> Self {
        Transaction {
            id: value.tx_id,
            size: value.tx_size,
            fee: value.tx_fee,
            fee_per_vbyte: value.tx_fee as f32 / value.tx_size as f32,
        }
    }
}

impl PartialEq for Transaction {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Res {
    pub max_allocated_bytes: usize,
    pub total_fee: u32,
    pub block_size: u32,
    pub ids: Vec<u32>,
}

pub fn greedy(rows: Vec<Row>, max_size: u32) -> Res {
    let mut transactions = rows.into_iter().map(Transaction::from).collect::<Vec<_>>();

    let max_allocated_bytes = transactions.len() * size_of::<Transaction>(); // all other values are negiglible compared to this, doesn't represent real allocated memory

    transactions.sort_unstable();
    let mut block_size = 0;

    let (ids, block_size, total_fee) = transactions
        .into_iter()
        .take_while(|tx| {
            let next_size = block_size + tx.size;
            if next_size <= max_size {
                block_size = next_size;
                true
            } else {
                false
            }
        })
        .fold((Vec::new(), 0, 0), |(mut vec, mut size, mut fee), tx| {
            vec.push(tx.id);
            size += tx.size;
            fee += tx.fee;
            (vec, size, fee)
        });

    Res {
        ids,
        total_fee,
        block_size,
        max_allocated_bytes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greedy() {
        let max_size = 19;
        let rows = vec![
            Row::from(1, 3, 24),  // 8
            Row::from(2, 6, 36),  // 6
            Row::from(3, 10, 40), // 4
            Row::from(4, 6, 42),  // 7
            Row::from(5, 5, 30),  //6
        ];
        let len = rows.len();

        let res = greedy(rows, max_size);

        assert_eq!(
            res,
            Res {
                max_allocated_bytes: len * size_of::<Transaction>(),
                total_fee: 102,
                block_size: 15,
                ids: vec![1, 4, 2]
            }
        )
    }
}
