use num::Float;
use vector::Vec3;
use unit::*;
use std::ops::{Mul, Div};
#[derive(Copy, Clone)]
pub struct Quaternion<T>
    where T: Float
{
    v: Vec3<T>,
    w: T,
}

impl<T> Quaternion<T>
    where T: Float
{
    pub fn new<R: ToRadians<T>>(axis: Vec3<T>, angle: R) -> Self {
        use num::NumCast;
        let half_angle = angle.value() / NumCast::from(2.0).unwrap();
        let w = half_angle.cos();
        let normal = axis.normalize().expect("Non zero vector");
        Quaternion {
            v: normal * half_angle.sin(),
            w: w,
        }
    }

    pub fn raw(v: Vec3<T>, w: T) -> Self {
        Quaternion { v: v, w: w }
    }

    pub fn conjugate(self) -> Self {
        Self::raw(self.v, -self.w)
    }

    pub fn inverse(self) -> Self {
        self.conjugate() / self.length_sq()
    }

    pub fn length_sq(self) -> T {
        self.dot(self)
    }

    pub fn length(self) -> T {
        self.length_sq().sqrt()
    }

    pub fn dot(self, other: Self) -> T {
        self.w * other.w + Vec3::dot(self.v, other.v)
    }
}

impl<T> Mul for Quaternion<T>
    where T: Float
{
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let w = self.w * other.w - Vec3::dot(self.v, other.v);
        let v = self.v * other.w + other.v * self.w + Vec3::cross(self.v, other.v);
        Self::raw(v, w)
    }
}

impl<T> Div<T> for Quaternion<T>
    where T: Float
{
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self::raw(self.v / other, self.w / other)
    }
}
impl<T> Mul<Vec3<T>> for Quaternion<T>
    where T: Float
{
    type Output = Vec3<T>;
    fn mul(self, other: Vec3<T>) -> Vec3<T> {
        // use num::NumCast;
        // let vcv = Vec3::cross(self.v, other);
        // let two: T = NumCast::from(2.0).unwrap();
        // other + vcv * (two * self.w) + Vec3::cross(self.v, vcv) * two
        let vq = Self::raw(other, T::zero());
        // q * v * q^(-1)
        let r = self * vq * self.inverse();
        r.v
    }
}
#[cfg(test)]
mod test {
    use vector::*;
    use quaternion::*;
    use unit::*;
    #[test]
    fn inverse() {
        let q = Quaternion::new(Vec3f::new(0., 1., 0.), Radians::new(3.14 / 2.));
        let v = Vec3f::new(1., 0., 0.);
        let v2 = q * v;
        println!("Inverse {:?}", v2);
        println!("Rad {}", Degrees::new(180.).radians().value);
    }
}
