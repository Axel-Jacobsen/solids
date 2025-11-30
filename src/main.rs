//! Generate the Platonic solids as STL files via constraints.
//!
//! Every edge of a Platonic solid is of the same length. Every vertex of a platonic solid has the
//! same number of edges.
//!
//! How do we do this?
//!
//! Iterively:
//! - Specify the number of veriticies and number of edges meeting at each vertex.

/// Regular polygons. Please excuse me dropping 'Regular'.
#[derive(Debug)]
enum Polygon {
    Triangle,
    Square,
    Pentagon,
}

fn number_of_edges(polygon: &Polygon) -> usize {
    match polygon {
        Polygon::Triangle => 3,
        Polygon::Square => 4,
        Polygon::Pentagon => 5,
    }
}

enum PlatonicSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

fn face_type(platonic_solid: &PlatonicSolid) -> Polygon {
    match platonic_solid {
        PlatonicSolid::Tetrahedron => Polygon::Triangle,
        PlatonicSolid::Cube => Polygon::Square,
        PlatonicSolid::Octahedron => Polygon::Triangle,
        PlatonicSolid::Dodecahedron => Polygon::Pentagon,
        PlatonicSolid::Icosahedron => Polygon::Triangle,
    }
}

fn number_of_faces(platonic_solid: &PlatonicSolid) -> usize {
    match platonic_solid {
        PlatonicSolid::Tetrahedron => 4,
        PlatonicSolid::Cube => 6,
        PlatonicSolid::Octahedron => 8,
        PlatonicSolid::Dodecahedron => 12,
        PlatonicSolid::Icosahedron => 20,
    }
}

fn number_of_verticies(platonic_solid: &PlatonicSolid) -> usize {
    match platonic_solid {
        PlatonicSolid::Tetrahedron => 4,
        PlatonicSolid::Cube => 8,
        PlatonicSolid::Octahedron => 6,
        PlatonicSolid::Dodecahedron => 20,
        PlatonicSolid::Icosahedron => 12,
    }
}

fn number_of_edges_per_vertex(platonic_solid: &PlatonicSolid) -> usize {
    number_of_edges(&face_type(platonic_solid))
}

type VertexId = usize;
type Locations = std::collections::HashMap<VertexId, [f64; 3]>;
type Neighbors = std::collections::HashMap<VertexId, Vec<VertexId>>;

fn neighbors_for_solid(platonic_solid: &PlatonicSolid) -> Neighbors {
    let mut neighbors: Neighbors = std::collections::HashMap::from_iter(
        (0..number_of_verticies(platonic_solid)).map(|i| (i, vec![])),
    );

    let edges_per_vertex = number_of_edges_per_vertex(platonic_solid);

    loop {
        // Get a vertex from the neighbor map.
        let v1 = match neighbors
            .iter()
            .find(|(_v, n)| n.len() < edges_per_vertex)
            .map(|(v, _)| *v)
        {
            Some(v) => v,
            None => break,
        };

        let v2 = match neighbors
            .iter()
            .find(|(v, n)| v != &&v1 && !n.contains(&&v1) && n.len() < edges_per_vertex)
            .map(|(v, _)| *v)
        {
            Some(v) => v,
            None => {
                panic!("shouldn't be possible, should not be able to have only 1 incomplete vertex")
            }
        };

        neighbors.get_mut(&v1).expect("not possible").push(v2);
        neighbors.get_mut(&v2).expect("not possible").push(v1);
    }

    neighbors
}

fn main() {
    let vs = neighbors_for_solid(&PlatonicSolid::Tetrahedron);
    println!("{:?}", vs);
}
