
use meshtools::mesh::{Mesh, Vertex, Face, VertWeight};

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
            Vertex::new(2.0, 2.0, -5.0)
        ],
        faces: vec![
            Face(9, 10, 15),
            Face(13, 2, 4),
            Face(15, 11, 13),
            Face(6, 5, 0),
            Face(12, 4, 5),
            Face(7, 12, 6),
            Face(15, 18, 14),
            Face(1, 14, 7),
            Face(8, 15, 14),
            Face(18, 17, 16),
            Face(14, 16, 12),
            Face(12, 17, 13),
            Face(13, 19, 15),
            Face(9, 3, 10),
            Face(13, 11, 2),
            Face(15, 10, 11),
            Face(6, 12, 5),
            Face(12, 13, 4),
            Face(7, 14, 12),
            Face(15, 19, 18),
            Face(1, 8, 14),
            Face(8, 9, 15),
            Face(18, 19, 17),
            Face(14, 18, 16),
            Face(12, 16, 17),
            Face(13, 17, 19)
        ],
        vertex_groups: vec![

            (
                "edge_left".to_string(), 
                vec![
                    VertWeight::new(0, 1.0),
                    VertWeight::new(2, 1.0),
                    VertWeight::new(4, 1.0),
                    VertWeight::new(5, 1.0)
                ]
            ),

            (
                "edge_right".to_string(), 
                vec![
                    VertWeight::new(1, 1.0),
                    VertWeight::new(3, 1.0),
                    VertWeight::new(8, 1.0),
                    VertWeight::new(9, 1.0)
                ]
            ),
        ].into_iter().collect()
    }  
}
