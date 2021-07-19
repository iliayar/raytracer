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
    pub fn white() -> Color { Color::new(0xff, 0xff, 0xff) }
    pub fn pixel(&self) -> Pixel {
	Pixel(self.r, self.g, self.b)
    }

    pub fn add(&self, rhs: Color) -> Color {
	Color::new(self.r.saturating_add(rhs.r),
		   self.g.saturating_add(rhs.g),
		   self.b.saturating_add(rhs.b))
    }

    pub fn sub(&self, rhs: Color) -> Color {
	Color::new(self.r.saturating_sub(rhs.r),
		   self.g.saturating_sub(rhs.g),
		   self.b.saturating_sub(rhs.b))
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
    pub color: Color,
    pub shine: Option<i32>,
    pub reflection: f64,
}

impl Default for Material {
    fn default() -> Self {
	Material::new(Color::default(), 0.)
    }
}

impl Material {
    pub fn new(color: Color, reflection: f64) -> Self { Self { color, shine: None, reflection } }
    pub fn new_shine(color: Color, shine: i32, reflection: f64) -> Self {
	Self {
	    color,
	    shine: Some(shine),
	    reflection
	}
    }

}

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

#[derive(Clone, Copy)]
pub struct LightColor {
    color: Option<Color>,
    intensity: f64,
}

impl LightColor {
    pub fn new(color: Option<Color>, intensity: f64) -> Self { Self { color: color, intensity } }
    pub fn add(&self, other: LightColor) -> LightColor {
	if self.intensity < f64::EPSILON {
	    other
	} else if other.intensity < f64::EPSILON  {
	    self.clone()
	} else {
	    let fact = other.intensity / (self.intensity + other.intensity);
	    let intensity = self.intensity + other.intensity;
	    if let Some(color) = self.color {
		if let Some(other_color) = other.color {
		    LightColor::new(Some(color.mul_float(1. - fact).add(other_color.mul_float(fact))), intensity)
		} else {
		    LightColor::new(Some(color), intensity)
		}
	    } else if let Some(other_color) = other.color {
		LightColor::new(Some(other_color), intensity)
	    } else {
		LightColor::new(None, self.intensity + other.intensity)
	    }
	}
    }
    pub fn calc_color(&self, material: Material) -> Color {
	if let Some(color) = self.color {
	    // FIXME
	    material.color.mul_float(self.intensity * 3. / 4.).add(color.mul_float(self.intensity / 4.)) 
	} else {
	    material.color.mul_float(self.intensity)
	}
    }
}

fn calc_light(dir: Vec3, ray: &Ray, intensity: f64, color: Option<Color>, intersection: &Intersection) -> LightColor {
	let diffuse = intensity * dir.dot(intersection.n) / (dir.len() * intersection.n.len());
	let shine_base = intersection.reflect.direction.dot(ray.direction) / (intersection.reflect.direction.len() * ray.direction.len());
	LightColor::new(color, diffuse + if let Some(shine) = intersection.material.shine {
	    intensity * shine_base.powi(shine)
	} else {
	    0.
	})
}

pub trait Light {
    fn calc(&self, ray: &Ray, intersection: &Intersection, it: &[Box<dyn Object>]) -> Option<LightColor>;
}

pub struct PointLight {
    position: Point3,
    color: Option<Color>,
    intensity: f64,
}

impl PointLight {
    pub fn new(position: Point3, intensity: f64) -> Self { Self { position, color: None, intensity } }
    pub fn new_color(position: Point3, intensity: f64, color: Color) -> Self { Self { position, color: Some(color), intensity } }
}

impl Light for PointLight {
    fn calc(&self, origin_ray: &Ray, intersection: &Intersection, it: &[Box<dyn Object>]) -> Option<LightColor> {
	let p = intersection.point;
	let dist = p.distance(self.position);
	let dir = self.position - p;
	let ray = Ray::new(p, dir);
	for object in it {
	    if let Some(int) = object.intersect(&ray) {
		if ray.distance(int.point) < dist {
		    return None;
		}
	    }
	}
	Some(calc_light(dir, &origin_ray, self.intensity, self.color, intersection))
    }
}

pub struct AmbientLight {
    color: Option<Color>,
    intensity: f64,
}

impl AmbientLight {
    pub fn new(intensity: f64) -> Self { Self { color: None, intensity } }
}

impl Light for AmbientLight {
    fn calc(&self, _: &Ray, intersection: &Intersection, _: &[Box<dyn Object>]) -> Option<LightColor> {
	Some(LightColor::new(self.color, self.intensity))
    }
}

pub struct DirectLight {
    color: Option<Color>,
    intensity: f64,
    direction: Vec3,
}

impl DirectLight {
    pub fn new(direction: Vec3, intensity: f64) -> Self { Self { color: None, intensity, direction } }
}

impl Light for DirectLight {
    fn calc(&self, origin_ray: &Ray, intersection: &Intersection, it: &[Box<dyn Object>]) -> Option<LightColor> {
	let ray = Ray::new(intersection.point, -1. * self.direction);
	for object in it {
	    if let Some(_) = object.intersect(&ray) {
		return None
	    }
	}
	Some(calc_light(-1. * self.direction, &origin_ray, self.intensity, self.color, intersection))
    }
}
