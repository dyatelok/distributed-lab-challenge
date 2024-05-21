use super::{BigInt, BigUInt};
use replace_with::replace_with;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, ShlAssign, ShrAssign, Sub,
    SubAssign,
};

impl AddAssign for BigUInt {
    fn add_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ + rhs);
    }
}

impl Add<u64> for BigUInt {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl AddAssign<u64> for BigUInt {
    fn add_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ + rhs);
    }
}

impl SubAssign for BigUInt {
    fn sub_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ - rhs);
    }
}

impl Sub<u64> for BigUInt {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        self - Self::from(rhs)
    }
}

impl SubAssign<u64> for BigUInt {
    fn sub_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ - rhs);
    }
}

impl MulAssign for BigUInt {
    fn mul_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ * rhs);
    }
}

impl Mul<u64> for BigUInt {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        self * Self::from(rhs)
    }
}

impl MulAssign<u64> for BigUInt {
    fn mul_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ * rhs);
    }
}

impl Div for BigUInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (div, _) = self.div_rem(rhs);
        div
    }
}

impl Div<u64> for BigUInt {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        self / Self::from(rhs)
    }
}

impl DivAssign<u64> for BigUInt {
    fn div_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ / rhs);
    }
}

impl DivAssign for BigUInt {
    fn div_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ / rhs);
    }
}

impl Rem for BigUInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let (_, rem) = self.div_rem(rhs);
        rem
    }
}

impl RemAssign for BigUInt {
    fn rem_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ % rhs);
    }
}

impl Rem<u64> for BigUInt {
    type Output = Self;
    fn rem(self, rhs: u64) -> Self::Output {
        let (_, rem) = self.div_rem(Self::from(rhs));
        rem
    }
}

impl RemAssign<u64> for BigUInt {
    fn rem_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ % rhs);
    }
}

impl ShlAssign<u32> for BigUInt {
    fn shl_assign(&mut self, rhs: u32) {
        replace_with(self, || unreachable!(), |self_| self_ << rhs);
    }
}

impl ShrAssign<u32> for BigUInt {
    fn shr_assign(&mut self, rhs: u32) {
        replace_with(self, || unreachable!(), |self_| self_ >> rhs);
    }
}

// -----------------------------------------------------------------------------

impl AddAssign for BigInt {
    fn add_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ + rhs);
    }
}

impl Add<u64> for BigInt {
    type Output = Self;
    fn add(self, rhs: u64) -> Self::Output {
        self + Self::from(rhs)
    }
}

impl AddAssign<u64> for BigInt {
    fn add_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ + rhs);
    }
}

impl SubAssign for BigInt {
    fn sub_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ - rhs);
    }
}

impl Sub<u64> for BigInt {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self::Output {
        self - Self::from(rhs)
    }
}

impl SubAssign<u64> for BigInt {
    fn sub_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ - rhs);
    }
}

impl MulAssign for BigInt {
    fn mul_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ * rhs);
    }
}

impl Mul<u64> for BigInt {
    type Output = Self;
    fn mul(self, rhs: u64) -> Self::Output {
        self * Self::from(rhs)
    }
}

impl MulAssign<u64> for BigInt {
    fn mul_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ * rhs);
    }
}

impl Div for BigInt {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let (div, _) = self.div_rem(rhs);
        div
    }
}

impl Div<u64> for BigInt {
    type Output = Self;
    fn div(self, rhs: u64) -> Self::Output {
        self / Self::from(rhs)
    }
}

impl DivAssign<u64> for BigInt {
    fn div_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ / rhs);
    }
}

impl DivAssign for BigInt {
    fn div_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ / rhs);
    }
}

impl Rem for BigInt {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self::Output {
        let (_, rem) = self.div_rem(rhs);
        rem
    }
}

impl RemAssign for BigInt {
    fn rem_assign(&mut self, rhs: Self) {
        replace_with(self, || unreachable!(), |self_| self_ % rhs);
    }
}

impl Rem<u64> for BigInt {
    type Output = Self;
    fn rem(self, rhs: u64) -> Self::Output {
        let (_, rem) = self.div_rem(Self::from(rhs));
        rem
    }
}

impl RemAssign<u64> for BigInt {
    fn rem_assign(&mut self, rhs: u64) {
        replace_with(self, || unreachable!(), |self_| self_ % rhs);
    }
}

impl ShlAssign<u32> for BigInt {
    fn shl_assign(&mut self, rhs: u32) {
        replace_with(self, || unreachable!(), |self_| self_ << rhs);
    }
}

impl ShrAssign<u32> for BigInt {
    fn shr_assign(&mut self, rhs: u32) {
        replace_with(self, || unreachable!(), |self_| self_ >> rhs);
    }
}
