use num::{Float, Zero};
use typenum::*;
use generic_array::*;
use std::ops::{Add, Sub, Mul, Div};
use std::mem;
#[derive(Copy, Debug)]
pub struct Vector<T: Float, N: ArrayLength<T>>
    where N::ArrayType: Copy
{
    pub data: GenericArray<T, N>,
}

impl<T, N> Clone for Vector<T, N>
    where N: ArrayLength<T>,
          N::ArrayType: Copy,
          T: Float
{
    fn clone(&self) -> Self {
        Vector::<T, N> { data: self.data.clone() }
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
    pub fn new(slice: &[T]) -> Vector<T, N> {
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

    pub fn max(self) -> T {
        self.data.iter().fold(self.data[0].clone(), |acc, &x| {
            if x > acc {
                x
            } else {
                acc
            }
        })
    }

    pub fn dot(self, other: Self) -> T {
        return self.data
            .iter()
            .zip(other.data.iter())
            .fold(T::zero(), |acc, (x, y)| acc + *x * *y);
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

    pub fn reflect(self, other: Self) -> Self {
        self - other * other.dot(self) * (T::one() + T::one())
    }

    pub fn distance_sq(self, other: Self) -> T {
        (self - other).length_sq()
    }

    pub fn distance(self, other: Self) -> T {
        self.distance(other).sqrt()
    }
}

#[test]
fn mul_v() {
    let v = Vec2f::new(&[1.0, 2.0]);
    let v1 = v * 1.0;
}

pub type Vec4<T> = Vector<T, U4>;
pub type Vec3<T> = Vector<T, U3>;
pub type Vec2<T> = Vector<T, U2>;

pub type Vec4f = Vec4<f32>;
pub type Vec3f = Vec3<f32>;
pub type Vec2f = Vec2<f32>;
