use nalgebra;

use crate::view::Draw;

pub type VertexId = usize;
pub type Locations = std::collections::HashMap<VertexId, nalgebra::Point3<f64>>;
pub type Neighbors = std::collections::HashMap<VertexId, Vec<VertexId>>;

pub struct Solid {
    pub locations: Locations,
    pub neighbors: Neighbors,
}

impl Draw for Solid {
    fn intersect(
        &self,
        _ray_source: nalgebra::Point3<f64>,
        _ray_direction: nalgebra::Vector3<f64>,
    ) -> bool {
        true
    }
}
