use vector::*;
use num::{Num, Float, Zero, NumCast};

#[derive(Matrix, Copy, Clone, Debug)]
#[repr(C)]
pub struct Mat2<T> {
    pub r0: Vec2<T>,
    pub r1: Vec2<T>,
}

#[derive(Matrix, Copy, Clone, Debug)]
#[repr(C)]
pub struct Mat3<T> {
    pub r0: Vec3<T>,
    pub r1: Vec3<T>,
    pub r2: Vec3<T>,
}

#[derive(Matrix, Copy, Clone, Debug)]
#[repr(C)]
pub struct Mat4<T> {
    pub r0: Vec4<T>,
    pub r1: Vec4<T>,
    pub r2: Vec4<T>,
    pub r3: Vec4<T>,
}


impl<T> Mat2<T>
    where T: Float
{
    pub fn rot(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Mat2::from_rows(Vec2::new(c, -s), Vec2::new(s, c))
    }
}

impl<T> Mat4<T>
    where T: Float
{
    pub fn scale(v: Vec3<T>) -> Self{
        let mut m = Mat4::identity();
        m[0][0] = v.x;
        m[1][1] = v.y;
        m[2][2] = v.z;
        m
    }

    pub fn translate(v: Vec3<T>) -> Self {
        Mat4::from_rows(Vec4::new(T::zero(), T::zero(), T::zero(), v.x),
                        Vec4::new(T::zero(), T::zero(), T::zero(), v.y),
                        Vec4::new(T::zero(), T::zero(), T::zero(), v.z),
                        Vec4::new(T::zero(), T::zero(), T::zero(), T::one()))
    }

    pub fn perspective_vk_revz(fov: T, ar: T, near: T, far: T) -> Self {
        let half_fov = fov / (T::one() + T::one());
        Mat4::from_rows(Vec4::new(T::one() / (ar * half_fov.tan()),
                                  T::zero(),
                                  T::zero(),
                                  T::zero()),
                        Vec4::new(T::zero(),
                                  -T::one() / (half_fov.tan()),
                                  T::zero(),
                                  T::zero()),
                        Vec4::new(T::zero(),
                                  T::zero(),
                                  -near / (far - near),
                                  (near * far) / (far - near)),
                        Vec4::new(T::zero(), T::zero(), T::one(), T::zero()))
    }

    pub fn perspective_vk(fov: T, ar: T, near: T, far: T) -> Self {
        let half_fov = fov / (T::one() + T::one());
        Mat4::from_rows(Vec4::new(T::one() / (ar * half_fov.tan()),
                                  T::zero(),
                                  T::zero(),
                                  T::zero()),
                        Vec4::new(T::zero(),
                                  -T::one() / (half_fov.tan()),
                                  T::zero(),
                                  T::zero()),
                        Vec4::new(T::zero(),
                                  T::zero(),
                                  -far / (near - far),
                                  (near * far) / (near - far)),
                        Vec4::new(T::zero(), T::zero(), T::one(), T::zero()))
    }

    pub fn look_at(eye: Vec3<T>, center: Vec3<T>, up: Vec3<T>) -> Self {
        let z = Vec3::normalize(center - eye).unwrap();
        let x = Vec3::cross(z, up).normalize().unwrap();
        let y = Vec3::cross(x, z).normalize().unwrap();
        Mat4::from_rows(x.extend(-x.dot(eye)),
                        y.extend(-y.dot(eye)),
                        z.extend(-z.dot(eye) * -T::one()),
                        Vec4::new(T::zero(), T::zero(), T::zero(), T::one()))
    }

    pub fn rotation_x(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::from_rows(Vec4::<T>::new(T::one(), T::zero(), T::zero(), T::zero()),
                        Vec4::<T>::new(T::zero(), c, -s, T::zero()),
                        Vec4::<T>::new(T::zero(), s, c, T::zero()),
                        Vec4::<T>::new(T::zero(), T::zero(), T::zero(), T::one()))
    }

    pub fn rotation_y(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::from_rows(Vec4::<T>::new(c, T::zero(), s, T::zero()),
                        Vec4::<T>::new(T::zero(), T::one(), T::zero(), T::zero()),
                        Vec4::<T>::new(-s, T::zero(), c, T::zero()),
                        Vec4::<T>::new(T::zero(), T::zero(), T::zero(), T::one()))
    }

    pub fn rotation_z(angle: T) -> Self {
        let s = angle.sin();
        let c = angle.cos();
        Self::from_rows(Vec4::<T>::new(c, -s, T::zero(), T::zero()),
                        Vec4::<T>::new(s, c, T::zero(), T::zero()),
                        Vec4::<T>::new(T::zero(), T::zero(), T::one(), T::zero()),
                        Vec4::<T>::new(T::zero(), T::zero(), T::zero(), T::one()))
    }
}
