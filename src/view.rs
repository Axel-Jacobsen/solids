//! Very simple rendering.
//!
//! Characteristics:
//! - orthographic camera
//! - image size (w,h) and pixel size derived from sensor size
//! - color / shade of the object in view will be proportional to the angle of incidence of the
//!   light ray on the object.

pub trait Draw {
    fn intersect(
        &self,
        ray_source: nalgebra::Point3<f64>,
        ray_direction: nalgebra::UnitVector3<f64>,
    ) -> f64;
}

pub struct ViewParams {
    /// Center of camera sensor
    pub camera_center: nalgebra::Point3<f64>,
    /// Normal direction of the camera sensor
    pub camera_normal: nalgebra::UnitVector3<f64>,
    /// Width of sensor in px
    pub image_width_px: usize,
    /// Height of sensor in px
    pub image_height_px: usize,
    /// Pixel size
    pub pixel_size: f64,
}

pub fn view<D: Draw>(object: &D, cfg: &ViewParams) -> ndarray::Array2<u8> {
    let (u, v) = basis(cfg.camera_normal);

    let w = cfg.image_width_px as f64 * cfg.pixel_size;
    let h = cfg.image_height_px as f64 * cfg.pixel_size;

    let origin = cfg.camera_center - 0.5 * w * u - 0.5 * h * v;

    let mut image = ndarray::Array2::<u8>::zeros((cfg.image_height_px, cfg.image_width_px));
    for x in 0..cfg.image_width_px {
        for y in 0..cfg.image_height_px {
            let ray_source = origin
                + cfg.pixel_size * ((x as f64) + 0.5) * u
                + cfg.pixel_size * ((y as f64) + 0.5) * v;

            image[(y, x)] = (object.intersect(ray_source, cfg.camera_normal) * 255.0) as u8;
        }
    }
    image
}

// Assume the sensor is in the x,y plane, and z is the normal plane of the camera.
// If we want a new normal, what is the basis?
fn basis(normal: nalgebra::UnitVector3<f64>) -> (nalgebra::Vector3<f64>, nalgebra::Vector3<f64>) {
    let n = normal.normalize();
    let a = if n.z.abs() < 0.9 {
        nalgebra::Vector3::z()
    } else {
        nalgebra::Vector3::x()
    };
    let u = a.cross(&n).normalize();
    let v = n.cross(&u);
    (u, v)
}
