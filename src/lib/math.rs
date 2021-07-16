
use std::ops::{Add, Sub, Mul, Div};

#[derive(Clone,Copy)]
pub struct Vec3(f64, f64, f64);

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

    fn dot(&self, rhs: Vec3) -> f64 {
	self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    fn cross(&self, rhs: Vec3) -> Vec3 {
	Vec3(
	    self.1 * rhs.2 - self.2 * rhs.1,
	    self.2 * rhs.0 - self.0 * rhs.2,
	    self.0 * rhs.1 - self.1 * rhs.0
	)
    }

    fn len(&self) -> f64 {
	(self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    fn norm(&self) -> Vec3 {
	self.div(self.len())
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


struct Polygon(Point3, Point3, Point3);

pub struct Ray {
    point: Point3,
    direction: Vec3
}

impl Ray {
    fn new(point: Point3, direction: Vec3) -> Ray {
	Ray {
	    point,
	    direction: direction.norm()
	}
    }
}

pub enum Transform {
    Shift(Vec3),
    Rotate(f64, f64, f64),
    Scale(f64),
    ScaleCameraDistance(f64),
}

pub struct Camera {
    position: Point3,
    direction: Vec3,
    distance: f64,
    screen_x: Vec3,
    screen_y: Vec3,
    screen_width: f64,
    screen_height: f64,
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Camera {
	Camera {
	    position: Vec3(0., 0., 0.),
	    direction: Vec3(0., 0., 1.),
	    distance: 1.,
	    screen_x: Vec3(1., 0., 0.),
	    screen_y: Vec3(0., 1., 0.),
	    screen_height: height as f64,
	    screen_width: width as f64,
	}
    }

    pub fn transform(&mut self, t: Transform) {
	use Transform::*;

	match t {
	    Shift(vec) => {
		self.position = self.position + vec;
	    },
	    Rotate(angle_x, angle_y, angle_z) => todo!("Camera rotation"),
	    Scale(factor) => {
		self.screen_x = self.screen_x * factor;
		self.screen_y = self.screen_y * factor;
	    },
	    ScaleCameraDistance(factor) => {
		self.distance *= factor;
	    },
	    _ => panic!("Unsupported transform for camera")
	}
    }

    fn screen_coords(&self, (x, y): (u32, u32)) -> Point3 {
	let (x, y) = (x as f64 - self.screen_width / 2., y as f64 - self.screen_height / 2.);
	self.screen_x * x + self.screen_y * y
    }

    pub fn get_ray(&self, coords: (u32, u32)) -> Ray {
	let screen_coords = self.screen_coords(coords);
	Ray::new(self.position, self.direction * self.distance + screen_coords - self.position)
    }
}
