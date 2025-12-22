//! Very simple rendering.
//!
//! Characteristics:
//! - orthographic camera
//! - image size (w,h) and pixel size derived from sensor size
//! - color / shade of the object in view will be proportional to the angle of incidence of the
//! light ray on the object.

use nalgebra;
use ndarray;

pub trait Draw {
    fn intersect(
        &self,
        ray_source: nalgebra::Point3<f64>,
        ray_direction: nalgebra::Vector3<f64>,
    ) -> bool;
}

pub struct ViewParams {
    /// Bottom-left corner of camera sensor
    pub camera_location: nalgebra::Point3<f64>,
    /// Normal direction of the camera sensor
    pub camera_normal: nalgebra::Vector3<f64>,
    /// Width of sensor in px
    pub image_width_px: usize,
    /// Height of sensor in px
    pub image_height_px: usize,
    /// Pixel size
    pub pixel_size: f64,
}

pub fn view<D: Draw>(object: &D, config: &ViewParams) -> ndarray::Array2<u8> {
    let (u, v) = basis(config.camera_normal);
    let mut image = ndarray::Array2::<u8>::zeros((config.image_height_px, config.image_width_px));
    for x in 0..config.image_width_px {
        for y in 0..config.image_height_px {
            // Pixel location wrt bottom-left pixel in sensor coordinates.
            let ray_source = config.camera_location
                + config.pixel_size * (x as f64) * u
                + config.pixel_size * (y as f64) * v;
            let intersects = object.intersect(ray_source, config.camera_normal);
            image[(y, x)] = if intersects { 255 } else { 0 };
        }
    }
    image
}

// Assume the sensor is in the x,y plane, and z is the normal plane of the camera.
// If we want a new normal, what is the basis?
fn basis(normal: nalgebra::Vector3<f64>) -> (nalgebra::Vector3<f64>, nalgebra::Vector3<f64>) {
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
