use nalgebra;

use crate::view::Draw;

const EPS: f64 = 1e-6;

pub type VertexId = usize;
pub type Locations = std::collections::HashMap<VertexId, nalgebra::Point3<f64>>;
pub type Neighbors = std::collections::HashMap<VertexId, Vec<VertexId>>;
pub type Triangles = Vec<[VertexId; 3]>;

pub struct Solid {
    pub locations: Locations,
    pub triangles: Triangles,
}

impl Draw for Solid {
    // The ray is defined by P(r) = ray_source + r * ray_direction.
    // Algorithm from https://web.archive.org/web/20210330124410/http://geomalgorithms.com/a06-_intersect-2.html
    fn intersect(
        &self,
        ray_source: nalgebra::Point3<f64>,
        ray_direction: nalgebra::Vector3<f64>,
    ) -> bool {
        for [v0_id, v1_id, v2_id] in self.triangles.iter() {
            // Locations of the verticies of the triangle.
            let (v0, v1, v2) = (
                self.locations[&v0_id],
                self.locations[&v1_id],
                self.locations[&v2_id],
            );

            // Two of the edges of the triangle.
            let u = v1 - v0;
            let v = v2 - v0;

            // First, determine if the ray intersects with the plane of the triangle.
            let triangle_normal = u.cross(&v);

            // Triangle is parallel or in-plane with the ray.
            if triangle_normal.dot(&ray_direction).abs() < EPS {
                continue;
            }

            let ray_triangle_plane_intersection =
                triangle_normal.dot(&(v0 - ray_source)) / triangle_normal.dot(&ray_direction);

            // Triangle is behind camera.
            if ray_triangle_plane_intersection < 0.0 {
                continue;
            }

            // Now, determine whether the intersection with the plane is in the triangle.
            let triangle_plane_intersection =
                ray_source + ray_triangle_plane_intersection * ray_direction;
            let w = triangle_plane_intersection - v0;

            let n1 = u.dot(&v) * w.dot(&v) - (v.dot(&v)) * w.dot(&u);
            let n2 = u.dot(&v) * w.dot(&u) - (u.dot(&u)) * w.dot(&v);
            let denom = u.dot(&v).powf(2.0) - (u.dot(&u)) * (v.dot(&v));
            if denom.abs() < EPS {
                continue;
            }

            let s = n1 / denom;
            let t = n2 / denom;

            // Intersection is within the triangle.
            if 0.0 <= s && 0.0 <= t && (0.0 <= s + t && s + t <= 1.0) {
                return true;
            }
        }
        false
    }
}
