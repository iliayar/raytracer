pub mod figures;
pub mod light;
pub mod material;

pub use figures::Object;
pub use figures::Sphere;
pub use figures::Plane;
pub use figures::Polygon;
pub use figures::Intersection;

pub use light::Light;
pub use light::AmbientLight;
pub use light::PointLight;
pub use light::DirectLight;
pub use light::LightColor;

pub use material::Color;
pub use material::Material;
