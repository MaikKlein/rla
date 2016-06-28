#![allow(dead_code)]
use vector::Vec3;
use num::Float;
use num::FromPrimitive;

#[derive(Eq, PartialEq)]
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

    pub fn expand(&self, distance: T) -> Self {
        let dist_vec = Vec3::new(distance, distance, distance);
        let min = self.min - dist_vec;
        let max = self.max + dist_vec;
        Aabb::new(min, max)
    }

    pub fn surface_area(&self) -> T {
        use num::NumCast;
        let d = self.diagonale();
        (d.x() * d.y() + d.y() * d.z() + d.x() * d.z()) * NumCast::from(2).unwrap()
    }

    pub fn volume(&self) -> T {
        self.diagonale().into_iter().fold(T::one(), |acc, val| acc * val)
    }

    pub fn maximum_exent(&self) -> u8 {
        let d = self.diagonale();
        if d.x() > d.y() && d.x() > d.z() {
            0
        } else if d.y() > d.z() {
            1
        } else {
            2
        }
    }

    fn bounding_sphere(&self) -> Sphere<T>
        where T: FromPrimitive
    {
        let v: T = FromPrimitive::from_f32(0.5).unwrap();
        let center: Vec3<T> = (self.min + self.max) * v;
        Sphere::new(center, center.distance(self.max))
    }
}

#[derive(Eq, PartialEq, Debug)]
pub struct Sphere<T>
    where T: Float
{
    center: Vec3<T>,
    radius: T,
}
impl<T: Float> Sphere<T> {
    fn new(center: Vec3<T>, radius: T) -> Self {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

#[cfg(test)]
mod test {
    use vector::*;
    use aabb::*;

    #[test]
    fn union() {
        let aabb = Aabb::new(Vec3f::new(-1., -1., -1.), Vec3f::new(3., 3., 3.));
        let aabb1 = Aabb::new(Vec3f::new(-2., -2., -2.), Vec3f::new(2., 2., 2.));
        let aabb2 = Aabb::union_aabb(aabb, aabb1);
        assert!(aabb2 == Aabb::new(Vec3f::new(-2., -2., -2.), Vec3f::new(3., 3., 3.)));
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

    #[test]
    fn volume() {
        let aabb = Aabb::new(Vec3f::new(0., 0., 0.), Vec3f::new(1., 1., 1.));
        assert!(aabb.volume() == 1.);
        let aabb1 = Aabb::new(Vec3f::new(0., 0., 0.), Vec3f::new(2., 2., 2.));
        assert!(aabb1.volume() == 8.);
    }

    #[test]
    fn bounding_sphere() {
        let aabb = Aabb::new(Vec3f::new(0., 0., 0.), Vec3f::new(1., 1., 1.));
        let sphere = aabb.bounding_sphere();
        assert!(sphere ==
                Sphere::new(Vec3f::new(0.5, 0.5, 0.5),
                            Vec3f::new(0.5, 0.5, 0.5).length()));
    }
}
