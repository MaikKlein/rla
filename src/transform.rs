#![allow(dead_code)]
use matrix::*;
use num::Float;
use vector::*;
pub struct Transform<T>
    where T: Float
{
    m: Mat4x4<T>,
    inverse: Mat4x4<T>,
}
impl<T> Transform<T>
    where T: Float
{
    pub fn translate(v: Vec3<T>) -> Self {
        let m = Mat4x4::<T>::translate(v);
        let m_inv = Mat4x4::<T>::translate(-v);
        Transform {
            m: m,
            inverse: m_inv,
        }
    }

    pub fn rotation_x(angle: T) -> Self {
        let m = Mat4x4::<T>::rotation_x(angle);
        let m_inv = Mat4x4::<T>::rotation_x(-angle);
        Transform {
            m: m,
            inverse: m_inv,
        }
    }

    pub fn rotation_y(angle: T) -> Self {
        let m = Mat4x4::<T>::rotation_y(angle);
        let m_inv = Mat4x4::<T>::rotation_y(-angle);
        Transform {
            m: m,
            inverse: m_inv,
        }
    }

    pub fn rotation_z(angle: T) -> Self {
        let m = Mat4x4::<T>::rotation_z(angle);
        let m_inv = Mat4x4::<T>::rotation_z(-angle);
        Transform {
            m: m,
            inverse: m_inv,
        }
    }

    pub fn scale(s: Vec3<T>) -> Self {
        let m = Mat4x4::<T>::scale(s);
        let s_inv = s.map(|val| T::one() / val);
        let m_inv = Mat4x4::<T>::scale(s_inv);
        Transform {
            m: m,
            inverse: m_inv,
        }
    }

    pub fn mul(&self, p: Vec3<T>) -> Vec3<T> {
        let new_p = self.m.mul_v(p.extend(T::one()));
        new_p.truncate() / new_p.w()
    }

    pub fn look_at(pos: Vec3<T>, target: Vec3<T>, up: Vec3<T>) -> Self {
        let z = (pos - target).normalize().unwrap();
        let x = up.cross(z).normalize().unwrap();
        let y = z.cross(x);

        let m = Mat4x4::<T>::new(&[
            x.extend(T::zero()),
            x.extend(T::zero()),
            x.extend(T::zero()),
            Vec4::new(T::one(), T:: zero(),T:: zero(),T:: zero())
        ]);
        let m1 = Mat4x4::<T>::new(&[
            x.extend(T::zero()),
            x.extend(T::zero()),
            x.extend(T::zero()),
            Vec4::new(T::one(), T:: zero(),T:: zero(),T:: zero())
        ]);

        Transform {
            m: m,
            inverse: m1,
        }
    }
}

#[cfg(test)]
mod test {
    use transform::*;
    use matrix::*;
    #[test]
    fn trans() {
        use std::mem::size_of;
        println!("{}", size_of::<Transform<f32>>());
        println!("{}", size_of::<Mat4x4<f32>>());
    }
}
