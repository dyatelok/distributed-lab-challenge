use super::{BigUInt, Pow};

use std::ops::{Add, Mul, Neg, Shl, Shr, Sub};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Sign {
    Plus,
    Minus,
}

// invariant - zero is always positive
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BigInt {
    pub inner: BigUInt,
    pub sign: Sign,
}

impl BigInt {
    pub fn div_rem(self, rhs: Self) -> (Self, Self) {
        let (div, rem) = self.inner.div_rem(rhs.inner);

        match (self.sign, rhs.sign) {
            (Sign::Plus, Sign::Plus) => (
                BigInt::from((div, Sign::Plus)),
                BigInt::from((rem, Sign::Plus)),
            ),
            (Sign::Plus, Sign::Minus) => (
                BigInt::from((div, Sign::Minus)).fix_zero(),
                BigInt::from((rem, Sign::Plus)),
            ),
            (Sign::Minus, Sign::Plus) => (
                BigInt::from((div, Sign::Minus)).fix_zero(),
                BigInt::from((rem, Sign::Minus)).fix_zero(),
            ),
            (Sign::Minus, Sign::Minus) => (
                BigInt::from((div, Sign::Plus)),
                BigInt::from((rem, Sign::Minus)).fix_zero(),
            ),
        }
    }
    pub fn fix_zero(self) -> Self {
        if self.inner.is_zero() {
            Self::from((self.inner, Sign::Plus))
        } else {
            self
        }
    }
}

impl Shr<u32> for BigInt {
    type Output = Self;
    fn shr(self, rhs: u32) -> Self::Output {
        BigInt {
            inner: self.inner >> rhs,
            sign: self.sign,
        }
    }
}

impl Shl<u32> for BigInt {
    type Output = Self;
    fn shl(self, rhs: u32) -> Self::Output {
        BigInt {
            inner: self.inner << rhs,
            sign: self.sign,
        }
    }
}

impl Neg for Sign {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Sign::Plus => Sign::Minus,
            Sign::Minus => Sign::Plus,
        }
    }
}

impl Neg for BigInt {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if self.inner.is_zero() {
            self
        } else {
            BigInt::from((self.inner, self.sign.neg()))
        }
    }
}

impl Add for BigInt {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self.sign, rhs.sign) {
            (Sign::Plus, Sign::Plus) => BigInt::from((self.inner + rhs.inner, Sign::Plus)),
            (Sign::Minus, Sign::Minus) => BigInt::from((self.inner + rhs.inner, Sign::Minus)),
            (Sign::Plus, Sign::Minus) => {
                if self.inner >= rhs.inner {
                    BigInt::from((self.inner - rhs.inner, Sign::Plus))
                } else {
                    BigInt::from((rhs.inner - self.inner, Sign::Minus))
                }
            }
            (Sign::Minus, Sign::Plus) => rhs + self,
        }
    }
}

impl Sub for BigInt {
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, rhs: Self) -> Self::Output {
        self + rhs.neg()
    }
}

impl Mul for BigInt {
    type Output = Self;

    // quadratic multiplication
    fn mul(self, rhs: Self) -> Self::Output {
        let sign = if self.sign == rhs.sign {
            Sign::Plus
        } else {
            Sign::Minus
        };
        Self::from((self.inner * rhs.inner, sign))
    }
}

impl Pow<u64> for BigInt {
    type Output = Self;

    fn pow(self, rhs: u64) -> Self::Output {
        let sign = if rhs % 2 == 0 { Sign::Plus } else { self.sign };

        let inner = self.inner.pow(rhs);

        Self::from((inner, sign))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn div_rem_test() {
        let (div, rem) = BigInt::from(0).div_rem(BigInt::from(1));
        assert_eq!(div, BigInt::from(0));
        assert_eq!(rem, BigInt::from(0));

        let (div, rem) = BigInt::from(127).div_rem(BigInt::from(2));
        assert_eq!(div, BigInt::from(63));
        assert_eq!(rem, BigInt::from(1));

        let (div, rem) = BigInt::from(122).div_rem(BigInt::from(-3));
        assert_eq!(div, BigInt::from(-40));
        assert_eq!(rem, BigInt::from(2));

        let (div, rem) = BigInt::from(-123456).div_rem(BigInt::from(47));
        assert_eq!(div, BigInt::from(-2626));
        assert_eq!(rem, BigInt::from(-34));

        let (div, rem) = "-1234567891011121314151617181920"
            .parse::<BigInt>()
            .unwrap()
            .div_rem(BigInt::from(-456789101112131415i64));
        assert_eq!(div, BigInt::from(2702708729269u64));
        assert_eq!(rem, BigInt::from(-423862836832296285i64));
    }

    #[test]
    fn add_test() {
        assert_eq!(
            BigInt::from(456_789_123) + BigInt::from(123_456_789),
            BigInt::from(580_245_912)
        );
        assert_eq!(
            BigInt::from(456_789_123) + BigInt::from(-123_456_789),
            BigInt::from(333_332_334)
        );
        assert_eq!(
            BigInt::from(-456_789_123) + BigInt::from(123_456_789),
            BigInt::from(-333_332_334)
        );
        assert_eq!(
            BigInt::from(-456_789_123) + BigInt::from(-123_456_789),
            BigInt::from(-580_245_912)
        );
    }

    #[test]
    fn sub_test() {
        assert_eq!(
            BigInt::from(456_789_123) - BigInt::from(-123_456_789),
            BigInt::from(580_245_912)
        );
        assert_eq!(
            BigInt::from(456_789_123) - BigInt::from(123_456_789),
            BigInt::from(333_332_334)
        );
        assert_eq!(
            BigInt::from(-456_789_123) - BigInt::from(-123_456_789),
            BigInt::from(-333_332_334)
        );
        assert_eq!(
            BigInt::from(-456_789_123) - BigInt::from(123_456_789),
            BigInt::from(-580_245_912)
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
        assert_eq!(BigInt::from(0) * 123_456_789, BigInt::from(0));
        assert_eq!(BigInt::from(-1) * 123_456_789, BigInt::from(-123_456_789));
        assert_eq!(BigInt::from(-3) * 123_456_789, BigInt::from(-370_370_367));
        assert_eq!(
            BigInt::from(u64::MAX) * BigInt::from(u64::MAX),
            BigInt::from(vec![0x0000_0000_0000_0001, 0xFFFF_FFFF_FFFF_FFFE])
        );
        assert_eq!(
            BigInt::from((
                BigUInt::from([0xFFFF_FFFF_FFFF_FFFF, 0x1111_1111_1111_1111, 0x3333].to_vec()),
                Sign::Minus,
            )) * BigInt::from([0xFFFF_FFFF_FFFF_FFFF, 0x2222_2222_2222_2222, 0x3456].to_vec()),
            BigInt::from((
                BigUInt::from(
                    [
                        0x0000_0000_0000_0001,
                        0xCCCC_CCCC_CCCC_CCCB,
                        0x530E_CA86_41FD_51EC,
                        0xCF13_579B_E024_C5E5,
                        0xA77_9972
                    ]
                    .to_vec()
                ),
                Sign::Minus
            ))
        );

        let original = BigInt::from((BigUInt::from(ORIGINAL.to_vec()), Sign::Minus));
        let pow2 = BigInt::from(POW2.to_vec());
        let pow4 = BigInt::from(POW4.to_vec());
        let pow5 = BigInt::from((
            BigUInt::from(POW4.to_vec()) * BigUInt::from(ORIGINAL.to_vec()),
            Sign::Minus,
        ));
        let pow8 = BigInt::from(POW8.to_vec());
        let pow10 = BigInt::from(POW10.to_vec());

        assert_eq!(original.clone() * original.clone(), pow2);
        assert_eq!(pow2.clone() * pow2.clone(), pow4);
        assert_eq!(pow4.clone() * pow4.clone(), pow8);
        assert_eq!(pow4.clone() * original, pow5);
        assert_eq!(pow2 * pow8, pow10);
    }

    #[test]
    fn pow_test() {
        let original = BigInt::from((BigUInt::from(ORIGINAL.to_vec()), Sign::Minus));
        let pow2 = BigInt::from(POW2.to_vec());
        let pow4 = BigInt::from(POW4.to_vec());
        let pow5 = BigInt::from((
            BigUInt::from(POW4.to_vec()) * BigUInt::from(ORIGINAL.to_vec()),
            Sign::Minus,
        ));
        let pow8 = BigInt::from(POW8.to_vec());
        let pow10 = BigInt::from(POW10.to_vec());

        assert_eq!(original.clone().pow(0), BigInt::from(1));
        assert_eq!(original.clone().pow(1), original);
        assert_eq!(original.clone().pow(2), pow2);
        assert_eq!(original.clone().pow(4), pow4);
        assert_eq!(original.clone().pow(5), pow5);
        assert_eq!(original.clone().pow(8), pow8);
        assert_eq!(original.pow(10), pow10);
    }

    const LEFT: [u64; 3] = [0x0000_0000_0000_0000, 0x2000_0000_0000_0000, 0x24_608A_C0F1];
    const RIGHT: [u64; 1] = [0x123_0456_0789];
    const SHIFTED10: [u64; 1] = [0x4_8C11_581E_2400];

    #[test]
    fn shl_test() {
        let right = BigInt::from((BigUInt::from(RIGHT.to_vec()), Sign::Minus));
        let left = BigInt::from((BigUInt::from(LEFT.to_vec()), Sign::Minus));
        let shifted10 = BigInt::from((BigUInt::from(SHIFTED10.to_vec()), Sign::Minus));

        assert_eq!(right.clone() << 10, shifted10);

        let mut num = right.clone();
        for _ in 0..125 {
            num <<= 1;
        }
        assert_eq!(num, left);

        assert_eq!(right << 125, left);
    }

    #[test]
    fn shr_test() {
        let right = BigInt::from((BigUInt::from(RIGHT.to_vec()), Sign::Minus));
        let left = BigInt::from((BigUInt::from(LEFT.to_vec()), Sign::Minus));
        let shifted10 = BigInt::from((BigUInt::from(SHIFTED10.to_vec()), Sign::Minus));
        assert_eq!(shifted10.clone() >> 10, right);

        let mut num = left.clone();
        for _ in 0..125 {
            num >>= 1;
        }
        assert_eq!(num, right);

        assert_eq!(left >> 125, right);
    }
}
