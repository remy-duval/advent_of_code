//! Utilities for math operations:
//! - [gcd](gcd): Greatest Common Divisor for i64
//! - [lcm](lcm): Lowest Common Multiple for i64

use std::fmt::Debug;
use std::ops::{Add, Div, Mul, Rem, Sub};

/// A trait to factorize the basic numeric operations of integers necessary for GCD and LCM
pub trait Integer:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Copy
    + Debug
    + Eq
{
    /// Return True if the numeric value is zero
    fn is_zero(self) -> bool;

    /// Return the absolute value of this number
    fn absolute_value(self) -> Self;
}

/// Find the Greatest Common Divisor of two integers
pub fn gcd<Int: Integer>(first: Int, second: Int) -> Int {
    let mut dividend = first.absolute_value();
    let mut divisor = second.absolute_value();
    loop {
        if divisor.is_zero() {
            break dividend;
        } else {
            let new_divisor = dividend % divisor;
            dividend = divisor;
            divisor = new_divisor;
        }
    }
}

/// Find the Lowest Common Multiple of two integers (using [gcd](gcd))
pub fn lcm<Int: Integer>(first: Int, second: Int) -> Int {
    (first * second) / gcd(first, second)
}

// Macro to implement the Integer trait for all actual integer types easily
macro_rules! impl_integer {
    ($int:ty, abs = $abs:expr) => {
        impl Integer for $int {
            fn is_zero(self) -> bool {
                self == 0
            }
            fn absolute_value(self) -> Self {
                $abs(self)
            }
        }
    };
}

/// The identity function for easily implementing the absolute value of an unsigned integer
fn identity<T>(t: T) -> T {
    t
}

impl_integer!(u8, abs = identity);
impl_integer!(i8, abs = i8::abs);
impl_integer!(u16, abs = identity);
impl_integer!(i16, abs = i16::abs);
impl_integer!(u32, abs = identity);
impl_integer!(i32, abs = i32::abs);
impl_integer!(u64, abs = identity);
impl_integer!(i64, abs = i64::abs);
impl_integer!(u128, abs = identity);
impl_integer!(i128, abs = i128::abs);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Check that the GCD implementation works correctly
    fn test_gcd() {
        fn helper<Int: Integer>(a: Int, b: Int, c: Int, result: Int) {
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
        fn helper<Int: Integer>(a: Int, b: Int, c: Int, result: Int) {
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
}
