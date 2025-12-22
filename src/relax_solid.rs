use nalgebra::{Point3, Vector3};
use rand::prelude::*;

use crate::solid::{Locations, Neighbors, VertexId};

type Forces = std::collections::HashMap<VertexId, Vector3<f64>>;

fn random_point(sphere_size: f64) -> Point3<f64> {
    let mut rng = rand::rng();
    let (x, y, z): (f64, f64, f64) = rng.random();
    sphere_size * Point3::new(x, y, z)
}

pub struct RelaxParams {
    pub spring_constant: f64,
    pub repulsion_constant: f64,
    pub natural_length: f64,
    pub step_size: f64,
    pub total_movement_thresh: f64,
}

// Relax the locations of the neighbors by assuming each edge is a spring with damper.
pub fn relax(neighbors: &Neighbors, relax_params: RelaxParams) -> Locations {
    let RelaxParams {
        spring_constant,
        natural_length,
        step_size,
        total_movement_thresh,
        repulsion_constant,
    } = relax_params;

    let mut locations: Locations =
        std::collections::HashMap::from_iter(neighbors.keys().map(|k| (*k, random_point(1.0))));
    let mut forces: Forces = std::collections::HashMap::from_iter(
        neighbors.keys().map(|k| (*k, Vector3::new(0.0, 0.0, 0.0))),
    );

    loop {
        // Reset forces
        for f in forces.values_mut() {
            *f = Vector3::new(0.0, 0.0, 0.0);
        }

        // Calculate the net force on each vertex.
        for (vertex, vertex_neighbors) in neighbors.iter() {
            let this_vertex_location = locations.get(vertex).unwrap(); // I weep for the unwraps.

            for neighbor in vertex_neighbors {
                let neighbor_location = locations.get(neighbor).unwrap();
                let distance = nalgebra::distance(neighbor_location, this_vertex_location);

                // Spring.
                let spring_force_mag = 0.5 * spring_constant * (distance - natural_length);
                *forces.get_mut(vertex).unwrap() +=
                    spring_force_mag * (neighbor_location - this_vertex_location).normalize();
            }

            // Repulsion.
            for other_vertex in neighbors.keys() {
                if other_vertex == vertex {
                    continue;
                }

                let neighbor_location = locations.get(other_vertex).unwrap();
                let distance = nalgebra::distance(neighbor_location, this_vertex_location);

                let repulsion_force_mag = -repulsion_constant / (distance * distance);

                *forces.get_mut(vertex).unwrap() +=
                    repulsion_force_mag * (neighbor_location - this_vertex_location).normalize();
            }
        }

        // Update states.
        let mut total_movement = 0.0;
        for vertex in neighbors.keys() {
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
