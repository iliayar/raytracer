use super::vector::*;

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
    pub point: Point3,
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
