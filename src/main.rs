//! Generate the Platonic solids as STL files via constraints.

mod platonic_solids;
mod relax_solid;
mod triangulate;

use platonic_solids::*;
use relax_solid::*;
use triangulate::*;

use strum::IntoEnumIterator;

fn main() {
    for solid in PlatonicSolid::iter() {
        let neighbors = &neighbors_for_solid(&solid);
        let locations = relax(
            neighbors,
            RelaxParams {
                spring_constant: 1.0,
                repulsion_constant: 1.0,
                natural_length: 1.0,
                step_size: 1e-6,
                total_movement_thresh: 1e-5,
            },
        );

        let triangles = triangulate_faces(&faces_for_solid(&solid));

        let mut path = std::env::current_dir().unwrap();
        path.push(format!("{}.stl", solid.to_string()));

        to_stl(solid.to_string(), path, &triangles, &locations).unwrap();

        println!(
            "{:?}: locations {:?} neighbors {:?}",
            solid, locations, neighbors
        );
    }
}
