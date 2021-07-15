
pub struct Raytracer {
}

impl Raytracer {
    pub fn new(_width: u32, _height: u32) -> Raytracer {
	Raytracer { }
    }

    pub fn render(&self, _scene: &Scene) -> Canvas {
	Canvas::new()
    }
}

pub struct Scene {
}

impl Scene {
    pub fn new() -> Scene { Scene {  } }
}

pub struct Canvas {
    matrix: Vec<Vec<Pixel>>
}

impl Canvas {
    pub fn new() -> Canvas { Canvas { matrix: vec![] } }

    pub fn iter(&self) -> std::iter::Flatten<std::slice::Iter<Vec<Pixel>>> {
	self.matrix.iter().flatten()
    }
}

#[derive(Clone, Copy)]
pub struct Pixel(pub u8, pub u8, pub u8);
