use super::{math, math::*};
use super::raytracer::Pixel;

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0x00, 0x00, 0x00)
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self { Self { r, g, b } }
    pub fn pixel(&self) -> Pixel {
	Pixel(self.r, self.g, self.b)
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color
}

impl Default for Material {
    fn default() -> Self {
        Material::new(Color::default())
    }
}

impl Material {
    pub fn new(color: Color) -> Self { Self { color } }
}

pub struct Intersection {
    pub point: Point3,
    pub reflect: Vec3,
    pub material: Material
}

impl Intersection {
    pub fn new(point: Point3, reflect: Vec3, material: Material) -> Self { Self { point, reflect, material } }
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
	let (p, refl) = self.polygon.intersection(ray)?;
	Some(Intersection::new(p, refl, self.material))
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
	let (p, refl) = self.plane.intersection(ray)?;
	Some(Intersection::new(p, refl, self.material))
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
	let (p, refl) = self.sphere.intersection(ray)?;
	Some(Intersection::new(p, refl, self.material))
    }
}

struct LightColor {
    color: Color
}

impl LightColor {
    fn new(color: Color) -> Self { Self { color } }
}

pub trait Light {
    fn min_dist(&self, p: Point3) -> (f64, LightColor);
}

struct PointLight {
    position: Point3,
    color: Color
}

impl Light for PointLight {
    fn min_dist(&self, p: Point3) -> (f64, LightColor) {
	((p - self.position).len(), LightColor::new(self.color))
    }
}
