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
