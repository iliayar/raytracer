
use std::ops::{Add, Sub, Mul, Div};

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

    fn cross(&self, rhs: Vec3) -> Vec3 {
	Vec3(
	    self.1 * rhs.2 - self.2 * rhs.1,
	    self.2 * rhs.0 - self.0 * rhs.2,
	    self.0 * rhs.1 - self.1 * rhs.0
	)
    }

    pub fn len(&self) -> f64 {
	(self.0*self.0 + self.1*self.1 + self.2*self.2).sqrt()
    }

    fn norm(&self) -> Vec3 {
	self.div(self.len())
    }

    fn rotate(&self, ax: f64, ay: f64, az: f64) -> Vec3 {
	let (xcos, ycos, zcos) = (ax.cos(), ay.cos(), az.cos());
	let (xsin, ysin, zsin) = (ax.sin(), ay.sin(), az.sin());
	let rx = Vec3(zcos*ycos, zcos*ysin*xsin - zsin*xcos, zcos*ysin*xcos + zsin*xsin);
	let ry = Vec3(zsin*ycos, zsin*ysin*xsin + zcos*xcos, zsin*ysin*xcos - zcos*xsin);
	let rz = Vec3(-ysin, ycos*xsin, ycos*xcos);
	Vec3(self.dot(rx), self.dot(ry), self.dot(rz))
    }

    fn rotate_by_point(&self, ax: f64, ay: f64, az: f64, p: Point3) -> Vec3 {
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

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * v.dot(n.norm()) * n.norm()
}

fn fix_point_reflect(p: Point3, v: Vec3, n: Vec3) -> (Point3, Vec3, Vec3) {
    let mut n = n.norm();
    if n.dot(v) > 0. {
	n = -1. * n;
    }
    (p + n * 100. * f64::EPSILON, reflect(v, n), n)
}

fn intersect_plane(ray: &Ray, n: Vec3, d: f64) -> Option<Point3> {
    if n.dot(ray.direction).abs() < f64::EPSILON {
	return None;
    }

    let t = - (n.dot(ray.point) + d) / n.dot(ray.direction);

    if t < 0. {
	return None;
    }

    let int = ray.point + t * ray.direction;
    Some(int)
}

pub struct Polygon(pub Point3, pub Point3, pub Point3);

pub trait RayIntersect {
    fn intersection(&self, ray: &Ray) -> Option<(Point3, Vec3, Vec3)>;
}

impl RayIntersect for Polygon {
    fn intersection(&self, ray: &Ray) -> Option<(Point3, Vec3, Vec3)> {
	let n = (self.1 - self.0).cross(self.2 - self.0);
	let d = -1. * n.dot(self.0);
	let int = intersect_plane(ray, n, d)?;

	let check = |pb: Point3, pe: Point3| n.dot((pe - pb).cross(int - pb)) >= 0.;
	if !check(self.0, self.1) || !check(self.1, self.2) || !check(self.2, self.0) {
	    return None;
	}
	Some(fix_point_reflect(int, ray.direction, n))
    }
}

pub struct Sphere(pub Point3, pub f64);

impl RayIntersect for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<(Point3, Vec3, Vec3)> {
	let center_vec = ray.point - self.0;
	let a = ray.direction.dot(ray.direction);
	let b = 2.0 * ray.direction.dot(center_vec);
	let c = center_vec.dot(center_vec) - self.1*self.1;

	let det = b * b - 4. * a * c;
	if det < 0. {
	    return None;
	}

	let t1 = (-b - det.sqrt())/(2. * a);
	let t2 = (-b + det.sqrt())/(2. * a);
	let mut p = ray.direction * t1 + ray.point;
	
	if t1 < 0. {
	    if t2 < 0. {
		return None;
	    } else {
		p = ray.direction * t2 + ray.point;
	    }
	}

	let n = p - self.0;
	Some(fix_point_reflect(p, ray.direction, n))
    }
}

pub struct Plane(pub Vec3, pub f64);

impl RayIntersect for Plane {
    fn intersection(&self, ray: &Ray) -> Option<(Point3, Vec3, Vec3)> {
	Some(fix_point_reflect(intersect_plane(ray, self.0, self.1)?, ray.direction, self.0))
    }
}

#[derive(Debug,PartialEq)]
pub struct Ray {
    point: Point3,
    pub direction: Vec3
}

impl Ray {
    pub fn new(point: Point3, direction: Vec3) -> Ray {
	Ray {
	    point,
	    direction: direction.norm()
	}
    }
}

pub enum CameraTransform {
    ScaleScreen(f64),
    ScaleDistance(f64),
    Move(f64),
    RotateHorizontal(f64),
    RotateVertical(f64),
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
	    position: Vec3(0., 0.5, -1.),
	    direction: Vec3(0., 0., 1.),
	    distance: 1.,
	    screen_x: Vec3(-1. / width as f64, 0., 0.),
	    screen_y: Vec3(0., -1. / height as f64, 0.),
	    screen_height: height as f64,
	    screen_width: width as f64,
	}
    }

    pub fn transform(&mut self, t: CameraTransform) {
	use CameraTransform::*;

	match t {
	    ScaleScreen(factor) => {
		self.screen_x = self.screen_x * factor;
		self.screen_y = self.screen_y * factor;
	    },
	    ScaleDistance(factor) => {
		self.distance *= factor;
	    },
	    Move(distance) => {
		self.position = self.position + self.direction * distance;
	    },
	    RotateHorizontal(angle) => {
		self.rotate(0., angle, 0.);
	    },
	    RotateVertical(angle) => {
		let v = Vec3(0., 1., 0.).cross(self.direction).norm();
		self.rotate(angle * v.0, 0., angle * v.2);
	    },
	    _ => ()
	}
    }

    pub fn rotate(&mut self, ax: f64, ay: f64, az: f64) {
	let screen_center = self.direction * self.distance;
	self.direction = self.direction.rotate(ax, ay, az);
	self.screen_x = self.screen_x.rotate_by_point(ax, ay, az, screen_center);
	self.screen_y = self.screen_y.rotate_by_point(ax, ay, az, screen_center);
    }

    pub fn screen_intersection(&self, (x, y): (u32, u32)) -> Point3 {
	self.position + self.direction * self.distance + self.screen_coords((x, y))
    }

    fn screen_coords(&self, (x, y): (u32, u32)) -> Vec3 {
	let (x, y) = (x as f64 - self.screen_width / 2., y as f64 - self.screen_height / 2.);
	self.screen_x * x + self.screen_y * y
    }

    pub fn get_ray(&self, coords: (u32, u32)) -> Ray {
	let screen_coords = self.screen_coords(coords);
	Ray::new(self.position, self.direction * self.distance + screen_coords)
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

    #[test]
    fn camera() {
	let camera = Camera::new(100, 100);

	assert_eq!(camera.screen_x.cross(camera.screen_y).norm(), camera.direction);
	// assert_eq!(camera.get_ray((0, 0)), Ray::new(Vec3(0., 50., 0.), Vec3(50., 50., 1.)));
	// assert_eq!(camera.get_ray((50, 50)), Ray::new(Vec3(0., 50., 0.), Vec3(0., 0., 1.)));
	// assert_eq!(camera.get_ray((100, 100)), Ray::new(Vec3(0., 50., 0.), Vec3(-50., -50., 1.)));
	// assert_eq!(camera.get_ray((50, 0)), Ray::new(Vec3(0., 50., 0.), Vec3(0., 50., 1.)));
    }

    // #[test]
    // fn intersection_plane() {
    // 	let plane = Plane(Vec3(0., 1., 0.), 0.);

    // 	assert_eq!(plane.intersection(&Ray::new(Vec3(0., 1., 0.), Vec3(0., -1., 0.))).unwrap().0, Vec3(0., 0. + f64::EPSILON, 0.));
    // }

    // #[test]
    // fn intersection_polygon() {
    // 	let polygon = Polygon(Vec3(0., 0., 1.), Vec3(0., 0., -1.), Vec3(1., 0., 0.));

    // 	assert_eq!(polygon.intersection(&Ray::new(Vec3(0., 1., 0.), Vec3(0., -1., 0.))).unwrap().0, Vec3(0., 0. + f64::EPSILON * 2., 0.));
    // }
}
