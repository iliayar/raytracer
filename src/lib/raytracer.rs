use super::body::*;
use super::math::*;

pub struct Raytracer {
    canvas: Option<Canvas>
}

impl Raytracer {
    pub fn new() -> Raytracer {
	Raytracer {
	    canvas: None
	}
    }

    pub fn render(&mut self, scene: &Scene) -> &Canvas {
	if let Some(canvas) = &self.canvas {
	    let (width, height) = canvas.size();
	    if width != scene.width || height != scene.height {
		self.canvas = None;
	    } 
	}
	
	self.canvas.get_or_insert_with(|| Canvas::new(scene.width, scene.height)).update(Raytracer::update);

	return self.canvas.as_ref().unwrap();
    }

    fn update(ray: Ray) -> Pixel {
	Pixel(0x00, 0xff, 0xa0)
    }
}

pub struct Scene {
    width: u32,
    height: u32,
    bodies: Vec<Box<dyn Body>>
}

impl Scene {
    pub fn new(width: u32, height: u32) -> Scene {
	Scene {
	    width,
	    height,
	    bodies: vec![]
	}
    }

    pub fn add(&mut self, body: Box<dyn Body>) {
	self.bodies.push(body);
    }
}

pub struct Canvas {
    matrix: Vec<Vec<Pixel>>,
    camera: Camera
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
	Canvas {
	    matrix: vec![vec![Pixel::new(); width as usize]; height as usize],
	    camera: Camera::new(width, height)
	}
    }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<Vec<Pixel>>> {
	self.matrix.iter().flatten()
    }

    pub fn size(&self) -> (u32, u32) {
	assert_ne!(self.matrix.len(), 0, "Canvas cannot be empty");
	(self.matrix[0].len() as u32, self.matrix.len() as u32)
    }

    fn update<T>(&mut self, f: T)
    where T: Fn(Ray) -> Pixel {
	let camera: &Camera = &self.camera;
	self.matrix
	    .iter_mut()
	    .zip(0..)
	    .flat_map(
		|(row, x)| row.iter_mut()
		    .zip(0..)
		    .map(move |(pixel, y)| ((x, y), pixel)))
	    .for_each(|(coords, pixel)| {
		*pixel = f(camera.get_ray(coords));
	    });
    }
}

#[derive(Clone, Copy)]
pub struct Pixel(pub u8, pub u8, pub u8);

impl Pixel {
    fn new() -> Pixel {
	Pixel(0xff, 0x00, 0x00)
    }
}
