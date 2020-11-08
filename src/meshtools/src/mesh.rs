use std::collections::HashMap;
use glam::Vec3;


/// A point in space, often used to represent the corner of a triangle
pub type Vertex = Vec3;


/// A pointer at a vertex
pub type VertexIndex = u32;

#[derive(Debug, PartialEq, Clone)]
pub struct Face(pub VertexIndex, pub VertexIndex, pub VertexIndex);

#[derive(Clone)]
pub struct VertWeight {
    pub vert_index: VertexIndex,
    pub weight: f32,
}

pub type VertexGroup = Vec<VertWeight>;

pub fn sort_vertex_group_radial(vertices: &Vec<Vertex>, vert_group: &mut VertexGroup) {
    vert_group.sort_by(|a, b| {
        let pos_a = &vertices[a.vert_index as usize];
        let pos_b = &vertices[b.vert_index as usize];
        let angle_a = f32::atan2(pos_a[1], pos_a[2]);
        let angle_b = f32::atan2(pos_b[1], pos_b[2]);
        
        if angle_a > angle_b {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    
    vert_group.push(
        vert_group.first().expect("Tried to sort empty vertex group").clone()
    ); // So the group is a loop
}


impl VertWeight {
    pub fn new(vert_index: VertexIndex, weight: f32) -> Self {
        Self {
            vert_index,
            weight
        }
    }
}


#[derive(Default, Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>,
    pub vertex_groups: HashMap<String, VertexGroup>,
}


pub fn offset_vert_group(group: &VertexGroup, offset: u32) -> VertexGroup {
    group.iter().map(|x| VertWeight::new(x.vert_index + offset, x.weight)).collect()
}


impl Mesh {
    /// Joins another mesh into this mesh by appending vertices and offsetting
    /// indices. Vertex groups with the same name are merged.
    /// Returns the amount that `other`'s verticies have been offset by.
    pub fn extend(&mut self, other: &Self) -> u32{
        let vert_offset = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&other.vertices);
    
        for face in &other.faces {
            self.faces.push(Face(
                face.0 + vert_offset,
                face.1 + vert_offset,
                face.2 + vert_offset,
            ))
        }
        
        for (name, vertex_group) in other.vertex_groups.iter() {
            
            match self.vertex_groups.get_mut(name) {
                Some(existing_group) => {
                    existing_group.extend(offset_vert_group(vertex_group, vert_offset));
                },
                None => {
                    self.vertex_groups.insert(name.to_string(), offset_vert_group(vertex_group, vert_offset));
                }
            }
        }
        
        vert_offset
    }
    
    pub fn scale(&mut self, scale: Vec3) {
        println!("scale: {:?}", scale);
        for mut vert in self.vertices.iter_mut() {
            *vert *= scale;
        }
    }

    
    /// Calculates the dimensions of the track. Returns the minimum and maximum bounds
    /// and the length/width/height
    pub fn calc_bounds(&self) -> (Vec3, Vec3, Vec3) {
        let mut min_vec = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max_vec = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        
        for vert in self.vertices.iter() {
            min_vec = min_vec.min(*vert);
            max_vec = max_vec.max(*vert);
        };
        
        let dim = max_vec - min_vec;
        
        (min_vec, max_vec, dim)
    }
    
    /// Merge nearby vertices. Note this is O(n^2) operation.
    /// TODO: Note: Destroys vertex groups.
    pub fn merge_by_distance(&self, distance: f32) -> Self {
        
        let mut index_remap: HashMap<VertexIndex, VertexIndex> = HashMap::new();
        
        let sq_error = distance.powi(2);
        
        let mut new_vertices = vec![];
        
        for (index, vert) in self.vertices.iter().enumerate() {
            let mut merged = false;
            for (existing_index, existing_vert) in new_vertices.iter().enumerate() {
                if (*vert - *existing_vert).length_squared() < sq_error {
                    merged = true;
                    index_remap.insert(index as VertexIndex, existing_index as VertexIndex); //.expect_none("Overwriting vert");
                    break;
                }
            }
            if !merged {
                new_vertices.push(vert.clone());
                index_remap.insert(index as VertexIndex, (new_vertices.len() - 1)  as VertexIndex); //.expect_none("Overwriting vert");
            }
        }
        
        let mut faces = vec![];
        
        for face in self.faces.iter() {
            
            let v1 = index_remap.get(&(face.0 as VertexIndex)).expect("Index remap missing vertex").clone();
            let v2 = index_remap.get(&(face.1 as VertexIndex)).expect("Index remap missing vertex").clone();
            let v3 = index_remap.get(&(face.2 as VertexIndex)).expect("Index remap missing vertex").clone();
            
            if v1 != v2 && v2 != v3 && v1 != v2 {
                faces.push(Face(
                    v1,
                    v2,
                    v3
                ));
            }
        }
        
        let vertex_groups = HashMap::new();
        
        Mesh {
            vertices: new_vertices,
            faces: faces,
            vertex_groups: vertex_groups,
        }
        
    }
    
    /// Move all the vertices by the specified vector
    pub fn linear_offset(&mut self, offset: Vec3) {
        for mut vert in self.vertices.iter_mut() {
            *vert += offset;
        }
    }
    
    
    /// Converts from polar to cartesian treating the z axis as radius
    /// and the y axis as the angle
    pub fn bend(&mut self, amount: f32) {
        for vert in self.vertices.iter_mut() {
            let angle = vert[1] * amount;
            
            let radius = vert[2];
            let (s, c) = angle.sin_cos();
            
            vert[1] = s * radius;
            vert[2] = c * radius;
        }
    }
    
    pub fn calc_face_normal(&self, face: &Face) -> Vec3 {
        let vert0 = self.vertices[face.0 as usize];
        let vert1 = self.vertices[face.1 as usize];
        let vert2 = self.vertices[face.2 as usize];
        
        let v1 = vert0 - vert1;
        let v2 = vert0 - vert2;
        
        let normal = v1.cross(v2);
        
        normal
    }
}




mod tests {
    #[test]
    fn extend_empty_mesh() {
        use super::*;
        use std::collections::HashMap;
        
        let mut blank_mesh = Mesh::default();
        
        let test_mesh = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(1.0, 0.0, 0.0),
                Vertex::new(2.0, 0.0, 0.0),
            ],
            faces: vec![Face(0,1,2)],
            vertex_groups: HashMap::default()
        };
        
        assert!(blank_mesh.extend(&test_mesh) == 0);
        assert!(blank_mesh.faces == vec![Face(0,1,2)]);
    }
    
    #[test]
    fn extend_mesh() {
        use super::*;
        use std::collections::HashMap;
        let mut original_mesh = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(1.0, 0.0, 0.0),
                Vertex::new(2.0, 0.0, 0.0),
            ],
            faces: vec![Face(0,1,2)],
            vertex_groups: HashMap::default()
        };
        
        let test_mesh = Mesh {
            vertices: vec![
                Vertex::new(0.0, 0.0, 0.0),
                Vertex::new(1.0, 0.0, 0.0),
                Vertex::new(2.0, 0.0, 0.0),
            ],
            faces: vec![Face(0,1,2)],
            vertex_groups: HashMap::default()
        };
        
        assert!(original_mesh.extend(&test_mesh) == 3);
        
        println!("{:?}", original_mesh.faces);
        assert!(original_mesh.faces == vec![Face(0,1,2), Face(3,4,5)]);
        assert!(original_mesh.vertices.len() == 6);
        
    }
}
