/// Tools are things that take in one mesh and output another new mesh.
use super::mesh::{Mesh, Face, VertexGroup, VertWeight, Vertex, Vec3, dist_squared};
use super::modifiers;

use std::collections::HashMap;



/// Returns a new mesh with lots of duplicates spaced by the specified
/// distance
pub fn make_array(mesh: &Mesh, count:u32, offset: Vec3) -> Mesh {
    let mut array = Mesh::default();
    for i in 0..count {
        let mut next = mesh.clone();
        next.linear_offset(offset.scale(i as f32));
        array.extend(&next);
    }
    array
}



pub fn generate_vertex_bridge(mesh1: &Mesh, mesh2: &Mesh, vertex_group_1: &VertexGroup, vertex_group_2: &VertexGroup) -> Mesh {
    let mut verts = Vec::with_capacity(vertex_group_1.len() + vertex_group_2.len());
    
    
    let loop_1 = {
        let mut l = VertexGroup::new();
        for vert in vertex_group_1.iter() {
            l.push(VertWeight{
                vert_index: verts.len() as u32, 
                weight: vert.weight
            });
            verts.push(mesh1.vertices[vert.vert_index as usize].clone());
        }
        l
    };
      
    let loop_2 = {
        let mut l = VertexGroup::new();
        
        for vert in vertex_group_2.iter() {
            l.push(VertWeight {
                vert_index: verts.len() as u32, 
                weight: vert.weight
            });
            verts.push(mesh2.vertices[vert.vert_index as usize].clone());
        }
        l
    };
    
    let mut faces = Vec::new();
    
    let mut p1 = 1;
    let mut p2 = 1;
    
    
    loop {
        let vert_1 = &verts[loop_1[p1].vert_index as usize];
        let vert_2 = &verts[loop_2[p2].vert_index as usize];
        
        let vert_2_next = &verts[loop_2[p2 + 1].vert_index as usize];

        let dist_here = dist_squared(&vert_1, &vert_2);
        let dist_next = dist_squared(&vert_1, &vert_2_next);
        

        if dist_here < dist_next {
            faces.push(Face(
                loop_1[p1-1].vert_index,
                loop_1[p1].vert_index,
                loop_2[p2-1].vert_index
            ));
            p1 += 1;
            
        } else {
            faces.push(Face(
                loop_1[p1-1].vert_index,
                loop_2[p2].vert_index,
                loop_2[p2-1].vert_index
            ));
            p2 += 1;
        }
        
        if p1 == loop_1.len() - 1 {
            faces.push(Face(
                loop_1[p1-1].vert_index,
                loop_1[p1].vert_index,
                loop_2[p2-1].vert_index
            ));
            
            while p2 < loop_2.len() {
                faces.push(Face(
                    loop_1[p1].vert_index,
                    loop_2[p2].vert_index,
                    loop_2[p2-1].vert_index
                ));
                p2 += 1;
            }
            break
        }
        
        if p2 == loop_2.len() - 1 {
            faces.push(Face(
                loop_1[p1-1].vert_index,
                loop_2[p2].vert_index,
                loop_2[p2-1].vert_index
            ));
            
            while p1 < loop_1.len() {
                faces.push(Face(
                    loop_1[p1 - 1].vert_index,
                    loop_1[p1].vert_index,
                    loop_2[p2].vert_index
                ));
                p1 += 1;
            }
            break
        }
    }
    
    Mesh {
        vertices: verts,
        faces: faces,
        vertex_groups: HashMap::new(),
    }
}


/// Makes a loop with the origin at the center)
pub fn make_loop(mesh: &Mesh, duplicates: u32) -> Mesh {
    let (_, _, dim) = mesh.calc_bounds();
    
    
    let circumference = dim.y * (duplicates as f32);
    let radius = circumference / (2.0 * std::f32::consts::PI);
    
    let mut loop_mesh = make_array(&mesh, duplicates, Vec3::new(0.0, dim.y, 0.0));
    loop_mesh.linear_offset(Vec3::new(0.0, 0.0, radius));
    
    let length = dim.y;
    
    loop_mesh.bend(2.0 * std::f32::consts::PI / (length * (duplicates as f32)));
    
    loop_mesh
}


mod tests{
    #[test]
    fn test_bridge_simple() {
        use std::fs;
        use super::*;
        use std::collections::HashMap;
        
        let mesh1 = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(0.1, 0.0, 0.0),
                Vertex::new(0.2, 0.0, 0.0),
                Vertex::new(0.3, 0.0, 0.0),
                Vertex::new(0.4, 0.0, 0.0),
                Vertex::new(0.5, 0.0, 0.0),
                Vertex::new(0.6, 0.0, 0.0),
            ],
            faces: vec![],
            vertex_groups: vec![
                (
                    "edge_right".to_string(), 
                    vec![
                        VertWeight::new(0, 1.0),
                        VertWeight::new(1, 1.0),
                        VertWeight::new(2, 1.0),
                        VertWeight::new(3, 1.0),
                        VertWeight::new(4, 1.0),
                        VertWeight::new(5, 1.0),
                        VertWeight::new(6, 1.0),
                    ]
                )
            ].into_iter().collect()
        };
        
        let mesh2 = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 1.0),
                Vertex::new(0.1, 0.0, 1.0),
                Vertex::new(0.2, 0.0, 1.0),
                Vertex::new(0.3, 0.0, 1.0),
                Vertex::new(0.4, 0.0, 1.0),
                Vertex::new(0.5, 0.0, 1.0),
                Vertex::new(0.6, 0.0, 1.0),
            ],
            faces: vec![],
            vertex_groups: vec![
                (
                    "edge_right".to_string(), 
                    vec![
                        VertWeight::new(0, 1.0),
                        VertWeight::new(1, 1.0),
                        VertWeight::new(2, 1.0),
                        VertWeight::new(3, 1.0),
                        VertWeight::new(4, 1.0),
                        VertWeight::new(5, 1.0),
                        VertWeight::new(6, 1.0),
                    ]
                )
            ].into_iter().collect()
        };
        
        
        let bridge = generate_vertex_bridge(
            &mesh1, 
            &mesh2,
            mesh1.vertex_groups.get("edge_right").as_ref().unwrap(),
            mesh2.vertex_groups.get("edge_right").as_ref().unwrap()
        );
        
        let data = crate::stl::generate_binary_stl(&bridge);
        fs::write("/tmp/foo.stl", data).expect("Unable to write file");
    }
    
    
    #[test]
    fn test_bridge_triangle_1() {
        use std::fs;
        use super::*;
        use std::collections::HashMap;
        
        let mesh1 = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(1.0, 0.0, 0.0),
                Vertex::new(2.0, 0.0, 0.0),
            ],
            faces: vec![],
            vertex_groups: vec![
                (
                    "edge_right".to_string(), 
                    vec![
                        VertWeight::new(0, 1.0),
                        VertWeight::new(1, 1.0),
                        VertWeight::new(2, 1.0),
                    ]
                )
            ].into_iter().collect()
        };
        
        let mesh2 = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 1.0),
                Vertex::new(1.0, 0.0, 1.0),
                Vertex::new(2.0, 0.0, 1.0),
            ],
            faces: vec![],
            vertex_groups: vec![
                (
                    "edge_right".to_string(), 
                    vec![
                        VertWeight::new(0, 1.0),
                        VertWeight::new(1, 1.0),
                        VertWeight::new(2, 1.0),
                    ]
                )
            ].into_iter().collect()
        };
        
        
        let bridge = generate_vertex_bridge(
            &mesh1, 
            &mesh2,
            mesh1.vertex_groups.get("edge_right").as_ref().unwrap(),
            mesh2.vertex_groups.get("edge_right").as_ref().unwrap()
        );
        
        let data = crate::stl::generate_binary_stl(&bridge);
        fs::write("/tmp/simple.stl", data).expect("Unable to write file");
    }
}
