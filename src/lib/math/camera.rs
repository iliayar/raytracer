use super::vector::*;
use super::ray::Ray;

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
	    }
	}
    }

    pub fn rotate(&mut self, ax: f64, ay: f64, az: f64) {
	let screen_center = self.direction * self.distance;
	self.direction = self.direction.rotate(ax, ay, az);
	self.screen_x = self.screen_x.rotate_by_point(ax, ay, az, screen_center);
	self.screen_y = self.screen_y.rotate_by_point(ax, ay, az, screen_center);
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

