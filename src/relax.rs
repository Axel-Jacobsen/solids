use std::sync::mpsc::Sender;

use nalgebra::{Point3, Vector3};
use rand::prelude::*;

use crate::solid::{Locations, Neighbors};

type Forces = Vec<Vector3<f64>>;

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
    pub snapshot_period: u32,
    pub locations_tx: Option<Sender<Locations>>,
}

// Relax the locations of the neighbors by assuming each edge is a spring with damper.
pub fn relax(neighbors: &Neighbors, relax_params: RelaxParams) -> Locations {
    let RelaxParams {
        spring_constant,
        natural_length,
        step_size,
        total_movement_thresh,
        repulsion_constant,
        snapshot_period,
        locations_tx,
    } = relax_params;

    let mut locations: Locations = neighbors
        .iter()
        .map(|_| random_point(1.0))
        .collect();
    let mut forces: Forces = neighbors
        .iter()
        .map(|_| Vector3::new(0.0, 0.0, 0.0))
        .collect();

    let mut step = 0;
    loop {
        // Reset forces
        for f in forces.iter_mut() {
            *f = Vector3::new(0.0, 0.0, 0.0);
        }

        // Calculate the net force on each vertex.
        for (vertex, vertex_neighbors) in neighbors.iter().enumerate() {
            let this_vertex_location = &locations[vertex];

            for &neighbor in vertex_neighbors {
                let neighbor_location = &locations[neighbor];
                let distance = nalgebra::distance(neighbor_location, this_vertex_location);

                // Spring.
                let spring_force_mag = 0.5 * spring_constant * (distance - natural_length);
                forces[vertex] +=
                    spring_force_mag * (neighbor_location - this_vertex_location).normalize();
            }

            // Repulsion.
            for other_vertex in 0..neighbors.len() {
                if other_vertex == vertex {
                    continue;
                }

                let neighbor_location = &locations[other_vertex];
                let distance = nalgebra::distance(neighbor_location, this_vertex_location);

                let repulsion_force_mag = -repulsion_constant / (distance * distance);

                forces[vertex] +=
                    repulsion_force_mag * (neighbor_location - this_vertex_location).normalize();
            }
        }

        // Update states.
        let mut total_movement = 0.0;
        for vertex in 0..neighbors.len() {
            let movement = forces[vertex] * step_size;
            locations[vertex] += movement;
            total_movement += movement.norm();
        }

        // Recenter.
        let mut centroid = Vector3::new(0.0, 0.0, 0.0);
        for p in locations.iter() {
            centroid += p.coords;
        }
        centroid /= locations.len() as f64;
        for p in locations.iter_mut() {
            p.coords -= centroid;
        }

        if let Some(ref ch) = locations_tx {
            if step % snapshot_period == 0 {
                let _ = ch.send(locations.clone());
            }
        }

        step += 1;

        if total_movement / (neighbors.len() as f64) < total_movement_thresh {
            if let Some(ref ch) = locations_tx {
                let _ = ch.send(locations.clone());
            }
            break;
        }
    }

    locations
}
