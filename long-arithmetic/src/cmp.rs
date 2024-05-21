use super::{BigInt, BigUInt, Sign};
use std::cmp::Ordering;

impl Ord for BigUInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.0.len().cmp(&other.0.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => self
                .0
                .iter()
                .rev()
                .zip(other.0.iter().rev())
                .map(|(s, o)| match s.cmp(o) {
                    Ordering::Less => Some(Ordering::Less),
                    Ordering::Greater => Some(Ordering::Greater),
                    Ordering::Equal => None,
                })
                .find(std::option::Option::is_some)
                .flatten()
                .unwrap_or(Ordering::Equal),
        }
    }
}

impl PartialOrd for BigUInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.sign, other.sign) {
            (Sign::Plus, Sign::Minus) => Ordering::Greater,
            (Sign::Minus, Sign::Plus) => Ordering::Less,
            (Sign::Plus, Sign::Plus) => self.inner.cmp(&other.inner),
            (Sign::Minus, Sign::Minus) => other.inner.cmp(&self.inner),
        }
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biguint_ord_test() {
        let first = BigUInt::from(vec![123, 456, 789]);
        let second = BigUInt::from(vec![789, 456, 789]);
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(second.cmp(&first), Ordering::Greater);

        let first = BigUInt::from(vec![123, 456, 789, 123]);
        let second = BigUInt::from(vec![789, 456, 789]);
        assert_eq!(first.cmp(&second), Ordering::Greater);
        assert_eq!(second.cmp(&first), Ordering::Less);

        let first = BigUInt::from(vec![123, 456, 789]);
        let second = BigUInt::from(vec![123, 456, 789]);
        assert_eq!(first.cmp(&second), Ordering::Equal);
    }

    #[test]
    fn bigint_ord_test() {
        let first = "-123".parse::<BigInt>().unwrap();
        let second = "-123456789".parse::<BigInt>().unwrap();
        assert_eq!(first.cmp(&second), Ordering::Greater);
        assert_eq!(second.cmp(&first), Ordering::Less);

        let first = "-123123123123778687".parse::<BigInt>().unwrap();
        let second = "123123123123778687".parse::<BigInt>().unwrap();
        assert_eq!(first.cmp(&second), Ordering::Less);
        assert_eq!(second.cmp(&first), Ordering::Greater);

        let first = "123123123123778687".parse::<BigInt>().unwrap();
        let second = "123123123123778687".parse::<BigInt>().unwrap();
        assert_eq!(first.cmp(&second), Ordering::Equal);

        let first = "-7890123213".parse::<BigInt>().unwrap();
        let second = "-7890123213".parse::<BigInt>().unwrap();
        assert_eq!(first.cmp(&second), Ordering::Equal);
    }
}
