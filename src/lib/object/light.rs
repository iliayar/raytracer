use crate::lib::math::*;

use super::figures::Object;
use super::figures::Intersection;
use super::material::Color;
use super::material::Material;

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
