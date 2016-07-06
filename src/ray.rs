#![allow(dead_code)]

use vector::Vec3;
use num::Float;
pub struct Ray<T>
    where T: Float
{
    pub origin: Vec3<T>,
    pub direction: Vec3<T>,
    pub max_time: T
}

impl<T> Ray<T>
    where T: Float
{
    pub fn new(o: Vec3<T>, d: Vec3<T>, max_time: T) -> Self {
        Ray {
            origin: o,
            direction: d,
            max_time: max_time
        }
    }
    pub fn from_start_end(start: Vec3<T>, end: Vec3<T>) -> Self {
        let dir = end - start;
        let max_time = dir.length();
        let dir_normal = dir.normalize().expect("Should not be zero");
        Ray::new(start, dir_normal, max_time)
    }
}


pub struct RayHit<T>
    where T: Float
{
    ray: Ray<T>,
    time: T
}


