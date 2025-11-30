//! Generate the Platonic solids as STL files via constraints.

use nalgebra::{Point3, Vector3};
use rand::prelude::*;
use strum::IntoEnumIterator;

mod platonic_solids;
use platonic_solids::*;

type VertexId = usize;
type Neighbors = std::collections::HashMap<VertexId, Vec<VertexId>>;

type Locations = std::collections::HashMap<VertexId, Point3<f64>>;
type Forces = std::collections::HashMap<VertexId, Vector3<f64>>;

fn random_point() -> Point3<f64> {
    let mut rng = rand::rng();
    let (x, y, z): (f64, f64, f64) = rng.random();
    Point3::new(x, y, z)
}

fn neighbors_for_solid(solid: &PlatonicSolid) -> Neighbors {
    let edges = edges_for_solid(solid);

    let n = number_of_verticies(solid);
    let mut neighbors: Neighbors = (0..n).map(|i| (i, Vec::new())).collect();

    for &(a, b) in edges {
        neighbors
            .get_mut(&a)
            .expect("can't find vertex for edge")
            .push(b);
        neighbors
            .get_mut(&b)
            .expect("can't find vertex for edge")
            .push(a);
    }

    neighbors
}

struct RelaxParams {
    spring_constant: f64,
    natural_length: f64,
    step_size: f64,
    total_movement_thresh: f64,
}

// Relax the locations of the neighbors by assuming each edge is a spring with damper.
fn relax(neighbors: &Neighbors, relax_params: RelaxParams) -> Locations {
    let RelaxParams {
        spring_constant,
        natural_length,
        step_size,
        total_movement_thresh,
    } = relax_params;

    let mut locations: Locations =
        std::collections::HashMap::from_iter(neighbors.keys().map(|k| (*k, random_point())));
    let mut forces: Forces = std::collections::HashMap::from_iter(
        neighbors.keys().map(|k| (*k, Vector3::new(0.0, 0.0, 0.0))),
    );

    loop {
        // Reset forces
        for f in forces.values_mut() {
            *f = Vector3::new(0.0, 0.0, 0.0);
        }

        // Calculate the net force on each vertex.
        for (vertex, neighbors) in neighbors.iter() {
            let this_vertex_location = locations.get(vertex).unwrap(); // I weep for the unwraps.

            // Spring.
            for neighbor in neighbors {
                let neighbor_location = locations.get(neighbor).unwrap();

                let distance = nalgebra::distance(neighbor_location, this_vertex_location);
                let force_mag = spring_constant * (distance - natural_length);

                *forces.get_mut(vertex).unwrap() +=
                    force_mag * (neighbor_location - this_vertex_location).normalize()
            }
        }

        // Update states.
        // x = x0 + vt + 1/2 at^2
        // F = ma => a = F / m
        // a = dv/dt = (v1 - v0) / t so v1 = at + v0
        let mut total_movement = 0.0;
        for vertex in neighbors.keys() {
            // Update position and velocity.
            let movement = forces[vertex] * step_size;
            *locations.get_mut(vertex).unwrap() += movement;
            total_movement += movement.norm();
        }

        // Recenter.
        let mut centroid = Vector3::new(0.0, 0.0, 0.0);
        for p in locations.values() {
            centroid += p.coords;
        }
        centroid /= locations.len() as f64;
        for p in locations.values_mut() {
            p.coords -= centroid;
        }

        if total_movement / (neighbors.len() as f64) < total_movement_thresh {
            break;
        }
    }

    locations
}

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
