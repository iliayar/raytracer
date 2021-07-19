
#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;
    
    #[test]
    fn primitives_sum_scalar() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	let sum1 = x + y + z;
	let sum2 = x + y * 2. + z * 3.;
	let sum3 = x - y * 2. + z * 3.;
	assert_eq!(sum1, Vec3(1., 1., 1.));
	assert_eq!(sum2, Vec3(1., 2., 3.));
	assert_eq!(sum3, Vec3(1., -2., 3.));
    }

    #[test]
    fn primitives_dot_product() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	let a = x * 2. + y * 3. + z * 4.;
	let b = x + y + z;

	let zero = Vec3(0., 0., 0.);
	assert_eq!(a.dot(b), 2. + 3. + 4.);
	assert_eq!(zero.dot(b), 0.);
	assert_eq!(zero.dot(a), 0.);
    }

    #[test]
    fn primitives_cross_product() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	assert_eq!(z.cross(x), y);
	assert_eq!(y.cross(z), x);
	assert_eq!(x.cross(y), z);

	let a = x * 3. + y * 3.;
	let b = x * -3. + y * 3.;

	assert_eq!(a.cross(b).norm(), z);
    }

    #[test]
    fn primitives_rotation() {
	let x = Vec3(1., 0., 0.);
	let y = Vec3(0., 1., 0.);
	let z = Vec3(0., 0., 1.);

	assert_eq!(x.rotate(PI, 0., 0.), x);
	assert_eq!(y.rotate(0., PI, 0.), y);
	assert_eq!(z.rotate(0., 0., PI), z);

	assert_eq!(x.rotate(0., - PI / 2., 0.), z);
	assert_eq!(x.rotate(0., 0., PI / 2.), y);
	
	assert_eq!(y.rotate(PI / 2., 0., 0.), z);
	assert_eq!(y.rotate(0., 0., - PI / 2.), x);

	assert_eq!(z.rotate(-PI / 2., 0., 0.), y);
	assert_eq!(z.rotate(0., PI / 2., 0.), x);

	assert_eq!(z.rotate_by_point(PI / 2., 0., 0., y), -1. * y);
    }

    #[test]
    fn camera() {
	let camera = Camera::new(100, 100);

	assert_eq!(camera.screen_x.cross(camera.screen_y).norm(), camera.direction);
	// assert_eq!(camera.get_ray((0, 0)), Ray::new(Vec3(0., 50., 0.), Vec3(50., 50., 1.)));
	// assert_eq!(camera.get_ray((50, 50)), Ray::new(Vec3(0., 50., 0.), Vec3(0., 0., 1.)));
	// assert_eq!(camera.get_ray((100, 100)), Ray::new(Vec3(0., 50., 0.), Vec3(-50., -50., 1.)));
	// assert_eq!(camera.get_ray((50, 0)), Ray::new(Vec3(0., 50., 0.), Vec3(0., 50., 1.)));
    }

    // #[test]
    // fn intersection_plane() {
    // 	let plane = Plane(Vec3(0., 1., 0.), 0.);

    // 	assert_eq!(plane.intersection(&Ray::new(Vec3(0., 1., 0.), Vec3(0., -1., 0.))).unwrap().0, Vec3(0., 0. + f64::EPSILON, 0.));
    // }

    // #[test]
    // fn intersection_polygon() {
    // 	let polygon = Polygon(Vec3(0., 0., 1.), Vec3(0., 0., -1.), Vec3(1., 0., 0.));

    // 	assert_eq!(polygon.intersection(&Ray::new(Vec3(0., 1., 0.), Vec3(0., -1., 0.))).unwrap().0, Vec3(0., 0. + f64::EPSILON * 2., 0.));
    // }
}
