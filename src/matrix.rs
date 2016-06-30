#![allow(dead_code)]
use num::Float;
use typenum::*;
use generic_array::*;
use std::ops;
use std::ops::{Index, IndexMut, Mul};
use std::mem;

use vector::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy
{
    data: GenericArray<Vector<T, N>, M>,
}

impl<T, N, M> Mul<T> for Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy,
          Vector<T, N>: Copy
{
    type Output = Matrix<T, N, M>;
    fn mul(self, scalar: T) -> Self::Output {
        let mut m = Self::zero();
        for (index, new_val) in self.data.into_iter().map(|v| v * scalar).enumerate() {
            m.data[index] = new_val;
        }
        m
    }
}
impl<T, N, M> Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy,
          Vector<T, N>: Copy
{
    pub fn new(slice: &[Vector<T, N>]) -> Matrix<T, N, M> {
        Matrix::<T, N, M> { data: GenericArray::from_slice(slice) }
    }

    pub fn zero() -> Matrix<T, N, M> {
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

impl<T, N, M> Index<usize> for Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy
{
    type Output = Vector<T, N>;

    fn index(&self, idx: usize) -> &Vector<T, N> {
        &self.data[idx]
    }
}
impl<T, N, M> IndexMut<usize> for Matrix<T, N, M>
    where T: Float,
          N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy
{
    fn index_mut(&mut self, idx: usize) -> &mut Vector<T, N> {
        &mut self.data[idx]
    }
}

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
        Self::new(&[Vec4::<T>::new(T::one(),  T::zero(),  T::zero(), T::zero()),
                    Vec4::<T>::new(T::zero(), c,         -s,         T::zero()),
                    Vec4::<T>::new(T::zero(), s,          c,         T::zero()),
                    Vec4::<T>::new(T::zero(), T::zero(),  T::zero(), T::one())])
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

    pub fn inverse(&self) -> Self {
        let coef00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
        let coef02 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let coef03 = self[1][2] * self[2][3] - self[2][2] * self[1][3];

        let coef04 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
        let coef06 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
        let coef07 = self[1][1] * self[2][3] - self[2][1] * self[1][3];

        let coef08 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
        let coef10 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
        let coef11 = self[1][1] * self[2][2] - self[2][1] * self[1][2];

        let coef12 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
        let coef14 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
        let coef15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];

        let coef16 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
        let coef18 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
        let coef19 = self[1][0] * self[2][2] - self[2][0] * self[1][2];

        let coef20 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let coef22 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let coef23 = self[1][0] * self[2][1] - self[2][0] * self[1][1];

        let fac0 = Vec4::new(coef00, coef00, coef02, coef03);
        let fac1 = Vec4::new(coef04, coef04, coef06, coef07);
        let fac2 = Vec4::new(coef08, coef08, coef10, coef11);
        let fac3 = Vec4::new(coef12, coef12, coef14, coef15);
        let fac4 = Vec4::new(coef16, coef16, coef18, coef19);
        let fac5 = Vec4::new(coef20, coef20, coef22, coef23);

        let vec0 = Vec4::new(self[1][0], self[0][0], self[0][0], self[0][0]);
        let vec1 = Vec4::new(self[1][1], self[0][1], self[0][1], self[0][1]);
        let vec2 = Vec4::new(self[1][2], self[0][2], self[0][2], self[0][2]);
        let vec3 = Vec4::new(self[1][3], self[0][3], self[0][3], self[0][3]);

        let inv0 = vec1 * fac0 - vec2 * fac1 + vec3 * fac2;
        let inv1 = vec0 * fac0 - vec2 * fac3 + vec3 * fac4;
        let inv2 = vec0 * fac1 - vec1 * fac3 + vec3 * fac5;
        let inv3 = vec0 * fac2 - vec1 * fac4 + vec2 * fac5;

        let sign_a = Vec4::new(T::one(), -T::one(), T::one(), -T::one());
        let sign_b = Vec4::new(-T::one(), T::one(), -T::one(), T::one());

        let inverse = Self::new(&[inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b]);

        let row0 = Vec4::new(inverse[0][0], inverse[1][0], inverse[2][0], inverse[3][0]);

        let dot0 = self[0] * row0;
        let dot1 = (dot0.x() + dot0.y()) + (dot0.z() + dot0.w());

        let one_over_det = T::one() / dot1;
        inverse * one_over_det
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
