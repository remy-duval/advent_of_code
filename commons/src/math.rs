//! Utilities for math operations:
//! - [gcd](gcd): Greatest Common Divisor for any integer type
//! - [lcm](lcm): Lowest Common Multiple for any integer type
//! - [extended_gcd](extended_gcd): The extended euclidean algorithm for any signed integer type

use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

/// A trait to factorize the basic numeric operations of integers necessary for GCD and LCM
pub trait Integer:
    Clone
    + Copy
    + PartialEq
    + Eq
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
{
    /// The zero value of the integer type
    const ZERO: Self;
    /// The one value of the integer type
    const ONE: Self;
    /// Get the absolute value of this integer
    fn absolute_value(self) -> Self;
    /// The modulo of this number by another
    fn remainder_euclid(self, rhs: Self) -> Self;
}

/// An Integer that is in fact signed
pub trait SignedInteger: Integer + Neg<Output = Self> {
    /// The sign of this integer
    fn sign(self) -> Self;
}

/// Convert an integer to a floating point
pub trait IntegerToFloat: Integer {
    /// Convert this integer to a floating point
    fn to_f64(self) -> f64;
}

/// Find the Greatest Common Divisor of two integers (positive)
pub fn gcd<Int: Integer>(first: Int, second: Int) -> Int {
    let mut dividend = first.absolute_value();
    let mut divisor = second.absolute_value();
    while divisor != Int::ZERO {
        let new_divisor = dividend.remainder_euclid(divisor);
        dividend = divisor;
        divisor = new_divisor;
    }
    dividend
}

/// Find the Lowest Common Multiple of two integers (using [gcd](gcd))
#[inline]
pub fn lcm<Int: Integer>(first: Int, second: Int) -> Int {
    (first / gcd(first, second)) * second
}

/// The result of the extended GCD algorithm such that `first * a + second * b = gcd`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExtendedGcd<Int> {
    pub a: Int,
    pub b: Int,
    pub gcd: Int,
}

impl<Int> ExtendedGcd<Int> {
    #[inline]
    fn new(a: Int, b: Int, gcd: Int) -> Self {
        Self { a, b, gcd }
    }
}

/// Compute the [extended gcd] of two numbers, returning the Bezout coefficients and gcd
/// [extended gcd]: https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn extended_gcd<Int: SignedInteger>(first: Int, second: Int) -> ExtendedGcd<Int> {
    let mut current = ExtendedGcd::new(Int::ONE, Int::ZERO, first);
    let mut next = ExtendedGcd::new(Int::ZERO, Int::ONE, second);
    while next.gcd != Int::ZERO {
        let quotient = current.gcd / next.gcd;
        // Computing the next step and move the previous values to current
        next.a = std::mem::replace(&mut current.a, next.a) - quotient * next.a;
        next.b = std::mem::replace(&mut current.b, next.b) - quotient * next.b;
        next.gcd = std::mem::replace(&mut current.gcd, next.gcd) - quotient * next.gcd;
    }

    current
}

macro_rules! impl_signed {
    ($t:ty) => {
        impl Integer for $t {
            const ONE: Self = 1;
            const ZERO: Self = 0;
            #[inline]
            fn absolute_value(self) -> Self {
                self.abs()
            }
            #[inline]
            fn remainder_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
        }
        impl SignedInteger for $t {
            #[inline]
            fn sign(self) -> Self {
                match self {
                    0 => 0,
                    n if n > 0 => 1,
                    _ => -1,
                }
            }
        }
        impl IntegerToFloat for $t {
            #[inline]
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);

macro_rules! impl_unsigned {
    ($t:ty) => {
        impl Integer for $t {
            const ONE: Self = 1;
            const ZERO: Self = 0;
            #[inline]
            fn absolute_value(self) -> Self {
                self
            }
            #[inline]
            fn remainder_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }
        }
        impl IntegerToFloat for $t {
            #[inline]
            fn to_f64(self) -> f64 {
                self as f64
            }
        }
    };
}

impl_unsigned!(u8);
impl_unsigned!(u16);
impl_unsigned!(u32);
impl_unsigned!(u64);
impl_unsigned!(u128);
impl_unsigned!(usize);

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
        assert_eq!(extended_gcd::<i8>(4, 3), ExtendedGcd::new(1, -1, 1));
        assert_eq!(extended_gcd::<i16>(4, 3), ExtendedGcd::new(1, -1, 1));
        assert_eq!(extended_gcd::<i32>(4, 3), ExtendedGcd::new(1, -1, 1));
        assert_eq!(extended_gcd::<i64>(4, 3), ExtendedGcd::new(1, -1, 1));
        assert_eq!(extended_gcd::<i128>(4, 3), ExtendedGcd::new(1, -1, 1));

        // 5 * 5 + (-2) * 12 = 1
        assert_eq!(extended_gcd::<i8>(5, 12), ExtendedGcd::new(5, -2, 1));
        assert_eq!(extended_gcd::<i16>(5, 12), ExtendedGcd::new(5, -2, 1));
        assert_eq!(extended_gcd::<i32>(5, 12), ExtendedGcd::new(5, -2, 1));
        assert_eq!(extended_gcd::<i64>(5, 12), ExtendedGcd::new(5, -2, 1));
        assert_eq!(extended_gcd::<i128>(5, 12), ExtendedGcd::new(5, -2, 1));

        // (-9) * 240 + 47 * 46 = 2
        assert_eq!(extended_gcd::<i16>(240, 46), ExtendedGcd::new(-9, 47, 2));
        assert_eq!(extended_gcd::<i32>(240, 46), ExtendedGcd::new(-9, 47, 2));
        assert_eq!(extended_gcd::<i64>(240, 46), ExtendedGcd::new(-9, 47, 2));
        assert_eq!(extended_gcd::<i128>(240, 46), ExtendedGcd::new(-9, 47, 2));
    }
}
