
use meshtools::mesh::{Mesh, Vertex, Face, VertWeight};

pub fn get_mesh() -> Mesh {
    Mesh {
        vertices: vec![
            Vertex::new(-10.0, -10.0, 0.0),
            Vertex::new(10.0, -10.0, 0.0),
            Vertex::new(-10.0, 10.0, 0.0),
            Vertex::new(10.0, 10.0, 0.0),
            Vertex::new(-10.0, 3.0, 0.0),
            Vertex::new(-10.0, -3.0, 0.0),
            Vertex::new(10.0, -3.0, 0.0),
            Vertex::new(10.0, 3.0, 0.0),
            Vertex::new(-10.0, -1.0, 5.0),
            Vertex::new(-10.0, -4.0, 5.0),
            Vertex::new(10.0, -4.0, 5.0),
            Vertex::new(10.0, -1.0, 5.0)
        ],
        faces: vec![
            Face(7, 2, 4),
            Face(1, 5, 0),
            Face(4, 11, 7),
            Face(10, 8, 9),
            Face(7, 10, 6),
            Face(5, 8, 4),
            Face(6, 9, 5),
            Face(7, 3, 2),
            Face(1, 6, 5),
            Face(4, 8, 11),
            Face(10, 11, 8),
            Face(7, 11, 10),
            Face(5, 9, 8),
            Face(6, 10, 9)
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
                    VertWeight::new(6, 1.0),
                    VertWeight::new(7, 1.0)
                ]
            ),
        ].into_iter().collect()
    }  
}
