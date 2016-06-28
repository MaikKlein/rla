#![allow(dead_code)]
use num::Float;
use typenum::*;
use generic_array::*;
use std::ops;
use std::mem;

use vector::*;

#[derive(PartialEq, Eq, Debug)]
pub struct Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy
{
    data: GenericArray<Vector<T, N>, M>,
}

impl<T, N, M> Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy,
          Vector<T, N>: Copy
{
    fn new(slice: &[Vector<T, N>]) -> Matrix<T, N, M> {
        Matrix::<T, N, M> { data: GenericArray::from_slice(slice) }
    }

    fn zero() -> Matrix<T, N, M> {
        unsafe {
            let mut mat: Matrix<T, N, M> = mem::uninitialized();
            for j in 0..M::to_usize() {
                mat.data[j] = Vector::<T, N>::zero();
            }
            mat
        }
    }
}
impl<T, N> Matrix<T, N, N>
    where T: Float,
          N: ArrayLength<T> + ArrayLength<Vector<T, N>>,
          <N as ArrayLength<T>>::ArrayType: Copy,
          <N as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
          Vector<T, N>: Copy
{
    fn identity() -> Matrix<T, N, N> {
        let mut mat = Matrix::<T, N, N>::zero();
        for index in 0..N::to_usize() {
            mat.data[index].data[index] = T::one();
        }
        mat
    }
}

impl<T, N, M> Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T> + ArrayLength<Vector<T, M>>,
          M: ArrayLength<T> + ArrayLength<Vector<T, N>>,
          <N as ArrayLength<Vector<T, M>>>::ArrayType: Copy,
          <M as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
          <N as ArrayLength<T>>::ArrayType: Copy,
          <M as ArrayLength<T>>::ArrayType: Copy
{
    fn transpose(&self) -> Matrix<T, M, N> {
        unsafe {
            let mut data: GenericArray<Vector<T, M>, N> = mem::uninitialized();
            for j in 0..N::to_usize() {
                for i in 0..M::to_usize() {
                    data[j].data[i] = self.data[i].data[j];
                }
            }
            Matrix::<T, M, N> { data: data }
        }
    }
}
// funny stuff is happening here
impl<T, N, M> Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T> + ArrayLength<Vector<T, M>>,
          M: ArrayLength<T> + ArrayLength<Vector<T, N>>,
          <N as ArrayLength<Vector<T, M>>>::ArrayType: Copy,
          <M as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
          <N as ArrayLength<T>>::ArrayType: Copy,
          <M as ArrayLength<T>>::ArrayType: Copy
{
    pub fn mul_v(&self, other: Vector<T, N>) -> Vector<T, N>
        where Vector<T, N>: Copy
    {
        let mut v = Vector::<T, N>::zero();
        for index in 0..N::to_usize() {
            v.data[index] = self.data[index].dot(other);
        }
        v
    }
    pub fn mul<N1>(&self, other: &Matrix<T, N1, N>) -> Matrix<T, M, N1>
        where N1: ArrayLength<T> + ArrayLength<Vector<T, M>> + ArrayLength<Vector<T, N>>,
              <N1 as ArrayLength<T>>::ArrayType: Copy,
              <N1 as ArrayLength<Vector<T, M>>>::ArrayType: Copy,
              <N1 as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
              N: ArrayLength<Vector<T, N1>>,
              <N as ArrayLength<Vector<T, N1>>>::ArrayType: Copy,
              Vector<T, N>: Copy
    {
        unsafe {
            let mut new_matrix: Matrix<T, M, N1> = mem::uninitialized();
            let other_transposed: Matrix<T, N, N1> = other.transpose();
            for j in 0..N1::to_usize() {
                for i in 0..M::to_usize() {
                    new_matrix.data[j].data[i] = self.data[j].dot(other_transposed.data[i]);
                }
            }
            new_matrix
        }
    }
}
use asprim::AsPrim;
impl<T> Matrix<T, U4, U4>
    where T: Float
{
    pub fn translate(v: Vec3<T>) -> Self {
        let mut m = Self::identity();
        *m.data[0].w_m() = v.x();
        *m.data[1].w_m() = v.y();
        *m.data[2].w_m() = v.z();
        m
    }

    pub fn scale(s: Vec3<T>) -> Self {
        let mut identity = Self::identity();
        for index in 0..s.dim() {
            identity.data[index].data[index] = s.data[index];
        }
        identity
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotation_x(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::new(&[Vec4::<T>::new(T::one(),  T::zero(), T::zero(), T::zero()),
                    Vec4::<T>::new(T::zero(), c,         -s,        T::zero()),
                    Vec4::<T>::new(T::zero(), s,          c,        T::zero()),
                    Vec4::<T>::new(T::zero(), T::zero(), T::zero(), T::one())])
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotation_y(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::new(&[Vec4::<T>::new(c,         T::zero(), s,         T::zero()),
                    Vec4::<T>::new(T::zero(), T::one(),  T::zero(), T::zero()),
                    Vec4::<T>::new(-s,        T::zero(), c,         T::zero()),
                    Vec4::<T>::new(T::zero(), T::zero(), T::zero(), T::one())])
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    pub fn rotation_z(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::new(&[Vec4::<T>::new(c,        -s,         T::zero(), T::zero()),
                    Vec4::<T>::new(s,         c,         T::zero(), T::zero()),
                    Vec4::<T>::new(T::zero(), T::zero(), T::one(),  T::zero()),
                    Vec4::<T>::new(T::zero(), T::zero(), T::zero(), T::one())])
    }
}

#[cfg(test)]
mod test {
    use vector::*;
    use matrix::*;
    #[test]
    fn rot_x() {
        use std::f32;
        let v = Vec4f::new(0., 0., 1., 0.);
        let pi = f32::consts::PI;
        let m = Mat4x4f::rotation_x(pi / 2.0);
        let v2 = m.mul_v(v);
        println!("{:?}", v2);
    }
}
#[test]
fn mul() {
    // use vector::*;
    // let v2 = Vec2f::new(1., 2.);
    // let v3 = Vec3f::new(1., 2., 3.);
    // let m1 = Mat3x2f::new(&[v3, v3]);
    // let m2 = Mat2x3f::new(&[v2, v2, v2]);

    // let m3: Mat2x2f = m1.mul(&m2);
    // let m4 = m3.mul(&m3);
    // println!("{:?}", m3);
}
#[test]
fn matrix_identity() {
    // use vector::*;
    // let m = Mat2x2f::identity();
}

pub type Mat4x4<T> = Matrix<T, U4, U4>;
pub type Mat3x3<T> = Matrix<T, U3, U3>;
pub type Mat3x2<T> = Matrix<T, U3, U2>;
pub type Mat2x3<T> = Matrix<T, U2, U3>;
pub type Mat2x2<T> = Matrix<T, U2, U2>;
pub type Mat4x1<T> = Matrix<T, U4, U2>;

pub type Mat4x4f = Mat4x4<f32>;
pub type Mat4x1f = Mat4x1<f32>;
pub type Mat3x2f = Mat3x2<f32>;
pub type Mat2x3f = Mat2x3<f32>;
pub type Mat2x2f = Mat2x2<f32>;
