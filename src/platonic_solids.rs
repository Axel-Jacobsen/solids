use strum::{Display, EnumIter};

pub type VertexId = usize;
pub type Neighbors = std::collections::HashMap<VertexId, Vec<VertexId>>;

#[derive(Debug, Display, EnumIter)]
pub enum PlatonicSolid {
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

pub fn faces_for_solid(solid: &PlatonicSolid) -> Vec<Vec<VertexId>> {
    match solid {
        PlatonicSolid::Tetrahedron => {
            vec![vec![0, 1, 2], vec![0, 3, 1], vec![0, 2, 3], vec![1, 3, 2]]
        }

        PlatonicSolid::Cube => vec![
            vec![0, 3, 2, 1],
            vec![0, 1, 7, 6],
            vec![0, 6, 5, 3],
            vec![4, 2, 3, 5],
            vec![4, 7, 1, 2],
            vec![4, 5, 6, 7],
        ],

        PlatonicSolid::Octahedron => vec![
            vec![0, 1, 5],
            vec![1, 3, 5],
            vec![3, 4, 5],
            vec![0, 5, 4],
            vec![0, 2, 1],
            vec![1, 2, 3],
            vec![3, 2, 4],
            vec![0, 4, 2],
        ],

        PlatonicSolid::Dodecahedron => vec![
            vec![0, 13, 11, 1, 3],
            vec![0, 3, 2, 8, 10],
            vec![0, 10, 18, 12, 13],
            vec![1, 4, 7, 2, 3],
            vec![1, 11, 14, 5, 4],
            vec![2, 7, 9, 6, 8],
            vec![5, 15, 9, 7, 4],
            vec![5, 14, 17, 19, 15],
            vec![6, 16, 18, 10, 8],
            vec![6, 9, 15, 19, 16],
            vec![12, 17, 14, 11, 13],
            vec![12, 18, 16, 19, 17],
        ],

        PlatonicSolid::Icosahedron => vec![
            vec![0, 11, 5],
            vec![0, 5, 1],
            vec![0, 1, 7],
            vec![0, 7, 10],
            vec![0, 10, 11],
            vec![1, 5, 9],
            vec![5, 11, 4],
            vec![11, 10, 2],
            vec![10, 7, 6],
            vec![7, 1, 8],
            vec![3, 9, 4],
            vec![3, 4, 2],
            vec![3, 2, 6],
            vec![3, 6, 8],
            vec![3, 8, 9],
            vec![4, 9, 5],
            vec![2, 4, 11],
            vec![6, 2, 10],
            vec![8, 6, 7],
            vec![9, 8, 1],
        ],
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

pub const EDGES_TETRAHEDRON: &[(usize, usize)] = &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)];

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
