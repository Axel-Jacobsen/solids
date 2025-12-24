use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::solid::{Locations, VertexId};

/// Slow hull algorithm.
pub fn hull_triangles(locations: &Locations) -> Vec<[VertexId; 3]> {
    let n = locations.len();
    let ids: Vec<VertexId> = (0..n).collect();

    let mut seen = std::collections::HashSet::<(VertexId, VertexId, VertexId)>::new();
    let mut tris = Vec::<[VertexId; 3]>::new();

    for a in 0..n {
        for b in (a + 1)..n {
            for c in (b + 1)..n {
                let i = ids[a];
                let j = ids[b];
                let k = ids[c];

                let pi = locations[i];
                let pj = locations[j];
                let pk = locations[k];

                let v1: nalgebra::Vector3<f64> = pj - pi;
                let v2: nalgebra::Vector3<f64> = pk - pi;
                let mut nrm = v1.cross(&v2);
                let nrm_norm = nrm.norm();

                let eps = 1e-6;
                if nrm_norm < eps {
                    continue; // colinear / degenerate
                }
                nrm /= nrm_norm;

                // Check all other points lie on one side (or in plane)
                let mut min_d = 0.0;
                let mut max_d = 0.0;

                for &l_id in &ids {
                    if l_id == i || l_id == j || l_id == k {
                        continue;
                    }
                    let pl = locations[l_id];
                    let d = nrm.dot(&(pl - pi));
                    if d < min_d {
                        min_d = d;
                    }
                    if d > max_d {
                        max_d = d;
                    }
                    // if we have points on both sides, not a hull facet
                    if min_d < -eps && max_d > eps {
                        break;
                    }
                }

                if min_d < -eps && max_d > eps {
                    continue; // both sides -> internal triangle
                }

                // orient roughly outward (assuming centered at origin)
                let face_center = (pi.coords + pj.coords + pk.coords) / 3.0;
                let mut tri = [i, j, k];
                if nrm.dot(&face_center) < 0.0 {
                    tri.swap(1, 2); // flip winding
                }

                // dedup by sorted ids
                let mut key = tri;
                key.sort();
                if seen.insert((key[0], key[1], key[2])) {
                    tris.push(tri);
                }
            }
        }
    }

    tris
}

pub fn to_stl<P: AsRef<Path>>(
    name: String,
    path: P,
    triangles: &Vec<[VertexId; 3]>,
    locations: &Locations,
) -> std::io::Result<()> {
    let file = File::create(path)?;
    let mut w = BufWriter::new(file);

    writeln!(w, "solid {}", name)?;

    for [i0, i1, i2] in triangles {
        let p0 = locations.get(*i0).expect("missing vertex");
        let p1 = locations.get(*i1).expect("missing vertex");
        let p2 = locations.get(*i2).expect("missing vertex");

        let v1: nalgebra::Vector3<f64> = p1 - p0;
        let v2: nalgebra::Vector3<f64> = p2 - p0;
        let mut n = v1.cross(&v2);
        if n.norm_squared() > 0.0 {
            n = n.normalize();
        } else {
            n = nalgebra::Vector3::new(0.0, 0.0, 0.0);
        }

        writeln!(w, "  facet normal {:.6} {:.6} {:.6}", n.x, n.y, n.z)?;
        writeln!(w, "    outer loop")?;
        writeln!(w, "      vertex {:.6} {:.6} {:.6}", p0.x, p0.y, p0.z)?;
        writeln!(w, "      vertex {:.6} {:.6} {:.6}", p1.x, p1.y, p1.z)?;
        writeln!(w, "      vertex {:.6} {:.6} {:.6}", p2.x, p2.y, p2.z)?;
        writeln!(w, "    endloop")?;
        writeln!(w, "  endfacet")?;
    }

    writeln!(w, "endsolid {}", name)?;
    w.flush()?;
    Ok(())
}
