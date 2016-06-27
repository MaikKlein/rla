#![allow(dead_code)]
use vector::Vec3;
use num::Float;
pub struct Aabb<T>
    where T: Float
{
    min: Vec3<T>,
    max: Vec3<T>,
}

impl<T> Aabb<T>
    where T: Float
{
    pub fn new(min: Vec3<T>, max: Vec3<T>) -> Self {
        Aabb {
            min: min,
            max: max,
        }
    }

    pub fn single(single_point: Vec3<T>) -> Self {
        Aabb::new(single_point, single_point)
    }

    pub fn union_point(&self, p: Vec3<T>) -> Self {
        let min_v = self.min.min(p);
        let max_v = self.max.max(p);
        Aabb::new(min_v, max_v)
    }

    pub fn union_aabb(first: Self, second: Self) -> Self {
        let min_v = first.min.min(second.min);
        let max_v = first.max.max(second.max);
        Aabb::new(min_v, max_v)
    }

    pub fn diagonale(&self) -> Vec3<T> {
        self.max - self.min
    }

    pub fn overlap(&self, other: Self) -> bool {
        let is_max = Iterator::zip(self.max.into_iter(), other.min.into_iter())
            .fold(true, |acc, (max, min)| acc && max >= min);

        let is_min = Iterator::zip(self.min.into_iter(), other.max.into_iter())
            .fold(true, |acc, (min, max)| acc && max >= min);

        is_max && is_min
    }

    pub fn inside(&self, point: Vec3<T>) -> bool {
        let is_max = Iterator::zip(self.max.into_iter(), point.into_iter())
            .fold(true, |acc, (max, p)| acc && max >= p);
        let is_min = Iterator::zip(self.min.into_iter(), point.into_iter())
            .fold(true, |acc, (min, p)| acc && min <= p);

        is_max && is_min
    }

    /// Returns the position of a point relative to the corners of the box, where a point
    /// at the minimum corner has offset (0, 0, 0), a point at the maximum corner has offset
    /// (1, 1, 1)
    pub fn offset(&self, point: Vec3<T>) -> Vec3<T> {
        (point - self.min) / self.diagonale()
    }
}

#[cfg(test)]
mod test {
    use vector::*;
    use aabb::*;

    #[test]
    fn union() {
        let aabb = Aabb::new(Vec3f::new(-1., -1., -1.), Vec3f::new(1., 1., 1.));
        let aabb1 = Aabb::new(Vec3f::new(-2., -2., -2.), Vec3f::new(2., 2., 2.));
        let aabb2 = Aabb::union_aabb(aabb, aabb1);
    }

    #[test]
    fn offset() {
        let aabb = Aabb::new(Vec3f::new(-2., -2., -2.), Vec3f::new(2., 2., 2.));
        assert!(aabb.offset(Vec3::new(-2., -2., -2.)) == Vec3::new(0., 0., 0.));
        assert!(aabb.offset(Vec3::new(2., 2., 2.)) == Vec3::new(1., 1., 1.));
    }

    #[test]
    fn inside() {
        let aabb = Aabb::new(Vec3f::new(-2., -2., -2.), Vec3f::new(2., 2., 2.));
        assert!(aabb.inside(Vec3f::new(0., 0., 0.)));
        assert!(aabb.inside(Vec3f::new(2., 2., 2.)));
        assert!(aabb.inside(Vec3f::new(-2., -2., -2.)));
        assert!(!aabb.inside(Vec3f::new(-4., 2., -4.)));
        assert!(!aabb.inside(Vec3f::new(-1., 2., -4.)));
    }

    #[test]
    fn overlap() {
        let aabb = Aabb::new(Vec3f::new(-2., -2., -2.), Vec3f::new(2., 2., 2.));
        assert!(aabb.overlap(Aabb::new(Vec3f::new(0., 0., 0.), Vec3f::new(2., 1., 1.))));
        assert!(aabb.overlap(Aabb::new(Vec3f::new(0., 0., 0.), Vec3f::new(5., 5., 5.))));
        assert!(!aabb.overlap(Aabb::new(Vec3f::new(3., 3., 3.), Vec3f::new(5., 5., 5.))));
        assert!(!aabb.overlap(Aabb::new(Vec3f::new(-4., -4., -4.), Vec3f::new(-3., -3., -3.))));
    }
}
