//! Utilities for math operations:
//! - [gcd](gcd): Greatest Common Divisor for any integer type
//! - [lcm](lcm): Lowest Common Multiple for any integer type
//! - [extended_gcd](extended_gcd): The extended euclidean algorithm for any signed integer type

use std::ops::{Add, Div, Mul, Sub};

/// A trait to factorize the basic numeric operations of integers necessary for GCD and LCM
pub trait Integer:
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> + Copy + Eq
{
    /// The zero value of the integer type
    const ZERO: Self;
    /// The one value of the integer type
    const ONE: Self;

    /// The modulo of this number by another
    fn remainder_euclid(self, rhs: Self) -> Self;
}

/// A marker trait that indicates an Integer is in fact signed
pub trait SignedInteger: Integer {}

/// Find the Greatest Common Divisor of two integers
pub fn gcd<Int: Integer>(first: Int, second: Int) -> Int {
    let mut dividend = first;
    let mut divisor = second;
    loop {
        if divisor == Int::ZERO {
            break dividend;
        } else {
            let new_divisor = dividend.remainder_euclid(divisor);
            dividend = divisor;
            divisor = new_divisor;
        }
    }
}

/// Find the Lowest Common Multiple of two integers (using [gcd](gcd))
#[inline]
pub fn lcm<Int: Integer>(first: Int, second: Int) -> Int {
    (first / gcd(first, second)) * second
}

/// Compute the [extended gcd] of two numbers, returning the Bezout coefficients and gcd
///
/// ### Arguments
/// * `first` - The first signed integer for which to compute the gcd
/// * `second` - The second signed integer for which to compute the gcd
///
/// ### Returns
/// (a, b, gcd) such that first * a + second * b = gcd
///
/// [extended gcd]: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd<Int: SignedInteger>(first: Int, second: Int) -> (Int, Int, Int) {
    let (mut r0, mut r1) = (first, second);
    let (mut s0, mut s1) = (Int::ONE, Int::ZERO);
    let (mut t0, mut t1) = (Int::ZERO, Int::ONE);
    while r1 != Int::ZERO {
        let quotient = r0 / r1;
        let temp = r0;
        r0 = r1;
        r1 = temp - quotient * r1;
        let temp = s0;
        s0 = s1;
        s1 = temp - quotient * s1;
        let temp = t0;
        t0 = t1;
        t1 = temp - quotient * t1;
    }

    (s0, t0, r0)
}

// Macro to implement the Integer trait easily
macro_rules! impl_integer {
    ($int:ty) => {
        impl Integer for $int {
            const ZERO: Self = 0;
            const ONE: Self = 1;

            #[inline] fn remainder_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
        }
    };
}

// Macro to implement the SignedInteger trait easily
macro_rules! impl_signed_integer {
    ($int:ty) => {
        impl_integer!($int);
        impl SignedInteger for $int {}
    };
}

impl_integer!(u8);
impl_signed_integer!(i8);
impl_integer!(u16);
impl_signed_integer!(i16);
impl_integer!(u32);
impl_signed_integer!(i32);
impl_integer!(u64);
impl_signed_integer!(i64);
impl_integer!(u128);
impl_signed_integer!(i128);

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    #[test]
    /// Check that the GCD implementation works correctly
    fn test_gcd() {
        fn helper<Int: Integer + Debug>(a: Int, b: Int, c: Int, result: Int) {
            assert_eq!(result, gcd(a, gcd(b, c)));
            assert_eq!(result, gcd(gcd(a, b), c));
            assert_eq!(result, gcd(c, gcd(b, a)));
            assert_eq!(result, gcd(gcd(c, b), a));
        }

        // Known results:
        // - GCD(30, 24, 36) = 6
        // - GCD(4200, 3780, 3528) = 84

        // Unsigned integers
        helper::<u8>(30, 24, 36, 6);
        helper::<u16>(30, 24, 36, 6);
        helper::<u32>(30, 24, 36, 6);
        helper::<u64>(30, 24, 36, 6);
        helper::<u128>(30, 24, 36, 6);
        helper::<u16>(4200, 3780, 3528, 84);
        helper::<u32>(4200, 3780, 3528, 84);
        helper::<u64>(4200, 3780, 3528, 84);
        helper::<u128>(4200, 3780, 3528, 84);

        // Signed integers
        helper::<i8>(30, 24, 36, 6);
        helper::<i8>(-30, 24, -36, 6);
        helper::<i16>(30, 24, 36, 6);
        helper::<i16>(-30, 24, -36, 6);
        helper::<i32>(30, 24, 36, 6);
        helper::<i32>(-30, 24, -36, 6);
        helper::<i64>(30, 24, 36, 6);
        helper::<i64>(-30, 24, -36, 6);
        helper::<i128>(30, 24, 36, 6);
        helper::<i128>(-30, 24, -36, 6);
        helper::<i16>(4200, 3780, 3528, 84);
        helper::<i16>(4200, -3780, 3528, 84);
        helper::<i32>(4200, 3780, 3528, 84);
        helper::<i32>(4200, -3780, 3528, 84);
        helper::<i64>(4200, 3780, 3528, 84);
        helper::<i64>(4200, -3780, 3528, 84);
        helper::<i128>(4200, 3780, 3528, 84);
        helper::<i128>(4200, -3780, 3528, 84);
    }

    #[test]
    /// Check that the LCM implementation works correctly
    fn test_lcm() {
        fn helper<Int: Integer + Debug>(a: Int, b: Int, c: Int, result: Int) {
            assert_eq!(result, lcm(a, lcm(b, c)));
            assert_eq!(result, lcm(lcm(a, b), c));
            assert_eq!(result, lcm(c, lcm(b, a)));
            assert_eq!(result, lcm(lcm(c, b), a));
        }

        // Known results
        // - LCM(2, 4, 8) = 8
        // - LCM(10, 5, 4) = 20
        // - LCM(8, 9, 21) = 504

        // Unsigned integers
        helper::<u8>(2, 4, 8, 8);
        helper::<u16>(2, 4, 8, 8);
        helper::<u32>(2, 4, 8, 8);
        helper::<u64>(2, 4, 8, 8);
        helper::<u128>(2, 4, 8, 8);
        helper::<u8>(10, 5, 4, 20);
        helper::<u16>(10, 5, 4, 20);
        helper::<u32>(10, 5, 4, 20);
        helper::<u64>(10, 5, 4, 20);
        helper::<u128>(10, 5, 4, 20);
        helper::<u16>(8, 9, 21, 504);
        helper::<u32>(8, 9, 21, 504);
        helper::<u64>(8, 9, 21, 504);
        helper::<u128>(8, 9, 21, 504);

        // Signed integers
        helper::<u8>(2, 4, 8, 8);
        helper::<u16>(2, 4, 8, 8);
        helper::<u32>(2, 4, 8, 8);
        helper::<u64>(2, 4, 8, 8);
        helper::<u128>(2, 4, 8, 8);
        helper::<i16>(10, 5, 4, 20);
        helper::<i32>(10, 5, 4, 20);
        helper::<i64>(10, 5, 4, 20);
        helper::<i128>(10, 5, 4, 20);
        helper::<i16>(8, 9, 21, 504);
        helper::<i32>(8, 9, 21, 504);
        helper::<i64>(8, 9, 21, 504);
        helper::<i128>(8, 9, 21, 504);
    }

    #[test]
    fn test_extended_gcd() {
        // 1 * 4 + (-1) * 3 = 1
        assert_eq!(extended_gcd(4i8, 3i8), (1, -1, 1));
        assert_eq!(extended_gcd(4i16, 3i16), (1, -1, 1));
        assert_eq!(extended_gcd(4i32, 3i32), (1, -1, 1));
        assert_eq!(extended_gcd(4i64, 3i64), (1, -1, 1));
        assert_eq!(extended_gcd(4i128, 3i128), (1, -1, 1));

        // 5 * 5 + (-2) * 12 = 1
        assert_eq!(extended_gcd(5i8, 12i8), (5, -2, 1));
        assert_eq!(extended_gcd(5i16, 12i16), (5, -2, 1));
        assert_eq!(extended_gcd(5i32, 12i32), (5, -2, 1));
        assert_eq!(extended_gcd(5i64, 12i64), (5, -2, 1));
        assert_eq!(extended_gcd(5i128, 12i128), (5, -2, 1));

        // (-9) * 240 + 47 * 46 = 2
        assert_eq!(extended_gcd(240i16, 46i16), (-9, 47, 2));
        assert_eq!(extended_gcd(240i32, 46i32), (-9, 47, 2));
        assert_eq!(extended_gcd(240i64, 46i64), (-9, 47, 2));
        assert_eq!(extended_gcd(240i128, 46i128), (-9, 47, 2));
    }
}
