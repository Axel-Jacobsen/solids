//! Generate the Platonic solids as STL files via constraints.

mod platonic_solids;
mod relax_solid;
mod solid;
mod triangulate;
mod view;

use platonic_solids::*;
use relax_solid::*;
use triangulate::*;

use image::GrayImage;

fn main() {
    let platonic_solid = PlatonicSolid::Dodecahedron;
    let neighbors = neighbors_for_solid(&platonic_solid);
    let locations = relax(
        &neighbors,
        RelaxParams {
            spring_constant: 1.0,
            repulsion_constant: 0.1,
            natural_length: 0.1,
            step_size: 1e-4,
            total_movement_thresh: 1e-3,
        },
    );

    let triangles = hull_triangles(&locations);

    let solid = solid::Solid {
        locations,
        triangles,
    };

    let view_params = view::ViewParams {
        camera_location: nalgebra::Point3::new(0.0, 0.0, -10.0),
        camera_normal: nalgebra::Vector3::z(),
        image_width_px: 4000,
        image_height_px: 4000,
        pixel_size: 0.1,
    };

    let image = view::view(&solid, &view_params);
    save_image(
        image,
        view_params.image_width_px as u32,
        view_params.image_height_px as u32,
    );

    let mut path = std::env::current_dir().unwrap();
    path.push(format!("{}.stl", platonic_solid.to_string()));

    to_stl(
        platonic_solid.to_string(),
        &path,
        &solid.triangles,
        &solid.locations,
    )
    .unwrap();
}

fn save_image(image: ndarray::Array2<u8>, width: u32, height: u32) {
    assert!(image.is_standard_layout());
    let raw: Vec<u8> = match image.as_standard_layout().as_slice() {
        Some(slice) => slice.to_vec(),
        None => image.iter().copied().collect(),
    };

    GrayImage::from_raw(width, height, raw)
        .expect("container should have the right size for the image dimensions")
        .save("out.png")
        .expect("to save the image");
}
