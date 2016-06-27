#![allow(dead_code)]
use num::{Float, Zero, NumCast};
use typenum::*;
use generic_array::*;
use std::ops::{Add, Sub, Mul, Div};
use std::mem;
#[derive(PartialEq, Eq, Copy, Debug)]
pub struct Vector<T, N>
    where N::ArrayType: Copy,
          T: Float,
          N: ArrayLength<T>
{
    pub data: GenericArray<T, N>,
}

impl<T, N> Clone for Vector<T, N>
    where N: ArrayLength<T>,
          N::ArrayType: Copy,
          T: Float
{
    fn clone(&self) -> Self {
        Vector::<T, N> { data: self.data }
    }
}

macro_rules! as_expr { ($e:expr) => {$e} }
macro_rules! impl_op_vec{
    ($trait_name: ident, $fn_name: ident, $op: tt) => {
        impl<T, N> $trait_name for Vector<T, N>
            where N::ArrayType: Copy,
                  N: ArrayLength<T>,
                  T: Float
        {
            type Output = Vector<T, N>;
            fn $fn_name(self, other: Self) -> Self::Output {
                unsafe {
                    let mut new_data: GenericArray<T, N> = mem::uninitialized();
                    let iter = self.data
                        .iter()
                        .zip(other.data.iter())
                        .map(|(a, b)| as_expr!( *a $op *b));
                    for (index, val) in iter.enumerate() {
                        new_data[index] = val;
                    }
                    Vector::<T, N> { data: new_data }
                }
            }
        }
    }
}

impl_op_vec!(Sub, sub, -);
impl_op_vec!(Add, add, +);
impl_op_vec!(Mul, mul, *);
impl_op_vec!(Div, div, /);

macro_rules! impl_vec_new{
    ($dim: ident, $( $x: ident),*) => {
        impl<T> Vector<T, $dim>
            where T: Float + Zero,
                  Vector<T, $dim>: Copy
        {
            pub fn new($($x : T), *) -> Self {
                Self::from_slice(&[$($x),*])
            }
        }
    };
}
impl_vec_new!(U2, x, y);
impl_vec_new!(U3, x, y, z);
impl_vec_new!(U4, x, y, z, w);

macro_rules! impl_op_vec_un{
    ($trait_name: ident, $fn_name: ident, $op: tt) => {
        impl<T, N> $trait_name<T> for Vector<T, N>
            where N::ArrayType: Copy,
                  N: ArrayLength<T>,
                  T: Float
        {
            type Output = Vector<T, N>;
            fn $fn_name(self, other: T) -> Self::Output {
                unsafe {
                    let mut new_data: GenericArray<T, N> = mem::uninitialized();
                    let iter = self.data
                        .iter()
                        .map(|a| as_expr!( *a $op other));
                    for (index, val) in iter.enumerate() {
                        new_data[index] = val;
                    }
                    Vector::<T, N> { data: new_data }
                }
            }
        }
    }
}

impl_op_vec_un!(Mul, mul, *);
impl_op_vec_un!(Add, add, +);
impl_op_vec_un!(Sub, sub, -);
impl_op_vec_un!(Div, div, /);

impl<T, N: ArrayLength<T>> Vector<T, N>
    where T: Float + Zero,
          N::ArrayType: Copy,
          Vector<T, N>: Copy
{
    /// Builds a `Vector<T, N >` from a `Vector<T, N-1>` with an additional value.
    /// # Example
    /// ```
    /// use rla::vector::*;
    /// let v = Vec3f::from_one_less(Vec2f::new(1.0, 2.0), 3.0);
    /// assert!(v == Vec3f::new(1.0, 2.0, 3.0));
    /// ```
    pub fn from_one_less(first: Vector<T, Sub1<N>>, val: T) -> Vector<T, N>
        where N: Sub<B1>,
              <N as Sub<B1>>::Output: ArrayLength<T>,
              <<N as Sub<B1>>::Output as ArrayLength<T>>::ArrayType: Copy
    {
        unsafe {
            let mut data: GenericArray<T, N> = mem::uninitialized();
            for (index, val) in first.data.iter().enumerate() {
                data[index] = *val;
            }
            data[N::to_usize() - 1] = val;
            Vector::<T, N> { data: data }
        }
    }

    pub fn from_slice(slice: &[T]) -> Vector<T, N> {
        Vector::<T, N> { data: GenericArray::from_slice(slice) }
    }

    pub fn zero() -> Vector<T, N> {
        unsafe {
            let mut data: GenericArray<T, N> = mem::uninitialized();
            for val in data.iter_mut() {
                *val = T::zero();
            }
            Vector::<T, N> { data: data }
        }
    }

    pub fn max_value(self) -> T {
        self.data.iter().fold(self.data[0], |acc, &x| {
            if x > acc {
                x
            } else {
                acc
            }
        })
    }

    pub fn dot(self, other: Self) -> T {
        Iterator::zip(self.data.into_iter(), other.data.into_iter())
            .fold(T::zero(), |acc, (x, y)| acc + x * y)
    }

    pub fn length_sq(self) -> T {
        self.dot(self)
    }

    pub fn length(self) -> T {
        self.length_sq().sqrt()
    }

    pub fn project(self, other: Self) -> Self {
        other * (self.dot(other) / other.length_sq())
    }

    pub fn normalize(self) -> Option<Self> {
        let len_sq = self.length_sq();
        if len_sq == T::one() {
            Some(self)
        } else if len_sq == T::zero() {
            None
        } else {
            Some(self / len_sq.sqrt())
        }
    }

    pub fn reflect_normal(self, normal: Self) -> Self {
        let two: T = NumCast::from(2).unwrap();
        self - normal * normal.dot(self) * two
    }

    pub fn distance_sq(self, other: Self) -> T {
        (self - other).length_sq()
    }

    pub fn distance(self, other: Self) -> T {
        self.distance_sq(other).sqrt()
    }

    pub fn map<F, B>(self, f: F) -> Vector<B, N>
        where F: Fn(T) -> B,
              N: ArrayLength<B>,
              B: Float,
              N: Copy,
              <N as ArrayLength<B>>::ArrayType: Copy
    {
        self.into_iter().map(f).collect()
    }
    pub fn min(self, other: Self) -> Self {
        Iterator::zip(self.data.into_iter(), other.data.into_iter())
            .map(|(a, b)| a.min(b))
            .collect::<Self>()
    }

    pub fn max(self, other: Self) -> Self {
        Iterator::zip(self.data.into_iter(), other.data.into_iter())
            .map(|(a, b)| a.max(b))
            .collect::<Self>()
    }

    pub fn lerp(self, torwards: Self, scale: T) -> Self{
        self + (torwards - self) * scale
    }

}

use std::ops::Deref;

impl<T, N> Deref for Vector<T, N>
    where T: Float,
          N: ArrayLength<T>,
          N::ArrayType: Copy
{
    type Target = GenericArray<T, N>;
    fn deref(&self) -> &GenericArray<T, N> {
        &self.data
    }
}
use std::iter::FromIterator;

impl<T, N> FromIterator<T> for Vector<T, N>
    where T: Float,
          N: ArrayLength<T>,
          N::ArrayType: Copy,
          Vector<T, N>: Copy
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        unsafe {
            let mut data: GenericArray<T, N> = mem::uninitialized();
            let mut index = 0;
            for val in iter {
                data[index] = val;
                index += 1;
            }
            Vector::from_slice(&data)
        }
    }
}

impl<T> Vector<T, U3>
    where T: Float
{
    pub fn x(&self) -> T {
        self.data[0]
    }

    pub fn y(&self) -> T {
        self.data[1]
    }

    pub fn z(&self) -> T {
        self.data[2]
    }
}
pub type Vec4<T> = Vector<T, U4>;
pub type Vec3<T> = Vector<T, U3>;
pub type Vec2<T> = Vector<T, U2>;

pub type Vec4f = Vec4<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;

#[cfg(test)]
mod test {
    use vector::*;
    #[test]
    fn test_reflection() {
        let v1 = Vec2f::from_slice(&[1., -1.]);
        let v2 = Vec2f::from_slice(&[1., 1.]);
        let n = Vec2f::from_slice(&[0., 1.]);
        assert!(v1.reflect_normal(n) == v2);
    }
    #[test]
    fn mul_v() {
        let v = Vec2f::from_slice(&[1.0, 2.0]);
        assert!(v + v == v * 2.0);
    }

    #[test]
    fn add_vector() {
        let v1 = Vec2f::from_slice(&[1., -1.]);
        let v2 = Vec2f::from_slice(&[1., 1.]);
        let v3 = Vec2f::new(1., 2.);
        let n = Vec2f::from_slice(&[0., 1.]);
        assert!(v1 + n * 2. == v2);
    }

    #[test]
    fn lerp_vector(){
        let v1 = Vec2f::new(-1., -1.);
        let v2 = Vec2f::new(1., 1.);
        assert!(Vec2f::lerp(v1, v2, 0.5) == Vec2f::new(0., 0.));
        assert!(Vec2f::lerp(v1, v2, 0.0) == v1);
        assert!(Vec2f::lerp(v1, v2, 1.0) == v2);
    }
    #[test]
    fn distance_vec() {
        let v1 = Vec2f::from_slice(&[0.0, 2.0]);
        let v2 = Vec2f::from_slice(&[0.0, 10.0]);
        assert!(v1.distance(v2) == 8.0);
        assert!(v1.length_sq() == 4.0);
        assert!(v1.length() == 2.0);
        assert!(v2.normalize().unwrap() == Vec2f::from_slice(&[0.0, 1.0]));
        let n = Vec2f::from_slice(&[0.0, 1.0]);
        let reflect_v1 = Vec2f::from_slice(&[1.0, -1.0]);
        assert!(reflect_v1.reflect_normal(n) == Vec2f::from_slice(&[1.0, 1.0]));
        Vec3f::from_one_less(v1, 1.0);
    }
}
