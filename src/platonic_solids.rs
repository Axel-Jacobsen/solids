use strum::EnumIter;

pub type VertexId = usize;
pub type Neighbors = std::collections::HashMap<VertexId, Vec<VertexId>>;

#[derive(Debug, EnumIter)]
pub enum PlatonicSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

pub fn number_of_faces(platonic_solid: &PlatonicSolid) -> usize {
    match platonic_solid {
        PlatonicSolid::Tetrahedron => 4,
        PlatonicSolid::Cube => 6,
        PlatonicSolid::Octahedron => 8,
        PlatonicSolid::Dodecahedron => 12,
        PlatonicSolid::Icosahedron => 20,
    }
}

pub fn number_of_verticies(platonic_solid: &PlatonicSolid) -> usize {
    match platonic_solid {
        PlatonicSolid::Tetrahedron => 4,
        PlatonicSolid::Cube => 8,
        PlatonicSolid::Octahedron => 6,
        PlatonicSolid::Dodecahedron => 20,
        PlatonicSolid::Icosahedron => 12,
    }
}

pub fn edges_for_solid(platonic_solid: &PlatonicSolid) -> &[(usize, usize)] {
    match platonic_solid {
        PlatonicSolid::Tetrahedron => EDGES_TETRAHEDRON,
        PlatonicSolid::Cube => EDGES_CUBE,
        PlatonicSolid::Octahedron => EDGES_OCTAHEDRON,
        PlatonicSolid::Dodecahedron => EDGES_DODECAHEDRON,
        PlatonicSolid::Icosahedron => EDGES_ICOSAHEDRON,
    }
}

pub fn neighbors_for_solid(solid: &PlatonicSolid) -> Neighbors {
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

/// Tetrahedron: K4
pub const EDGES_TETRAHEDRON: &[(usize, usize)] = &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

/// Cube: bottom square 0–3, top square 4–7
pub const EDGES_CUBE: &[(usize, usize)] = &[
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 4),
    (1, 5),
    (2, 6),
    (3, 7),
];

/// Octahedron: 0 = top, 5 = bottom, 1–4 = equator cycle
pub const EDGES_OCTAHEDRON: &[(usize, usize)] = &[
    (0, 1),
    (0, 2),
    (0, 3),
    (0, 4),
    (5, 1),
    (5, 2),
    (5, 3),
    (5, 4),
    (1, 2),
    (2, 3),
    (3, 4),
    (4, 1),
];

/// Icosahedron: coordinates-based pub construction, labeled 0..11
pub const EDGES_ICOSAHEDRON: &[(usize, usize)] = &[
    (0, 2),
    (0, 4),
    (0, 6),
    (0, 8),
    (0, 10),
    (1, 3),
    (1, 4),
    (1, 6),
    (1, 9),
    (1, 11),
    (2, 5),
    (2, 7),
    (2, 8),
    (2, 10),
    (3, 5),
    (3, 7),
    (3, 9),
    (3, 11),
    (4, 6),
    (4, 8),
    (4, 9),
    (5, 7),
    (5, 8),
    (5, 9),
    (6, 10),
    (6, 11),
    (7, 10),
    (7, 11),
    (8, 9),
    (10, 11),
];

/// Dodecahedron: 20-vertex 3-regular graph, labeled 0..19
pub const EDGES_DODECAHEDRON: &[(usize, usize)] = &[
    (0, 8),
    (0, 12),
    (0, 16),
    (1, 9),
    (1, 12),
    (1, 17),
    (2, 10),
    (2, 13),
    (2, 16),
    (3, 11),
    (3, 13),
    (3, 17),
    (4, 8),
    (4, 14),
    (4, 18),
    (5, 9),
    (5, 14),
    (5, 19),
    (6, 10),
    (6, 15),
    (6, 18),
    (7, 11),
    (7, 15),
    (7, 19),
    (8, 10),
    (9, 11),
    (12, 14),
    (13, 15),
    (16, 17),
    (18, 19),
];
