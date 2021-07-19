use std::ops::{Add, Sub, Mul, Div};
use super::ray::Ray;

#[derive(Clone,Copy,Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
	(self.0 - other.0).abs() < f64::EPSILON &&
	    (self.1 - other.1).abs() < f64::EPSILON &&
	    (self.2 - other.2).abs() < f64::EPSILON
    }
}

pub type Point3 = Vec3;

impl Vec3 {
    fn add(&self, rhs: Vec3) -> Vec3 {
	Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }

    fn sub(&self, rhs: Vec3) -> Vec3 {
	self.add(rhs.mul(-1.0))
    }

    fn mul(&self, rhs: f64) -> Vec3 {
	Vec3(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }

    fn div(&self, rhs: f64) -> Vec3 {
	Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }

    pub fn dot(&self, rhs: Vec3) -> f64 {
	self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: Vec3) -> Vec3 {
	Vec3(
	    self.1 * rhs.2 - self.2 * rhs.1,
	    self.2 * rhs.0 - self.0 * rhs.2,
	    self.0 * rhs.1 - self.1 * rhs.0
	)
    }

    pub fn len(&self) -> f64 {
	(self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    pub fn norm(&self) -> Vec3 {
	self.div(self.len())
    }

    pub fn rotate(&self, ax: f64, ay: f64, az: f64) -> Vec3 {
	let (xcos, ycos, zcos) = (ax.cos(), ay.cos(), az.cos());
	let (xsin, ysin, zsin) = (ax.sin(), ay.sin(), az.sin());
	let rx = Vec3(zcos*ycos, zcos*ysin*xsin - zsin*xcos, zcos*ysin*xcos + zsin*xsin);
	let ry = Vec3(zsin*ycos, zsin*ysin*xsin + zcos*xcos, zsin*ysin*xcos - zcos*xsin);
	let rz = Vec3(-ysin, ycos*xsin, ycos*xcos);
	Vec3(self.dot(rx), self.dot(ry), self.dot(rz))
    }

    pub fn rotate_by_point(&self, ax: f64, ay: f64, az: f64, p: Point3) -> Vec3 {
	let begin = p;
	let end = self.add(p);
	end.rotate(ax, ay, az) - begin.rotate(ax, ay, az)
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
	Vec3::add(&self, rhs)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
	Vec3::sub(&self, rhs)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
	Vec3::mul(&self, rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
	Vec3::mul(&rhs, self)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
	Vec3::div(&self, rhs)
    }
}

pub trait Distance<Other> {
    fn distance(&self, other: Other) -> f64;
}

impl Distance<Point3> for Ray {
    fn distance(&self, other: Point3) -> f64 {
        (self.point - other).len()
    }
}

impl Distance<Point3> for Point3 {
    fn distance(&self, other: Point3) -> f64 {
	self.sub(other).len()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    
    #[test]
    fn primitives_sum_scalar() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	let sum1 = x + y + z;
	let sum2 = x + y * 2. + z * 3.;
	let sum3 = x - y * 2. + z * 3.;
	assert_eq!(sum1, Vec3(1., 1., 1.));
	assert_eq!(sum2, Vec3(1., 2., 3.));
	assert_eq!(sum3, Vec3(1., -2., 3.));
    }

    #[test]
    fn primitives_dot_product() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	let a = x * 2. + y * 3. + z * 4.;
	let b = x + y + z;

	let zero = Vec3(0., 0., 0.);
	assert_eq!(a.dot(b), 2. + 3. + 4.);
	assert_eq!(zero.dot(b), 0.);
	assert_eq!(zero.dot(a), 0.);
    }

    #[test]
    fn primitives_cross_product() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	assert_eq!(z.cross(x), y);
	assert_eq!(y.cross(z), x);
	assert_eq!(x.cross(y), z);

	let a = x * 3. + y * 3.;
	let b = x * -3. + y * 3.;

	assert_eq!(a.cross(b).norm(), z);
    }

    #[test]
    fn primitives_rotation() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	assert_eq!(x.rotate(PI, 0., 0.), x);
	assert_eq!(y.rotate(0., PI, 0.), y);
	assert_eq!(z.rotate(0., 0., PI), z);

	assert_eq!(x.rotate(0., - PI / 2., 0.), z);
	assert_eq!(x.rotate(0., 0., PI / 2.), y);
	
	assert_eq!(y.rotate(PI / 2., 0., 0.), z);
	assert_eq!(y.rotate(0., 0., - PI / 2.), x);

	assert_eq!(z.rotate(-PI / 2., 0., 0.), y);
	assert_eq!(z.rotate(0., PI / 2., 0.), x);

	assert_eq!(z.rotate_by_point(PI / 2., 0., 0., y), -1. * y);
    }
}
