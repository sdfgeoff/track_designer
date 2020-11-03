use super::mesh::Mesh;

pub fn generate_binary_stl(mesh: &Mesh) -> Vec<u8> {
    
    let mut out = vec![];
    
    // First 80 bytes of binary STL are a header
    for _ in 0..80 {
        out.push(255);
    }
    
    // Put in face count
    out.extend(&(mesh.faces.len() as u32).to_le_bytes());
    
    
    // Export face data
    for face in &mesh.faces {
        
        let v0 = &mesh.vertices[face.0 as usize];
        let v1 = &mesh.vertices[face.1 as usize];
        let v2 = &mesh.vertices[face.2 as usize];
        
        
        let normal = mesh.calc_face_normal(&face);
        
        out.extend(&normal.x.to_le_bytes());
        out.extend(&normal.y.to_le_bytes());
        out.extend(&normal.z.to_le_bytes());
        
        out.extend(&v0.x.to_le_bytes());
        out.extend(&v0.y.to_le_bytes());
        out.extend(&v0.z.to_le_bytes());
        
        out.extend(&v1.x.to_le_bytes());
        out.extend(&v1.y.to_le_bytes());
        out.extend(&v1.z.to_le_bytes());
        
        out.extend(&v2.x.to_le_bytes());
        out.extend(&v2.y.to_le_bytes());
        out.extend(&v2.z.to_le_bytes());
        
        out.extend(&u16::to_le_bytes(0));
    }
    
    out
}

mod tests {
    
    
    #[test]
    fn export_unit_plane() {
        
        use super::*;
        use crate::mesh::{Mesh, Vertex, Face};
        use std::collections::HashMap;
        
        use std::fs;
        let mut original_mesh = Mesh {
            vertices: vec![
                Vertex::new(-1.0, 1.0, 0.0),
                Vertex::new(1.0, 1.0, 0.0),
                Vertex::new(-1.0, -1.0, 0.0),
                Vertex::new(1.0, -1.0, 0.0),
            ],
            faces: vec![Face(0,1,2), Face(1,2,3)],
            vertex_groups: HashMap::default()
        };
        
        let data = generate_binary_stl(&original_mesh);
    
        fs::write("/tmp/plane2.stl", data).expect("Unable to write file");
    }
    
    
    
}
