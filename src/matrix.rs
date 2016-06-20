use num::{Float, Zero};
use typenum::*;
use generic_array::*;
use std::ops::{Add, Sub, Mul};
use std::iter::*;
use std::sync::mpsc::*;
use std::mem;
use num::Num;

use vector::*;
#[derive(PartialEq, Eq, Debug)]
struct Matrix<T: Float, N, M>
    where N: ArrayLength<T>,
          M: ArrayLength<Vector<T, N>>,
          N::ArrayType: Copy,
          M::ArrayType: Copy
{
    data: GenericArray<Vector<T, N>, M>,
}

impl<T: Float, N, M> Matrix<T, N, M>
    where N: ArrayLength<T>,
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
impl<T: Float, N> Matrix<T, N, N>
    where N: ArrayLength<T> + ArrayLength<Vector<T, N>>,
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

impl<T: Float, N, M> Matrix<T, N, M>
    where N: ArrayLength<T> + ArrayLength<Vector<T, M>>,
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
impl<T: Float, N, M> Matrix<T, N, M>
    where N: ArrayLength<T> + ArrayLength<Vector<T, M>>,
          M: ArrayLength<T> + ArrayLength<Vector<T, N>>,
          <N as ArrayLength<Vector<T, M>>>::ArrayType: Copy,
          <M as ArrayLength<Vector<T, N>>>::ArrayType: Copy,
          <N as ArrayLength<T>>::ArrayType: Copy,
          <M as ArrayLength<T>>::ArrayType: Copy
{
    fn mul<N1>(&self, other: &Matrix<T, N1, N>) -> Matrix<T, M, N1>
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
            for j in 0..M::to_usize() {
                for i in 0..N1::to_usize() {
                    new_matrix.data[j].data[i] = self.data[j].dot(other_transposed.data[i]);
                }
            }
            new_matrix
        }
    }
}

#[test]
fn mul() {

    use vector::*;
    let v2 = Vec2f::new(&[1., 2.]);
    let v3 = Vec3f::new(&[1., 2., 3.]);
    let m1 = Mat3x2f::new(&[v3, v3]);
    let m2 = Mat2x3f::new(&[v2, v2, v2]);

    let m3: Mat2x2f = m1.mul(&m2);
    let m4 = m3.mul(&m3);
    println!("{:?}", m3);
}
#[test]
fn matrix_identity() {
    use vector::*;
    let m = Mat2x2f::identity();
}

type Mat4x4<T> = Matrix<T, U4, U4>;
type Mat3x3<T> = Matrix<T, U3, U3>;
type Mat3x2<T> = Matrix<T, U3, U2>;
type Mat2x3<T> = Matrix<T, U2, U3>;
type Mat2x2<T> = Matrix<T, U2, U2>;

type Mat3x2f = Mat3x2<f32>;
type Mat2x3f = Mat2x3<f32>;
type Mat2x2f = Mat2x2<f32>;
