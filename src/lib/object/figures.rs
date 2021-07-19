use crate::lib::{math, math::*};

use super::material::Material;

pub struct Intersection {
    pub point: Point3,
    pub n: Vec3,
    pub reflect: Ray,
    pub material: Material,

}

impl Intersection {
    pub fn new(point: Point3, n: Vec3, reflect: Ray, material: Material) -> Self { Self { point, n, reflect, material } }
}

pub trait Object {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

pub struct Polygon {
    polygon: math::Polygon,
    material: Material
}

impl Polygon {
    pub fn new(p1: Point3, p2: Point3, p3: Point3, material: Material) -> Self {
	Self {
	    polygon: math::Polygon(p1, p2, p3),
	    material }
    }
}

impl Object for Polygon {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
	let (p, refl, n) = self.polygon.intersection(ray)?;
	Some(Intersection::new(p, n, Ray::new(p, refl), self.material))
    }
}

pub struct Plane {
    plane: math::Plane,
    material: Material,
}

impl Plane {
    pub fn new(n: Vec3, d: f64, material: Material) -> Self {
	Self {
	    plane: math::Plane(n, d),
	    material,
	}
    }
}

impl Object for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
	let (p, refl, n) = self.plane.intersection(ray)?;
	Some(Intersection::new(p, n, Ray::new(p, refl), self.material))
    }
}

pub struct Sphere {
    sphere: math::Sphere,
    material: Material,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Material) -> Sphere {
	Sphere {
	    sphere: math::Sphere(center, radius),
	    material,
	}
    }
}

impl Object for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
	let (p, refl, n) = self.sphere.intersection(ray)?;
	Some(Intersection::new(p, n, Ray::new(p, refl), self.material))
    }
}
