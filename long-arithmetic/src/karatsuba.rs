use crate::helpers::mul_helper;

use super::{BigInt, BigUInt};

// splits vec into two
// example ([1, 2, 3, 4, 5, 6, 7], 4)-> ([1, 2, 3, 4], [5, 6, 7])
fn split(mut vec: Vec<u64>, mut split: usize) -> (Vec<u64>, Vec<u64>) {
    split = split.min(vec.len());
    let right = vec[split..].to_vec();
    vec.truncate(split);

    (vec, right)
}

pub fn karatsuba_mul(rhs: BigUInt, lhs: BigUInt) -> BigUInt {
    if let (Ok(rhs), Ok(lhs)) = (u64::try_from(&rhs), u64::try_from(&lhs)) {
        return mul_helper(lhs, rhs);
    }

    let right = rhs.0;
    let left = lhs.0;

    let tmp = right.len().max(left.len());
    let split_point = tmp / 2 + tmp % 2;

    let (right_lower, right_upper) = split(right, split_point);
    let (left_lower, left_upper) = split(left, split_point);

    let right_lower = BigUInt::from(right_lower);
    let right_upper = BigUInt::from(right_upper);
    let left_lower = BigUInt::from(left_lower);
    let left_upper = BigUInt::from(left_upper);

    let upper = BigInt::from(karatsuba_mul(right_upper.clone(), left_upper.clone()));
    let lower = BigInt::from(karatsuba_mul(right_lower.clone(), left_lower.clone()));
    let middle = BigInt::from(karatsuba_mul(
        right_upper + right_lower,
        left_upper + left_lower,
    )) - upper.clone()
        - lower.clone();

    ((upper << (split_point * 128) as u32) + (middle << (split_point * 64) as u32) + lower)
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_test() {
        assert_eq!(
            split(vec![1, 2, 3, 4, 5, 6, 7], 4),
            (vec![1, 2, 3, 4], vec![5, 6, 7])
        );
        assert_eq!(
            split(vec![1, 2, 3, 4, 5, 6, 7], 2),
            (vec![1, 2], vec![3, 4, 5, 6, 7])
        );
        assert_eq!(
            split(vec![1, 2, 3, 4, 5, 6, 7], 10),
            (vec![1, 2, 3, 4, 5, 6, 7], vec![])
        );
    }

    const ORIGINAL: [u64; 1] = [0x123_0456_0789];
    const POW2: [u64; 2] = [0xDBA7_EE9B_5844_C751, 0x1_4AD2];
    const POW4: [u64; 3] = [0x8601_5398_2E37_07A1, 0x0F24_B6D5_18CB_E208, 0x1_AB84_4BFA];
    const POW8: [u64; 6] = [
        0x33E5_47C2_2368_3341,
        0x438A_893F_691C_BEE9,
        0x1DD8_A227_897B_FE70,
        0x8434_FDB6_53E3_A3DF,
        0xC9F2_99D2_9EF0_90E8,
        0x2,
    ];
    const POW10: [u64; 7] = [
        0xCC35_4D9A_2913_BE91,
        0xD87F_0854_504C_4A4C,
        0x078E_81D7_CF7E_C461,
        0x063F_F73B_69F5_D08B,
        0xD396_F1E9_C39B_8D33,
        0xC7E2_8FF2_C990_3B86,
        0x3_9A9E,
    ];

    #[test]
    fn karatsuba_test() {
        let left = BigUInt::from(123);
        let right = BigUInt::from(123);
        assert_eq!(karatsuba_mul(left.clone(), right.clone()), left * right);

        let left = BigUInt::from(0);
        let right = BigUInt::from(123_456_789);
        assert_eq!(karatsuba_mul(left.clone(), right.clone()), left * right);

        let left = BigUInt::from(1);
        let right = BigUInt::from(123_456_789);
        assert_eq!(karatsuba_mul(left.clone(), right.clone()), left * right);

        let left = BigUInt::from(3);
        let right = BigUInt::from(123_456_789);
        assert_eq!(karatsuba_mul(left.clone(), right.clone()), left * right);

        let left = BigUInt::from(u64::MAX);
        let right = BigUInt::from(u64::MAX);
        assert_eq!(karatsuba_mul(left.clone(), right.clone()), left * right);

        let left = BigUInt::from(vec![0xFFFF_FFFF_FFFF_FFFF, 0x1111_1111_1111_1111, 0x3333]);
        let right = BigUInt::from(vec![0xFFFF_FFFF_FFFF_FFFF, 0x2222_2222_2222_2222, 0x3456]);
        assert_eq!(karatsuba_mul(left.clone(), right.clone()), left * right);

        let original = BigUInt::from(ORIGINAL.to_vec());
        let pow2 = BigUInt::from(POW2.to_vec());
        let pow4 = BigUInt::from(POW4.to_vec());
        let pow8 = BigUInt::from(POW8.to_vec());
        let pow10 = BigUInt::from(POW10.to_vec());

        assert_eq!(karatsuba_mul(original.clone(), original.clone()), pow2);
        assert_eq!(karatsuba_mul(pow2.clone(), pow2.clone()), pow4);
        assert_eq!(karatsuba_mul(pow4.clone(), pow4.clone()), pow8);
        assert_eq!(karatsuba_mul(pow2, pow8), pow10);
    }
}
