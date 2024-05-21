use super::helpers::*;
use itertools::*;
use std::cmp::{Eq, PartialEq};
use std::ops::{Add, Mul, Shl, Shr, Sub};

// Unsigned arbitrary-precision numbers represented in "little-endian"-like way
// 0x1_0000_0000_0000_0000 will be represented like vec![0, 1]
// No leading zeros - 0 is represented by vec![]
#[derive(Eq, PartialEq, Clone, Debug)]
pub struct BigUInt(pub Vec<u64>);

impl BigUInt {
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0.is_empty()
    }
    #[must_use]
    pub fn divisible_by_2(&self) -> bool {
        self.0.first().unwrap_or(&0) % 2 == 0
    }
    #[must_use]
    pub fn div_rem(mut self, mut rhs: Self) -> (Self, Self) {
        match (self.is_zero(), rhs.is_zero()) {
            (_, true) => panic!("Division by zero"),

            (true, _) => (Self::from(0), Self::from(0)),
            (_, _) => {
                let self_size = self.bits_num();
                let rhs_size = rhs.bits_num();

                if self_size < rhs_size {
                    // divisor is bigger than dividend
                    (Self::from(0), self)
                } else {
                    let mut ans = Self::from(0);

                    // align divisor and dividend
                    rhs <<= self_size - rhs_size;
                    debug_assert_eq!(self.bits_num(), rhs.bits_num());

                    let mut i = self_size - rhs_size;
                    loop {
                        if self >= rhs {
                            self -= rhs.clone();
                            ans.set_bit(i as usize);
                        }
                        rhs >>= 1;
                        if i == 0 {
                            break;
                        }
                        i -= 1;
                    }

                    (ans, self)
                }
            }
        }
    }
    fn bits_num(&self) -> u32 {
        self.0.last().map_or(0, |last| {
            let rest = 64 - last.leading_zeros();
            (self.0.len() as u32 - 1) * 64 + rest
        })
    }
    fn set_bit(&mut self, n: usize) {
        let (sect, bit) = (n / 64, n % 64);
        if self.0.len() < sect + 1 {
            self.0.extend(vec![0; sect + 1 - self.0.len()]);
        }
        self.0[sect] |= 1 << bit;
    }
}

impl Shr<u32> for BigUInt {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        let (shamt64, rem) = (rhs / 64, rhs % 64);
        let tmp = shr_n64(&self, shamt64 as usize);

        shr64(tmp, rem)
    }
}

impl Shl<u32> for BigUInt {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        let (shamt64, rem) = (rhs / 64, rhs % 64);
        let tmp = shl_n64(self, shamt64 as usize);

        shl64(tmp, rem)
    }
}

impl Add for BigUInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let (mut inner, carry) = self
            .0
            .iter()
            .zip_longest(rhs.0.iter())
            .map(|x| match x {
                EitherOrBoth::Both(a, b) => (*a, *b),
                EitherOrBoth::Left(a) | EitherOrBoth::Right(a) => (*a, 0),
            })
            .fold((Vec::new(), false), |(mut v, carry), (a, b)| {
                let (tmp, carry) = a.carrying_add(b, carry);

                v.push(tmp);
                (v, carry)
            });

        if carry {
            inner.push(1);
        }

        Self(inner)
    }
}

impl Sub for BigUInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        assert!(self >= rhs, "Cannot subtract lower number from bigger");

        let (mut inner, borrow) = self
            .0
            .iter()
            .zip(rhs.0.iter().chain(std::iter::repeat(&0)))
            .fold((Vec::new(), false), |(mut v, borrow), (a, b)| {
                let (tmp, borrow) = a.borrowing_sub(*b, borrow);

                v.push(tmp);
                (v, borrow)
            });

        assert!(!borrow, "Cannot subtract lower number from bigger");

        while inner.last() == Some(&0) {
            let _ = inner.pop();
        }

        Self(inner)
    }
}

impl Mul for BigUInt {
    type Output = Self;

    // quadratic multiplication
    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self.0.into_iter().enumerate();
        let rhs = rhs.0.into_iter().enumerate();

        lhs.cartesian_product(rhs)
            .map(|((rshift, rhs), (lshift, lhs))| {
                let shamt = rshift + lshift;
                let tmp = mul_helper(lhs, rhs);
                shl_n64(tmp, shamt)
            })
            .fold(Self::from(0), |acc, n| acc + n)
    }
}

#[allow(dead_code)]
pub trait Pow<RHS> {
    type Output;

    fn pow(self, rhs: RHS) -> Self::Output;
}

impl Pow<u64> for BigUInt {
    type Output = Self;

    fn pow(mut self, mut rhs: u64) -> Self::Output {
        if rhs == 0 {
            Self::from(1)
        } else {
            let mut buff = Self::from(1);

            while rhs > 1 {
                if rhs % 2 != 0 {
                    buff *= self.clone();
                    rhs -= 1;
                }
                self *= self.clone();
                rhs /= 2;
            }

            buff * self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_zero_test() {
        let num = BigUInt::from(0);
        assert!(num.is_zero());

        let num = BigUInt::from(128);
        assert!(!num.is_zero());

        let num = BigUInt::from(127);
        assert!(!num.is_zero());

        let num = BigUInt::from(vec![0, 0, 0, 123]);
        assert!(!num.is_zero());
    }

    #[test]
    fn divisible_by_2_test() {
        let num = BigUInt::from(0);
        assert!(num.divisible_by_2());

        let num = BigUInt::from(12_345_678);
        assert!(num.divisible_by_2());

        let num = BigUInt::from(vec![12_345_678, 123, 456, 789]);
        assert!(num.divisible_by_2());
    }

    #[test]
    fn bits_num_test() {
        let num = BigUInt::from(0);
        assert_eq!(num.bits_num(), 0);

        let num = BigUInt::from(128);
        assert_eq!(num.bits_num(), 8);

        let num = BigUInt::from(127);
        assert_eq!(num.bits_num(), 7);

        let num = BigUInt::from(vec![0, 0, 0, 123]);
        assert_eq!(num.bits_num(), 64 * 3 + 7);
    }

    #[test]
    fn set_bit_test() {
        let mut num = BigUInt::from(0);
        num.set_bit(130);
        assert_eq!(num, BigUInt::from(vec![0, 0, 4]));

        let mut num = BigUInt::from(128);
        num.set_bit(130);
        assert_eq!(num, BigUInt::from(vec![128, 0, 4]));

        let mut num = BigUInt::from(vec![128, 0, 9]);
        num.set_bit(130);
        assert_eq!(num, BigUInt::from(vec![128, 0, 13]));

        let mut num = BigUInt::from(vec![128, 0, 8]);
        num.set_bit(131);
        assert_eq!(num, BigUInt::from(vec![128, 0, 8]));

        let mut num = BigUInt::from(128);
        num.set_bit(4);
        assert_eq!(num, BigUInt::from(144));

        let mut num = BigUInt::from(0);
        num.set_bit(0);
        assert_eq!(num, BigUInt::from(1));

        let mut num = BigUInt::from(0);
        num.set_bit(1);
        assert_eq!(num, BigUInt::from(2));
    }

    #[test]
    fn div_rem_test() {
        let (div, rem) = BigUInt::from(0).div_rem(BigUInt::from(1));
        assert_eq!(div, BigUInt::from(0));
        assert_eq!(rem, BigUInt::from(0));

        let (div, rem) = BigUInt::from(127).div_rem(BigUInt::from(2));
        assert_eq!(div, BigUInt::from(63));
        assert_eq!(rem, BigUInt::from(1));

        let (div, rem) = BigUInt::from(122).div_rem(BigUInt::from(3));
        assert_eq!(div, BigUInt::from(40));
        assert_eq!(rem, BigUInt::from(2));

        let (div, rem) = BigUInt::from(123456).div_rem(BigUInt::from(47));
        assert_eq!(div, BigUInt::from(2626));
        assert_eq!(rem, BigUInt::from(34));

        let (div, rem) = "1234567891011121314151617181920"
            .parse::<BigUInt>()
            .unwrap()
            .div_rem(BigUInt::from(456789101112131415));
        assert_eq!(div, BigUInt::from(2702708729269));
        assert_eq!(rem, BigUInt::from(423862836832296285));
    }

    #[test]
    fn add_test() {
        assert_eq!(BigUInt::from(0) + 123_456_789, BigUInt::from(123_456_789));
        assert_eq!(BigUInt::from(3) + 123_456_789, BigUInt::from(123_456_792));
        assert_eq!(
            BigUInt::from(u64::MAX) + BigUInt::from(u64::MAX),
            BigUInt::from(vec![u64::MAX - 1, 1])
        );
        assert_eq!(
            BigUInt::from(0) + BigUInt::from(vec![123, 456, 789]),
            BigUInt::from(vec![123, 456, 789])
        );
        assert_eq!(
            BigUInt::from(vec![987, 654, 321]) + BigUInt::from(vec![123, 456, 789]),
            BigUInt::from(vec![1110, 1110, 1110])
        );
        assert_eq!(
            BigUInt::from(vec![u64::MAX, u64::MAX, u64::MAX]) + BigUInt::from(1),
            BigUInt::from(vec![0, 0, 0, 1])
        );
        assert_eq!(
            BigUInt::from(vec![u64::MAX, u64::MAX, u64::MAX]) + BigUInt::from(vec![3, 2, 1]),
            BigUInt::from(vec![2, 2, 1, 1])
        );
    }

    #[test]
    fn sub_test() {
        assert_eq!(
            BigUInt::from(123_456_789) - BigUInt::from(0),
            BigUInt::from(123_456_789)
        );
        assert_eq!(
            BigUInt::from(123_456_789) - BigUInt::from(123_456_789),
            BigUInt::from(0)
        );
        assert_eq!(
            BigUInt::from(vec![123_456_789, 123_456_789]) - BigUInt::from(vec![9, 123_456_789]),
            BigUInt::from(vec![123_456_780])
        );
        assert_eq!(
            BigUInt::from(vec![u64::MAX, u64::MAX, 3]) - BigUInt::from(vec![u64::MAX, u64::MAX, 2]),
            BigUInt::from(vec![0, 0, 1])
        );
        assert_eq!(
            BigUInt::from(vec![0x123, 0x456, 0x789]) - BigUInt::from(vec![0x987, 0x654, 0x321]),
            BigUInt::from(vec![0xFFFF_FFFF_FFFF_F79C, 0xFFFF_FFFF_FFFF_FE01, 0x467])
        );
        assert_eq!(
            BigUInt::from(vec![0, 0, 0, 1]) - BigUInt::from(1),
            BigUInt::from(vec![u64::MAX, u64::MAX, u64::MAX]),
        );
        assert_eq!(
            BigUInt::from(vec![2, 2, 1, 1]) - BigUInt::from(vec![u64::MAX, u64::MAX, u64::MAX]),
            BigUInt::from(vec![3, 2, 1]),
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
    fn mul_test() {
        assert_eq!(BigUInt::from(0) * 123_456_789, BigUInt::from(0));
        assert_eq!(BigUInt::from(1) * 123_456_789, BigUInt::from(123_456_789));
        assert_eq!(BigUInt::from(3) * 123_456_789, BigUInt::from(370_370_367));
        assert_eq!(
            BigUInt::from(u64::MAX) * BigUInt::from(u64::MAX),
            BigUInt::from(vec![0x0000_0000_0000_0001, 0xFFFF_FFFF_FFFF_FFFE])
        );
        assert_eq!(
            BigUInt::from(vec![0xFFFF_FFFF_FFFF_FFFF, 0x1111_1111_1111_1111, 0x3333])
                * BigUInt::from(vec![0xFFFF_FFFF_FFFF_FFFF, 0x2222_2222_2222_2222, 0x3456]),
            BigUInt::from(vec![
                0x0000_0000_0000_0001,
                0xCCCC_CCCC_CCCC_CCCB,
                0x530E_CA86_41FD_51EC,
                0xCF13_579B_E024_C5E5,
                0xA77_9972
            ])
        );

        let original = BigUInt::from(ORIGINAL.to_vec());
        let pow2 = BigUInt::from(POW2.to_vec());
        let pow4 = BigUInt::from(POW4.to_vec());
        let pow8 = BigUInt::from(POW8.to_vec());
        let pow10 = BigUInt::from(POW10.to_vec());

        assert_eq!(original.clone() * original.clone(), pow2);
        assert_eq!(pow2.clone() * pow2.clone(), pow4);
        assert_eq!(pow4.clone() * pow4.clone(), pow8);
        assert_eq!(pow2 * pow8, pow10);
    }

    #[test]
    fn pow_test() {
        let original = BigUInt::from(ORIGINAL.to_vec());
        let pow2 = BigUInt::from(POW2.to_vec());
        let pow4 = BigUInt::from(POW4.to_vec());
        let pow8 = BigUInt::from(POW8.to_vec());
        let pow10 = BigUInt::from(POW10.to_vec());

        assert_eq!(original.clone().pow(0), BigUInt::from(1));
        assert_eq!(original.clone().pow(1), original);
        assert_eq!(original.clone().pow(2), pow2);
        assert_eq!(original.clone().pow(4), pow4);
        assert_eq!(original.clone().pow(8), pow8);
        assert_eq!(original.pow(10), pow10);
    }

    const LEFT: [u64; 3] = [0x0000_0000_0000_0000, 0x2000_0000_0000_0000, 0x24_608A_C0F1];
    const RIGHT: [u64; 1] = [0x123_0456_0789];
    const SHIFTED10: [u64; 1] = [0x4_8C11_581E_2400];

    #[test]
    fn shl_test() {
        assert_eq!(
            BigUInt::from(RIGHT.to_vec()) << 10,
            BigUInt::from(SHIFTED10.to_vec())
        );

        let original = BigUInt::from(RIGHT.to_vec());
        let shifted = BigUInt::from(LEFT.to_vec());

        let mut num = original.clone();
        for _ in 0..125 {
            num <<= 1;
        }
        assert_eq!(num, shifted);

        assert_eq!(original << 125, shifted);
    }

    #[test]
    fn shr_test() {
        assert_eq!(
            BigUInt::from(SHIFTED10.to_vec()) >> 10,
            BigUInt::from(RIGHT.to_vec()),
        );

        let original = BigUInt::from(LEFT.to_vec());
        let shifted = BigUInt::from(RIGHT.to_vec());

        let mut num = original.clone();
        for _ in 0..125 {
            num >>= 1;
        }
        assert_eq!(num, shifted);

        assert_eq!(original >> 125, shifted);
    }
}
