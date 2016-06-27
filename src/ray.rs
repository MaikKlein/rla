#![allow(dead_code)]

use vector::Vec3;
use num::Float;
struct Ray<T>
    where T: Float
{
    origin: Vec3<T>,
    direction: Vec3<T>,
}

impl<T> Ray<T>
    where T: Float
{
    fn new(o: Vec3<T>, d: Vec3<T>) -> Self {
        Ray {
            origin: o,
            direction: d,
        }
    }
}

struct RayIntersection<T>
    where T: Float
{
    max: T,
    min: T,
    ray: Ray<T>,
}
