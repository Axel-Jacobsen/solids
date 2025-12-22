//! Generate the Platonic solids as STL files via constraints.

mod platonic_solids;
mod relax_solid;
mod solid;
mod triangulate;
mod view;

use platonic_solids::*;
use relax_solid::*;
use triangulate::*;

fn main() {
    let platonic_solid = PlatonicSolid::Dodecahedron;
    let neighbors = neighbors_for_solid(&platonic_solid);
    let locations = relax(
        &neighbors,
        RelaxParams {
            spring_constant: 1.0,
            repulsion_constant: 1.0,
            natural_length: 1.0,
            step_size: 1e-4,
            total_movement_thresh: 1e-6,
        },
    );

    let triangles = hull_triangles(&locations);

    let solid = solid::Solid {
        neighbors,
        locations,
    };

    let image = view::view(
        solid,
        view::ViewParams {
            camera_location: nalgebra::Point3::new(0.0, 0.0, 0.0),
            camera_normal: nalgebra::Vector3::z(),
            image_width_px: 4000,
            image_height_px: 4000,
            pixel_size: 0.01,
        },
    );

    let mut path = std::env::current_dir().unwrap();
    path.push(format!("{}.stl", platonic_solid.to_string()));

    to_stl(
        platonic_solid.to_string(),
        &path,
        &triangles,
        &solid.locations,
    )
    .unwrap();
}
