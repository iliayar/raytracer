use super::math::*;

pub trait Body {
    fn intersect(&self, ray: &Ray) -> Option<Point3>;
}
