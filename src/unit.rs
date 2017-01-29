use num::{Float, NumCast};

pub trait ToDegrees<T: Float> {
    fn degrees(&self) -> Degrees<T>;
    fn value(&self) -> T;
}

pub trait ToRadians<T: Float> {
    fn radians(&self) -> Radians<T>;
    fn value(&self) -> T;
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Radians<T>
    where T: Float
{
    pub value: T,
}

impl<T> Radians<T>
    where T: Float
{
    pub fn new(value: T) -> Self {
        Radians { value: value }
    }
}

#[derive(Debug, Clone, Copy, Hash)]
pub struct Degrees<T>
    where T: Float
{
    pub value: T,
}

impl<T> Degrees<T>
    where T: Float
{
    pub fn new(value: T) -> Self {
        Degrees { value: value }
    }
}

impl<T> ToDegrees<T> for Degrees<T>
    where T: Float
{
    fn degrees(&self) -> Self {
        Self::new(self.value)
    }

    fn value(&self) -> T {
        self.value
    }
}

impl<T> ToDegrees<T> for Radians<T>
    where T: Float
{
    fn degrees(&self) -> Degrees<T> {
        use std::f64;
        Degrees::new(self.value * NumCast::from(180.0f64 / f64::consts::PI).unwrap())
    }

    fn value(&self) -> T {
        self.degrees().value
    }
}

impl<T> ToRadians<T> for Degrees<T>
    where T: Float
{
    fn radians(&self) -> Radians<T> {
        use std::f64;
        Radians::new(self.value * NumCast::from(f64::consts::PI / 180.0f64).unwrap())
    }

    fn value(&self) -> T {
        self.radians().value
    }
}

impl<T> ToRadians<T> for Radians<T>
    where T: Float
{
    fn radians(&self) -> Radians<T> {
        Radians::new(self.value)
    }

    fn value(&self) -> T {
        self.value
    }
}
