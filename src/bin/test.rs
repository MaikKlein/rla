#![allow(dead_code)]
#[macro_use]
extern crate rla;
extern crate cgmath;
use rla::*;
use std::time::*;
use cgmath::SquareMatrix;

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
    //let r = Degrees::new(90.0);
    //println!("{:?}", r.radians());
    //let m = Mat4::look_at(Vec3::new(0.0, 0.0, -5.0),
    //                      Vec3::new(0.0, 0.0, 0.0),
    //                      Vec3::new(0.0, 1.0, 0.0));
    //let angle: f32 = "10.0".parse().unwrap();
    //let m1 = Mat4::rotation_y(angle);
    //let m3 = m * &m1;
    //println!("{:?}", m3);
    let length: usize = 10000;

    let v1: Vec<Mat4<f32>> = (0..length)
        .map(|i| i as f32)
        .map(|i| {
            let v = Vec4::new(i, i, i, i);
            Mat4::from_rows(v, v, v, v)
        })
        .collect();

    let v2: Vec<cgmath::Matrix4<f32>> = (0..length)
        .map(|i| i as f32)
        .map(|i| {
            let v = cgmath::Vector4::new(i, i, i, i);
            cgmath::Matrix4::from_cols(v, v, v, v)
        })
        .collect();

    let times = 100;

    let v_cg = cgmath::Vector4::new(1.0, 2.0, 3.0, 4.0);
    let v_my = Vec4::new(1.0, 2.0, 3.0, 4.0);
    let take = 100000000;

    let t1 = SystemTime::now();
    let m1 = v1.iter().cycle().take(take).fold(Mat4::identity(), |acc, m| acc * m);
    println!("{:?}", t1.elapsed());

    let t2 = SystemTime::now();
    let m2 = v2.iter()
        .cycle()
        .take(take)
        .fold(cgmath::Matrix4::identity(), |acc, m| acc * m);
    println!("{:?}", t2.elapsed());



    println!("{:?}", m1);
    println!("{:?}", m2);
    //   println!("{:?}", t1.elapsed());
    //
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
