use super::super::raytracer::Pixel;

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
