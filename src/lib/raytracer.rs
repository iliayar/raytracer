use super::object::*;
use super::{math, math::*};

pub struct Raytracer {
    canvas: Option<Canvas>,
    pub scene: Scene
}

impl Raytracer {
    pub fn new(scene: Scene) -> Raytracer {
	Raytracer {
	    canvas: Some(Canvas::new(scene.width, scene.height)),
	    scene
	}
    }

    pub fn render(&mut self) -> &Canvas {
	let mut canvas = self.canvas.take().unwrap();
	canvas.update(|r: Ray| self.trace(r), &self.scene.camera);
	self.canvas.replace(canvas);
	return self.canvas.as_ref().unwrap();
    }

    fn trace(&self, ray: Ray) -> Pixel {
	if let Some(int) = self.scene.nearest_intersection(&ray) {
	    int.material.color.pixel()
	} else {
	    Pixel::default()
	}
    }
}

pub struct Scene {
    width: u32,
    height: u32,
    bodies: Vec<Box<dyn Object>>,
    lights: Vec<Box<dyn Light>>,
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
	Scene {
	    width,
	    height,
	    bodies: vec![],
	    lights: vec![],
	    camera: Camera::new(width, height),
	}
    }

    pub fn add<T: Object + 'static>(&mut self, obj: T) {
	self.bodies.push(Box::new(obj));
    }
    pub fn add_light<T: Light + 'static>(&mut self, light: T) {
	self.lights.push(Box::new(light));
    }

    fn nearest_intersection(&self, ray: &Ray) -> Option<Intersection> {
	let mut res = None;
	let mut res_dist = f64::INFINITY;

	for obj in self.bodies.iter() {
	    if let Some(int) = obj.intersect(ray) {
		let dist = ray.distance(int.point);
		if dist < res_dist {
		    res_dist = dist;
		    res = Some(int);
		}
	    }
	}

	return res;
    }
}

pub struct Canvas {
    matrix: Vec<Vec<Pixel>>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
	Canvas {
	    matrix: vec![vec![Pixel::default(); width as usize]; height as usize],
	}
    }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<Vec<Pixel>>> {
	self.matrix.iter().flatten()
    }

    pub fn size(&self) -> (u32, u32) {
	assert_ne!(self.matrix.len(), 0, "Canvas cannot be empty");
	(self.matrix[0].len() as u32, self.matrix.len() as u32)
    }

    fn update<T>(&mut self, f: T, camera: &Camera)
    where T: Fn(Ray) -> Pixel {
	self.matrix
	    .iter_mut()
	    .zip(0..)
	    .flat_map(
		|(row, y)| row.iter_mut()
		    .zip(0..)
		    .map(move |(pixel, x)| ((x, y), pixel)))
	    .for_each(|(coords, pixel)| {
		*pixel = f(camera.get_ray(coords));
	    });
    }
}

#[derive(Clone, Copy)]
pub struct Pixel(pub u8, pub u8, pub u8);

impl Default for Pixel {
    fn default() -> Self {
        Color::default().pixel()
    }
}
