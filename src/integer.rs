use core::ops::*;
use core::cmp::Ord;
use core::mem;

use num::Num;


pub trait Integer:
    Num + Ord

    + Add<Self, Output = Self>
    + Mul<Self, Output = Self>
    + Sub<Self, Output = Self>
    + Div<Self, Output = Self>
    + Rem<Self, Output = Self>

    + AddAssign<Self>
    + MulAssign<Self>
    + SubAssign<Self>
    + DivAssign<Self>
    + RemAssign<Self>

    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    fn count_ones(&self) -> u32;
    fn count_zeros(&self) -> u32;
    fn leading_zeros(&self) -> u32;
    fn trailing_zeros(&self) -> u32;
    fn rotate_left(&self, n: u32) -> Self;
    fn rotate_right(&self, n: u32) -> Self;
    fn signed_shl(&self, n: u32) -> Self;
    fn signed_shr(&self, n: u32) -> Self;
    fn integer_shl(&self, n: u32) -> Self;
    fn integer_shr(&self, n: u32) -> Self;
    fn swap_bytes(&self) -> Self;
    fn from_be(x: &Self) -> Self;
    fn from_le(x: &Self) -> Self;
    fn to_be(&self) -> Self;
    fn to_le(&self) -> Self;
    fn pow(&self, exp: u32) -> Self;
    fn gcd(&self, other: &Self) -> Self;
    fn lcm(&self, other: &Self) -> Self;
    fn is_multiple_of(&self, other: &Self) -> bool;
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
}


macro_rules! impl_core {
    ($T:ty, $S:ty, $U:ty) => {
        #[inline]
        fn count_ones(&self) -> u32 {
            <$T>::count_ones(*self)
        }
        #[inline]
        fn count_zeros(&self) -> u32 {
            <$T>::count_zeros(*self)
        }
        #[inline]
        fn leading_zeros(&self) -> u32 {
            <$T>::leading_zeros(*self)
        }
        #[inline]
        fn trailing_zeros(&self) -> u32 {
            <$T>::trailing_zeros(*self)
        }
        #[inline]
        fn rotate_left(&self, n: u32) -> Self {
            <$T>::rotate_left(*self, n)
        }
        #[inline]
        fn rotate_right(&self, n: u32) -> Self {
            <$T>::rotate_right(*self, n)
        }
        #[inline]
        fn signed_shl(&self, n: u32) -> Self {
            ((*self as $S) << n) as $T
        }
        #[inline]
        fn signed_shr(&self, n: u32) -> Self {
            ((*self as $S) >> n) as $T
        }
        #[inline]
        fn integer_shl(&self, n: u32) -> Self {
            ((*self as $U) << n) as $T
        }
        #[inline]
        fn integer_shr(&self, n: u32) -> Self {
            ((*self as $U) >> n) as $T
        }
        #[inline]
        fn swap_bytes(&self) -> Self {
            <$T>::swap_bytes(*self)
        }
        #[inline]
        fn from_be(x: &Self) -> Self {
            <$T>::from_be(*x)
        }
        #[inline]
        fn from_le(x: &Self) -> Self {
            <$T>::from_le(*x)
        }
        #[inline]
        fn to_be(&self) -> Self {
            <$T>::to_be(*self)
        }
        #[inline]
        fn to_le(&self) -> Self {
            <$T>::to_le(*self)
        }
        #[inline]
        fn pow(&self, exp: u32) -> Self {
            <$T>::pow(*self, exp)
        }
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(9.is_multiple_of(&3), true);
        /// assert_eq!(3.is_multiple_of(&9), false);
        /// ~~~
        #[inline]
        fn is_multiple_of(&self, other: &Self) -> bool {
            *self % *other == 0
        }
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(3.is_even(), false);
        /// assert_eq!(4.is_even(), true);
        /// ~~~
        #[inline]
        fn is_even(&self) -> bool {
            (*self) & 1 == 0
        }
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(3.is_odd(), true);
        /// assert_eq!(4.is_odd(), false);
        /// ~~~
        #[inline]
        fn is_odd(&self) -> bool {
            !self.is_even()
        }
    }
}

macro_rules! impl_gcd_lcd {
    ($T:ty, unsigned) => (
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(6usize.gcd(&8usize), 2usize);
        /// assert_eq!(7usize.gcd(&3usize), 1usize);
        /// ~~~
        #[inline]
        fn gcd(&self, other: &Self) -> Self {
            let mut m = *self;
            let mut n = *other;

            if m == 0 || n == 0 {
                m | n
            } else {
                let shift = (m | n).trailing_zeros();

                if m == Self::min_value() || n == Self::min_value() {
                    1 << shift
                } else {
                    m = m;
                    n = n;
                    n >>= n.trailing_zeros();

                    while m != 0 {
                        m >>= m.trailing_zeros();
                        if n > m {
                            mem::swap(&mut n, &mut m)
                        }
                        m -= n;
                    }

                    n << shift
                }
            }
        }
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(7usize.lcm(&3usize), 21usize);
        /// assert_eq!(2usize.lcm(&4usize), 4usize);
        /// ~~~
        #[inline]
        fn lcm(&self, other: &Self) -> Self {
            *self * (*other / self.gcd(other))
        }
    );
    ($T:ty, signed) => (
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(6.gcd(&8), 2);
        /// assert_eq!(7.gcd(&3), 1);
        /// ~~~
        #[inline]
        fn gcd(&self, other: &Self) -> Self {
            let mut m = *self;
            let mut n = *other;

            if m == 0 || n == 0 {
                <$T>::abs(m | n)
            } else {
                let shift = (m | n).trailing_zeros();

                if m == Self::min_value() || n == Self::min_value() {
                    <$T>::abs(1 << shift)
                } else {
                    m = <$T>::abs(m);
                    n = <$T>::abs(n);
                    n >>= n.trailing_zeros();

                    while m != 0 {
                        m >>= m.trailing_zeros();
                        if n > m {
                            mem::swap(&mut n, &mut m)
                        }
                        m -= n;
                    }

                    n << shift
                }
            }
        }
        /// # Examples
        /// ~~~
        /// use integer::Integer;
        ///
        /// assert_eq!(7.lcm(&3), 21);
        /// assert_eq!(2.lcm(&4), 4);
        /// ~~~
        #[inline]
        fn lcm(&self, other: &Self) -> Self {
            <$T>::abs(*self * (*other / self.gcd(other)))
        }
    );
}

macro_rules! impl_integer {
    ($T:ty, $S:ty, $U:ty, signed) => (
        impl Integer for $T {
            impl_core!($T, $S, $U);
            impl_gcd_lcd!($T, signed);
        }
    );
    ($T:ty, $S:ty, $U:ty, unsigned) => (
        impl Integer for $T {
            impl_core!($T, $S, $U);
            impl_gcd_lcd!($T, unsigned);
        }
    );
}

impl_integer!(u8,    i8,    u8,    unsigned);
impl_integer!(u16,   i16,   u16,   unsigned);
impl_integer!(u32,   i32,   u32,   unsigned);
impl_integer!(u64,   i64,   u64,   unsigned);
impl_integer!(usize, isize, usize, unsigned);

impl_integer!(i8,    i8,    u8,    signed);
impl_integer!(i16,   i16,   u16,   signed);
impl_integer!(i32,   i32,   u32,   signed);
impl_integer!(i64,   i64,   u64,   signed);
impl_integer!(isize, isize, usize, signed);
