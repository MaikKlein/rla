#![allow(dead_code)]
use num::{Float, Zero, NumCast};
use typenum::*;
use generic_array::*;
use std::ops::{Add, Sub, Mul, Div, Neg, Index, IndexMut};
use std::mem;

pub type Vector<T, N> = CoreVector<T, N, VectorType>;

pub type Vec4<T> = Vector<T, U4>;
pub type Vec3<T> = Vector<T, U3>;
pub type Vec2<T> = Vector<T, U2>;

pub type Vec4f = Vec4<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;

pub type Normal<T, N> = CoreVector<T, N, NormalType>;

pub type Normal3<T> = Normal<T, U3>;
pub type Normal3f = Normal3<f32>;
//
pub type Point<T, N> = CoreVector<T, N, PointType>;

pub type Point3<T> = Point<T, U3>;
pub type Point2<T> = Point<T, U2>;
//
// pub type Point3f = Point3<f32>;
// pub type Point2f = Point2<f32>;

#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub struct VectorType;

#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub struct PointType;

#[derive(PartialEq, Eq, Copy, Debug, Clone)]
pub struct NormalType;

use std::marker;
#[derive(PartialEq, Eq, Copy, Debug)]
pub struct CoreVector<T, N, Type>
    where N::ArrayType: Copy,
          T: Float,
          N: ArrayLength<T>
{
    pub data: GenericArray<T, N>,
    _type: marker::PhantomData<Type>,
}

impl<T, N, Type> Clone for CoreVector<T, N, Type>
    where N: ArrayLength<T>,
          N::ArrayType: Copy,
          T: Float
{
    fn clone(&self) -> Self {
        CoreVector {
            data: self.data,
            _type: marker::PhantomData,
        }
    }
}

macro_rules! as_expr { ($e:expr) => {$e} }
macro_rules! impl_op_vec{
    ($trait_name: ident, $fn_name: ident, $op: tt) => {
        impl<T, N, Type> $trait_name for CoreVector<T, N, Type>
            where N::ArrayType: Copy,
                  N: ArrayLength<T>,
                  T: Float,
                  CoreVector<T, N, Type>: Copy
        {
            type Output = CoreVector<T, N, Type>;
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
                    Self::from_slice(&new_data)
                }
            }
        }
    }
 }

impl_op_vec!(Sub, sub, -);
impl_op_vec!(Add, add, +);
impl_op_vec!(Mul, mul, *);
impl_op_vec!(Div, div, /);

macro_rules! impl_op_vec_un{
    ($trait_name: ident, $fn_name: ident, $op: tt) => {
        impl<T, N, Type> $trait_name<T> for CoreVector<T, N, Type>
            where N::ArrayType: Copy,
                  N: ArrayLength<T>,
                  T: Float,
                  CoreVector<T, N, Type>: Copy
        {
            type Output = CoreVector<T, N, Type>;
            fn $fn_name(self, other: T) -> Self::Output {
                unsafe {
                    let mut new_data: GenericArray<T, N> = mem::uninitialized();
                    let iter = self.data
                        .iter()
                        .map(|a| as_expr!( *a $op other));
                    for (index, val) in iter.enumerate() {
                        new_data[index] = val;
                    }
                    Self::from_slice(&new_data)
                }
            }
        }
    }
 }

impl_op_vec_un!(Mul, mul, *);
impl_op_vec_un!(Add, add, +);
impl_op_vec_un!(Sub, sub, -);
impl_op_vec_un!(Div, div, /);

impl<T, Type> CoreVector<T, U3, Type>
    where T: Float + Zero,
          CoreVector<T, U3, Type>: Copy
{
    pub fn cross(self, other: Self) -> Self {
        Self::new(self.y() * other.z() - self.z() * other.y(),
                  self.z() * other.x() - self.x() * other.z(),
                  self.x() * other.y() - self.y() * other.x())
    }
}

impl<T, N, Type> CoreVector<T, N, Type>
    where T: Float + Zero,
          N: ArrayLength<T>,
          N::ArrayType: Copy,
          CoreVector<T, N, Type>: Copy
{
    /// Builds a `CoreVector<T, N >` from a `CoreVector<T, N-1>` with an additional value.
    /// # Example
    /// ```
    /// use rla::vector::*;
    /// let v = Vec3f::from_one_less(Vec2f::new(1.0, 2.0), 3.0);
    /// assert!(v == Vec3f::new(1.0, 2.0, 3.0));
    /// ```
    pub fn from_one_less(first: CoreVector<T, Sub1<N>, Type>, val: T) -> Self
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
            Self::from_slice(&data)
        }
    }

    pub fn from_slice(slice: &[T]) -> CoreVector<T, N, Type> {
        CoreVector {
            data: GenericArray::from_slice(slice),
            _type: marker::PhantomData,
        }
    }
    pub fn zero() -> Self {
        unsafe {
            let mut data: GenericArray<T, N> = mem::uninitialized();
            for val in data.iter_mut() {
                *val = T::zero();
            }
            Self::from_slice(&data)
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

    pub fn map<F, B>(self, f: F) -> CoreVector<B, N, Type>
        where F: Fn(T) -> B,
              N: ArrayLength<B>,
              B: Float,
              N: Copy,
              <N as ArrayLength<B>>::ArrayType: Copy,
              CoreVector<B, N, Type>: Copy
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

    pub fn lerp(self, torwards: Self, scale: T) -> Self {
        self + (torwards - self) * scale
    }

    pub fn dim(&self) -> usize {
        N::to_usize()
    }

    pub fn extend(self, val: T) -> CoreVector<T, Add1<N>, Type>
        where N: Add<B1>,
                 <N as Add<B1>>::Output: ArrayLength<T>,
                 <<N as Add<B1>>::Output as ArrayLength<T>>::ArrayType: Copy,
                 CoreVector<T, Add1<N>, Type> : Copy
    {
        let mut v = CoreVector::zero();
        for (index, self_val) in self.into_iter().enumerate() {
            v.data[index] = self_val;
        }
        let last_index = v.dim() - 1;
        v.data[last_index] = val;
        v
    }

    pub fn truncate(self) -> CoreVector<T, Sub1<N>, Type>
        where N: Sub<B1>,
              <N as Sub<B1>>::Output: ArrayLength<T>,
              <<N as Sub<B1>>::Output as ArrayLength<T>>::ArrayType: Copy,
              CoreVector<T, Sub1<N>, Type> : Copy
    {
        let mut v = CoreVector::zero();
        for (index, val) in self.into_iter().enumerate() {
            v.data[index] = val;
        }
        v
    }
}
impl<T, N, Type> Neg for CoreVector<T, N, Type>
    where T: Float,
          N: ArrayLength<T>,
          N::ArrayType: Copy,
          CoreVector<T, N, Type>: Copy
{
    type Output = CoreVector<T, N, Type>;
    fn neg(self) -> Self {
        self * -T::one()
    }
}

impl<T, N, Type> Index<usize> for CoreVector<T, N, Type>
    where T: Float,
          N: ArrayLength<T>,
          N::ArrayType: Copy
{
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }
}
impl<T, N, Type> IndexMut<usize> for CoreVector<T, N, Type>
    where T: Float,
          N: ArrayLength<T>,
          N::ArrayType: Copy
{
    fn index_mut(&mut self, idx: usize) -> &mut T {
        &mut self.data[idx]
    }
}
use std::ops::Deref;

impl<T, N, Type> Deref for CoreVector<T, N, Type>
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
impl<T, N, Type> FromIterator<T> for CoreVector<T, N, Type>
    where T: Float,
          N: ArrayLength<T>,
          N::ArrayType: Copy,
          CoreVector<T, N, Type>: Copy
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        unsafe {
            let mut data: GenericArray<T, N> = mem::uninitialized();
            let mut index = 0;
            for val in iter {
                data[index] = val;
                index += 1;
            }
            CoreVector::from_slice(&data)
        }
    }
}

macro_rules! impl_vec_accessor{
    ($dim: ident, $(( $access: ident, $index: expr ) ),*) => {
        impl<T, Type> CoreVector<T, $dim, Type>
            where T: Float,
        {
            $(
                pub fn $access(&self) -> T {
                    self.data[$index]
                }
            )*
        }
    };
 }
macro_rules! impl_vec_accessor_mut{
    ($dim: ident, $(( $access: ident, $index: expr ) ),*) => {
        impl<T, Type> CoreVector<T, $dim, Type>
            where T: Float
        {
            $(
                pub fn $access(&mut self) -> &mut T {
                    &mut self.data[$index]
                }
            )*
        }
    };
 }

impl_vec_accessor_mut!(U2, (x_m, 0), (y_m, 1));
impl_vec_accessor_mut!(U3, (x_m, 0), (y_m, 1), (z_m, 2));
impl_vec_accessor_mut!(U4, (x_m, 0), (y_m, 1), (z_m, 2), (w_m, 3));

impl_vec_accessor!(U2, (x, 0), (y, 1));
impl_vec_accessor!(U3, (x, 0), (y, 1), (z, 2));
impl_vec_accessor!(U4, (x, 0), (y, 1), (z, 2), (w, 3));

macro_rules! impl_vec_new{
    ($dim: ident, $( $x: ident),*) => {
        impl<T, Type> CoreVector<T, $dim, Type>
            where T: Float + Zero,
                  CoreVector<T, $dim, Type>: Copy
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
       // let v1 = Vec2f::from_slice(&[1., -1.]);
       // let v2 = Vec2f::from_slice(&[1., 1.]);
       // let v3 = Vec2f::new(1., 2.);
       // let n = Vec2f::from_slice(&[0., 1.]);
       // assert!(v1 + n * 2. == v2);
   }

   #[test]
   fn lerp_vector() {
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

   #[test]
   fn extend() {
       let v = Vec2f::new(1., 1.);
       let v2 = v.extend(1.);
       assert!(v2 == Vec3f::new( 1., 1., 1. ));
   }
}
