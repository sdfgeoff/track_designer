use std::collections::HashMap;


#[derive(Clone)]
/// A point in space, often used to represent the corner of a triangle
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub type Vertex = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            x,
            y,
            z
        }
    }
    
    pub fn scale(&self, scale: f32) -> Self {
        Self::new(self.x * scale, self.y * scale, self.z * self.z)
    }
}

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
        let angle_a = f32::atan2(pos_a.y, pos_a.z);
        let angle_b = f32::atan2(pos_b.y, pos_b.z);
        
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

/// Returns the squared distance between two points
pub fn dist_squared(p1: &Vertex, p2: &Vertex) -> f32 {
    f32::powi(p1.x - p2.x, 2) + f32::powi(p1.y - p2.y, 2) + f32::powi(p1.z - p2.z, 2)
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

    
    /// Calculates the dimensions of the track. Returns the minimum and maximum bounds
    /// and the length/width/height
    pub fn calc_bounds(&self) -> (Vec3, Vec3, Vec3) {
        let mut min = Vec3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Vec3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY);
        
        for vert in self.vertices.iter() {
            min.x = f32::min(vert.x, min.x);
            min.y = f32::min(vert.y, min.y);
            min.z = f32::min(vert.z, min.z);
            max.x = f32::max(vert.x, max.x);
            max.y = f32::max(vert.y, max.y);
            max.z = f32::max(vert.z, max.z);
        };
        
        let dim = Vec3::new(
            max.x - min.x,
            max.y - min.y,
            max.z - min.z,
        );
        
        (min, max, dim)
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
                if dist_squared(vert, existing_vert) < sq_error {
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
        for vert in self.vertices.iter_mut() {
            vert.x += offset.x;
            vert.y += offset.y;
            vert.z += offset.z;
        }
    }
    
    
    /// Converts from polar to cartesian treating the z axis as radius
    /// and the y axis as the angle
    pub fn bend(&mut self, amount: f32) {
        for vert in self.vertices.iter_mut() {
            let angle = vert.y * amount;
            
            let radius = vert.z;
            let (s, c) = angle.sin_cos();
            
            vert.y = s * radius;
            vert.z = c * radius;
        }
    }
    
    pub fn calc_face_normal(&self, face: &Face) -> Vec3 {
        let vert0 = &self.vertices[face.0 as usize];
        let vert1 = &self.vertices[face.1 as usize];
        let vert2 = &self.vertices[face.2 as usize];
        
        let v1 = Vec3 {
            x: vert0.x - vert1.x,
            y: vert0.y - vert1.y,
            z: vert0.z - vert1.z,
        };
        let v2 = Vec3 {
            x: vert0.x - vert2.x,
            y: vert0.y - vert2.y,
            z: vert0.z - vert2.z,
        };
        
        Vec3 {
            x: v1.y * v2.z,
            y: v1.x * v2.z,
            z: v1.x * v2.y,
        }
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
