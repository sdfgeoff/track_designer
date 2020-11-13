use meshtools::mesh::{Face, Mesh, VertWeight, Vertex};

pub fn get_mesh() -> Mesh {
    Mesh {
        vertices: vec![
            Vertex::new(-10.0, -10.0, 0.0),
            Vertex::new(10.0, -10.0, 0.0),
            Vertex::new(-10.0, 10.0, 0.0),
            Vertex::new(10.0, 10.0, 0.0),
            Vertex::new(-10.0, 4.0, 0.0),
            Vertex::new(-10.0, -4.0, 0.0),
            Vertex::new(-3.0, -10.0, 0.0),
            Vertex::new(3.0, -10.0, 0.0),
            Vertex::new(10.0, -4.0, 0.0),
            Vertex::new(10.0, 4.0, 0.0),
            Vertex::new(3.0, 10.0, 0.0),
            Vertex::new(-3.0, 10.0, 0.0),
            Vertex::new(-3.0, -4.0, 0.0),
            Vertex::new(-3.0, 4.0, 0.0),
            Vertex::new(3.0, -4.0, 0.0),
            Vertex::new(3.0, 4.0, 0.0),
            Vertex::new(-2.0, -2.0, -5.0),
            Vertex::new(-2.0, 2.0, -5.0),
            Vertex::new(2.0, -2.0, -5.0),
            Vertex::new(2.0, 2.0, -5.0),
        ],
        faces: vec![
            Face(10, 9, 15),
            Face(2, 13, 4),
            Face(11, 15, 13),
            Face(5, 6, 0),
            Face(4, 12, 5),
            Face(12, 7, 6),
            Face(18, 15, 14),
            Face(14, 1, 7),
            Face(15, 8, 14),
            Face(17, 18, 16),
            Face(16, 14, 12),
            Face(17, 12, 13),
            Face(19, 13, 15),
            Face(10, 3, 9),
            Face(2, 11, 13),
            Face(11, 10, 15),
            Face(5, 12, 6),
            Face(4, 13, 12),
            Face(12, 14, 7),
            Face(18, 19, 15),
            Face(14, 8, 1),
            Face(15, 9, 8),
            Face(17, 19, 18),
            Face(16, 18, 14),
            Face(17, 16, 12),
            Face(19, 17, 13),
        ],
        vertex_groups: vec![
            (
                "edge_left".to_string(),
                vec![
                    VertWeight::new(0, 1.0),
                    VertWeight::new(2, 1.0),
                    VertWeight::new(4, 1.0),
                    VertWeight::new(5, 1.0),
                ],
            ),
            (
                "edge_right".to_string(),
                vec![
                    VertWeight::new(1, 1.0),
                    VertWeight::new(3, 1.0),
                    VertWeight::new(8, 1.0),
                    VertWeight::new(9, 1.0),
                ],
            ),
        ]
        .into_iter()
        .collect(),
    }
}
