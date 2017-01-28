use num::{Num, Float, Zero, NumCast};

#[derive(Copy, Clone, Debug, Vector)]
#[repr(C)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Copy, Clone, Debug, Vector)]
#[repr(C)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[derive(Copy, Clone, Debug, Vector)]
#[repr(C)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vec3<T>
    where T: Float
{
    pub fn cross(self, other: Self) -> Self {
        Self::new(self.y * other.z - self.z * other.y,
                  self.z * other.x - self.x * other.z,
                  self.x * other.y - self.y * other.x)
    }
}
