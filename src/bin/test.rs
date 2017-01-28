#![allow(dead_code)]
#[macro_use]
extern crate rla;
use rla::*;

pub struct Vec4T {
    v: [f32; 4],
}

impl Vec4T {
    pub fn dot(&self, other: &Self) -> f32 {
        let mut r = self.v[0] * other.v[0];
        for i in 1..4 {
            r += self.v[i] * other.v[i];
        }
        r
    }

    pub fn foo(&self, other: &Self) -> f32 {
        self.v[0] * other.v[0] + self.v[1] * other.v[1] + self.v[2] * other.v[2] +
        self.v[3] * other.v[3]
    }
}

fn main() {
    // let v = Vec2::new(1.0, 1.0).normalize().unwrap();
    // let m1 = Mat2::from_rows(Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0));
    // let m2 = Mat2::from_rows(Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0));
    // let v1 = Mat2::rot(std::f32::consts::PI / 4.0) * &v;

    let v = Vec4::new(1.0, 0.0, 0.0, 0.0);
    let v1 = Vec3::new(1.0, 0.0, 0.0);
    let v2 = Vec3::new(0.0, 0.0, 1.0);
    println!("{:?}", v1.cross(v2));
    println!("{:?}", (Mat4::rotation_y(std::f32::consts::PI / 2.0)) * v);
    println!("{:?}", (Mat4::rotation_y(std::f32::consts::PI / 4.0) * &Mat4::rotation_y(std::f32::consts::PI / 4.0)) * v);
    //    let length = 100000;
    //    let v1: Vec<_> = (0 .. length).map(|i| i as f32).map(|i| Vec4::new(i, i, i, i)).collect();
    //    let v2: Vec<_> = (0 .. length).map(|i| i as f32).map(|i| Vec4T{v: [i, i, i, i]}).collect();
    //
    //    let times = 100;
    //    let t1 = SystemTime::now();
    //    let mut sum = 0.0;
    //    for _ in 0 .. times{
    //        for v in v1.iter().map(|v| v.dot(&v)){
    //            sum += v;
    //        }
    //    }
    //    println!("{:?}", t1.elapsed());
    //    let t2 = SystemTime::now();
    //    for _ in 0 .. times{
    //        for v in v2.iter().map(|v| v.dot(&v)){
    //            sum += v;
    //        }
    //    }
    //    println!("{:?}", t2.elapsed());
    //    println!("{}", sum);
    //
    //
}
