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

    pub fn add(&self, rhs: Color) -> Color {
	Color::new(self.r.saturating_add(rhs.r),
		   self.g.saturating_add(rhs.g),
		   self.b.saturating_add(rhs.b))
    }

    pub fn mul(&self, fact: u8) -> Color {
	Color::new(self.r.saturating_mul(fact),
		   self.g.saturating_mul(fact),
		   self.b.saturating_mul(fact))
    }

    pub fn mul_float(&self, fact: f64) -> Color {
	Color::new((self.r as f64 * fact) as u8,
		   (self.g as f64 * fact) as u8,
		   (self.b as f64 * fact) as u8)
    }

    pub fn div(&self, fact: u8) -> Color {
	Color::new(self.r / fact,
		   self.g / fact,
		   self.b / fact)
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

#[derive(Clone, Copy)]
pub struct LightColor {
    color: Color,
    intensity: f64,
}

impl LightColor {
    pub fn new(color: Color, intensity: f64) -> Self { Self { color, intensity } }
    pub fn add(&self, other: LightColor) -> LightColor {
	if self.intensity < f64::EPSILON {
	    other
	} else if other.intensity < f64::EPSILON  {
	    self.clone()
	} else {
	    let fact = other.intensity / self.intensity;
	    LightColor::new(self.color.mul_float(1. / fact).add(other.color.mul_float(fact)), self.intensity + other.intensity)
	}
    }
    pub fn color(&self) -> Color {
	self.color.mul_float(self.intensity)
    }
}

pub trait Light {
    fn calc(&self, p: Point3, it: &[Box<dyn Object>]) -> Option<LightColor>;
}

pub struct PointLight {
    position: Point3,
    color: Color
}

impl PointLight {
    pub fn new(position: Point3, color: Color) -> Self { Self { position, color } }
}

impl Light for PointLight {
    fn calc(&self, p: Point3, it: &[Box<dyn Object>]) -> Option<LightColor> {
	let dist = p.distance(self.position);
	let ray = Ray::new(p, self.position - p);
	for object in it {
	    if let Some(int) = object.intersect(&ray) {
		if ray.distance(int.point) < dist {
		    return None;
		}
	    }
	}
	Some(LightColor::new(self.color, 0.5))
    }
}
