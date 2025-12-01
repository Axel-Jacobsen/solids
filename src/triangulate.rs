use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::platonic_solids::VertexId;
use crate::relax_solid::Locations;

pub fn triangulate_faces(faces: &Vec<Vec<VertexId>>) -> Vec<[VertexId; 3]> {
    let mut tris = Vec::new();
    for face in faces {
        if face.len() < 3 {
            continue;
        }
        // (v0, v_i, v_{i+1})
        for i in 1..(face.len() - 1) {
            tris.push([face[0], face[i], face[i + 1]]);
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
        let p0 = locations.get(&i0).expect("missing vertex");
        let p1 = locations.get(&i1).expect("missing vertex");
        let p2 = locations.get(&i2).expect("missing vertex");

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
