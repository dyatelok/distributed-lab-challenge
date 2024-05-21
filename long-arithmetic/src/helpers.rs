use super::BigUInt;

pub fn mul_helper(lhs: u64, rhs: u64) -> BigUInt {
    match lhs.widening_mul(rhs) {
        (f, 0) => BigUInt::from(f),
        (f, s) => BigUInt::from(vec![f, s]),
    }
}

pub fn split_shl(value: u64, rhs: u32) -> (u64, u64) {
    (
        value.checked_shr(64 - rhs).unwrap_or(0),
        value.checked_shl(rhs).unwrap_or(0),
    )
}

pub fn shl_n64(value: BigUInt, shamt: usize) -> BigUInt {
    let inner = value.0;
    let mut new_inner = vec![0; shamt];
    new_inner.extend(inner);

    BigUInt(new_inner)
}

pub fn split_shr(value: u64, rhs: u32) -> (u64, u64) {
    (
        value.checked_shr(rhs).unwrap_or(0),
        value.checked_shl(64 - rhs).unwrap_or(0),
    )
}

pub fn shr_n64(value: &BigUInt, shamt: usize) -> BigUInt {
    BigUInt(value.0[shamt..].to_vec())
}

pub fn shr64(value: BigUInt, rhs: u32) -> BigUInt {
    let mut iter = value
        .0
        .into_iter()
        .map(|value| split_shr(value, rhs))
        .flat_map(|(l, r)| [r, l]);

    let _ = iter.next(); // remove lower bits

    let mut buff = Vec::new();

    loop {
        match (iter.next(), iter.next()) {
            (Some(a), Some(b)) => {
                buff.push(a | b);
            }
            (Some(a), None) => {
                buff.push(a);
                break;
            }
            _ => unreachable!("iterator has even number of elements"),
        }
    }

    while buff.last() == Some(&0) {
        let _ = buff.pop();
    }

    buff.into()
}

pub fn shl64(value: BigUInt, rhs: u32) -> BigUInt {
    let mut iter = value
        .0
        .into_iter()
        .map(|value| split_shl(value, rhs))
        .flat_map(|(r, l)| [l, r]);

    let mut buff = Vec::new();

    if let Some(a) = iter.next() {
        buff.push(a);
    } else {
        return BigUInt::from(0);
    }

    loop {
        match (iter.next(), iter.next()) {
            (Some(a), Some(b)) => {
                buff.push(a | b);
            }
            (Some(a), None) => {
                buff.push(a);
                break;
            }
            _ => unreachable!("iterator has even number of elements"),
        }
    }

    while let Some(0) = buff.last() {
        let _ = buff.pop();
    }

    buff.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shl64_test() {
        assert_eq!(shl64(BigUInt::from(1), 0), BigUInt::from(1));
        assert_eq!(
            shl64(BigUInt::from(vec![0, 0, 0, 3 << 62, 1 << 63]), 1),
            BigUInt::from(vec![0, 0, 0, 1 << 63, 1, 1])
        );
        assert_eq!(
            shl64(BigUInt::from(0x123_0456_0789), 61),
            BigUInt::from(vec![0x2000_0000_0000_0000, 0x24_608A_C0F1]),
        );
    }

    #[test]
    fn shl_n64_test() {
        assert_eq!(shr_n64(&BigUInt::from(1), 0), BigUInt::from(1));
        assert_eq!(
            shl_n64(BigUInt::from(123), 5),
            BigUInt::from(vec![0, 0, 0, 0, 0, 123])
        );
        assert_eq!(
            shl_n64(
                BigUInt::from(vec![0x2000_0000_0000_0000, 0x24_608A_C0F1]),
                1
            ),
            BigUInt::from(vec![
                0x0000_0000_0000_0000,
                0x2000_0000_0000_0000,
                0x24_608A_C0F1
            ]),
        );
    }

    #[test]
    fn shr64_test() {
        assert_eq!(shr64(BigUInt::from(1), 0), BigUInt::from(1));
        assert_eq!(
            shr64(BigUInt::from(vec![0, 0, 0, 1 << 63, 1, 1]), 1),
            BigUInt::from(vec![0, 0, 0, 3 << 62, 1 << 63])
        );

        assert_eq!(
            shr64(
                BigUInt::from(vec![0x2000_0000_0000_0000, 0x24_608A_C0F1]),
                61
            ),
            BigUInt::from(0x123_0456_0789)
        );
    }

    #[test]
    fn shr_n64_test() {
        assert_eq!(shr_n64(&BigUInt::from(1), 0), BigUInt::from(1));
        assert_eq!(
            shr_n64(&BigUInt::from(vec![0, 0, 0, 0, 0, 123]), 5),
            BigUInt::from(123),
        );

        assert_eq!(
            shr_n64(
                &BigUInt::from(vec![
                    0x0000_0000_0000_0000,
                    0x2000_0000_0000_0000,
                    0x24_608A_C0F1
                ]),
                1
            ),
            BigUInt::from(vec![0x2000_0000_0000_0000, 0x24_608A_C0F1]),
        );
    }
}
