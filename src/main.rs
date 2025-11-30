//! Generate the Platonic solids as STL files via constraints.

use strum::IntoEnumIterator;

mod platonic_solids;
mod relax_solid;

use platonic_solids::*;
use relax_solid::*;

fn main() {
    for solid in PlatonicSolid::iter() {
        let neighbors = &neighbors_for_solid(&solid);
        let locs = relax(
            neighbors,
            RelaxParams {
                spring_constant: 100.0,
                natural_length: 1.0,
                step_size: 1e-3,
                total_movement_thresh: 1e-6,
            },
        );
        println!(
            "{:?}: locations {:?} neighbors {:?}",
            solid, locs, neighbors
        );
    }
}
