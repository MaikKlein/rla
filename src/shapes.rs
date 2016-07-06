
use num::Float;
use transform::*;
use std::rc::Rc;

fn quadratic<T: Float>(a: T, b: T, c: T) -> Option<(T, T)> {
    use num::NumCast;
    let four: T = NumCast::from(4).unwrap();
    let two: T = NumCast::from(2).unwrap();
    let d: T = b.powi(2) - four * a * c;
    let two_a = two * a;
    if d <= T::zero() || two_a <= T::zero() {
        return None;
    }

    let d1 = d / two_a;
    let b1 = -b / two_a;

    Some((b1 - d1, b1 + d1))

}
use ray::Ray;
trait Shape<T>
    where T: Float
{
    fn intersect_p(&self, ray: Ray<T>) -> Option<T>;
}
struct Sphere<T>
    where T: Float
{
    object_to_world: Rc<Transform<T>>,
    world_to_object: Rc<Transform<T>>,
    radius: T,
}

impl<T> Shape<T> for Sphere<T>
    where T: Float
{
    fn intersect_p(&self, ray: Ray<T>) -> Option<T> {
        use num::NumCast;
        let local_ray = self.world_to_object.mul_ray(ray);
        let a = local_ray.direction.length_sq();
        let b: T = (local_ray.direction * local_ray.origin)
            .into_iter()
            .fold(T::zero(), |acc, val| acc + val) * NumCast::from(2).unwrap();
        let c = local_ray.origin.length_sq() - self.radius;

        if let Some((t0, t1)) = quadratic(a, b, c) {
            return match t0.max(t1) {
                i if i < local_ray.max_time => Some(i),
                _ => None,
            };
        }
        None
    }
}
