use super::{BigInt, BigUInt, Sign};
use std::fmt::Display;
use std::str::FromStr;

impl From<(BigUInt, Sign)> for BigInt {
    fn from((inner, sign): (BigUInt, Sign)) -> Self {
        Self { inner, sign }
    }
}

impl From<u64> for BigUInt {
    fn from(value: u64) -> Self {
        match value {
            0 => Self(vec![]),
            value => Self(vec![value]),
        }
    }
}

impl From<u64> for BigInt {
    fn from(value: u64) -> Self {
        Self::from((value.into(), Sign::Plus))
    }
}

impl From<i64> for BigInt {
    fn from(value: i64) -> Self {
        let sign = if value >= 0 { Sign::Plus } else { Sign::Minus };
        Self::from((value.unsigned_abs().into(), sign))
    }
}

impl From<i32> for BigInt {
    fn from(value: i32) -> Self {
        let sign = if value >= 0 { Sign::Plus } else { Sign::Minus };
        Self::from(((value.unsigned_abs() as u64).into(), sign))
    }
}

impl From<Vec<u64>> for BigUInt {
    fn from(value: Vec<u64>) -> Self {
        Self(value)
    }
}

impl From<Vec<u64>> for BigInt {
    fn from(value: Vec<u64>) -> Self {
        Self::from((value.into(), Sign::Plus))
    }
}

impl TryFrom<&BigUInt> for u64 {
    type Error = ();

    fn try_from(value: &BigUInt) -> Result<Self, Self::Error> {
        match value.0.len() {
            0 => Ok(0),
            1 => Ok(value.0[0]),
            _ => Err(()),
        }
    }
}

impl From<BigUInt> for Vec<u64> {
    fn from(value: BigUInt) -> Self {
        value.0
    }
}

impl From<BigUInt> for BigInt {
    fn from(value: BigUInt) -> Self {
        Self {
            inner: value,
            sign: Sign::Plus,
        }
    }
}

impl TryFrom<BigInt> for BigUInt {
    type Error = ();

    fn try_from(value: BigInt) -> Result<Self, Self::Error> {
        match value.sign {
            Sign::Plus => Ok(value.inner),
            Sign::Minus => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseIntError {
    Empty,
    InvalidDigit,
}

impl FromStr for BigUInt {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(ParseIntError::Empty);
        }

        s.chars().try_fold(Self::from(0), |mut acc, ch| {
            if ch.is_ascii_digit() {
                let n = ch as u64 - u64::from(b'0');
                acc *= 10;
                acc += n;
                Ok(acc)
            } else {
                Err(ParseIntError::InvalidDigit)
            }
        })
    }
}

impl Display for BigUInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_zero() {
            write!(f, "0")
        } else {
            let mut buff = Vec::new();
            let mut num = self.clone();

            while !num.is_zero() {
                let (div, rem) = num.div_rem(Self::from(10));
                num = div;
                let digit = u64::try_from(&rem).unwrap() as u32;
                buff.push(char::from_digit(digit, 10).unwrap());
            }

            write!(f, "{}", buff.into_iter().rev().collect::<String>())
        }
    }
}

impl FromStr for BigInt {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.trim().strip_prefix('-') {
            Ok(Self::from((s.parse()?, Sign::Minus)).fix_zero())
        } else {
            Ok(Self::from((s.parse()?, Sign::Plus)))
        }
    }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = match self.sign {
            Sign::Plus => "",
            Sign::Minus => "-",
        };

        write!(f, "{}", sign)?;

        self.inner.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biguint_from_u64_test() {
        assert_eq!(BigUInt::from(0).0, vec![]);
        assert_eq!(BigUInt::from(123_456_789).0, vec![123_456_789]);
    }

    #[test]
    fn biguint_parse_test() {
        assert_eq!("0".parse(), Ok(BigUInt::from(0)));
        assert_eq!("123456789".parse(), Ok(BigUInt::from(123_456_789)));
        assert_eq!(
            "12345678910111213141516".parse(),
            Ok(BigUInt::from(vec![0x42B6_5689_328B_BE0C, 0x29D]))
        );
    }

    #[test]
    fn biguint_display_test() {
        assert_eq!(BigUInt::from(0).to_string(), "0");
        assert_eq!(BigUInt::from(123_456_789).to_string(), "123456789");
        assert_eq!(
            BigUInt::from(vec![0x42B6_5689_328B_BE0C, 0x29D]).to_string(),
            "12345678910111213141516"
        );
    }

    #[test]
    fn bigint_from_u64_test() {
        let bigint = BigInt::from(0u64);
        assert_eq!((bigint.inner, bigint.sign), (BigUInt::from(0), Sign::Plus));

        let bigint = BigInt::from(123_456_789u64);
        assert_eq!(
            (bigint.inner, bigint.sign),
            (BigUInt::from(123_456_789), Sign::Plus)
        );
    }

    #[test]
    fn bigint_from_i64_test() {
        let bigint = BigInt::from(0i64);
        assert_eq!((bigint.inner, bigint.sign), (BigUInt::from(0), Sign::Plus));

        let bigint = BigInt::from(1i64);
        assert_eq!((bigint.inner, bigint.sign), (BigUInt::from(1), Sign::Plus));

        let bigint = BigInt::from(-123_456_789i64);
        assert_eq!(
            (bigint.inner, bigint.sign),
            (BigUInt::from(123_456_789), Sign::Minus)
        );
    }

    #[test]
    fn bigint_parse_test() {
        assert_eq!("0".parse(), Ok(BigInt::from(0i64)));
        assert_eq!("123456789".parse(), Ok(BigInt::from(123_456_789i64)));
        assert_eq!(
            "12345678910111213141516".parse(),
            Ok(BigInt::from(vec![0x42B6_5689_328B_BE0C, 0x29D]))
        );

        assert_eq!("-0".parse(), Ok(BigInt::from(0i64)));
        assert_eq!("-123456789".parse(), Ok(BigInt::from(-123_456_789i64)));
        assert_eq!(
            "-12345678910111213141516".parse(),
            Ok(BigInt::from((
                BigUInt::from(vec![0x42B6_5689_328B_BE0C, 0x29D]),
                Sign::Minus
            )))
        );
    }

    #[test]
    fn bigint_display_test() {
        assert_eq!(BigInt::from(0i64).to_string(), "0");
        assert_eq!(BigInt::from(123_456_789i64).to_string(), "123456789");
        assert_eq!(BigInt::from(-123_456_789i64).to_string(), "-123456789");
        assert_eq!(
            BigInt::from((
                BigUInt::from(vec![0x42B6_5689_328B_BE0C, 0x29D]),
                Sign::Minus
            ))
            .to_string(),
            "-12345678910111213141516"
        );
    }
}
