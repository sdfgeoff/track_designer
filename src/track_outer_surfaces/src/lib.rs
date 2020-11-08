use meshtools::mesh::Mesh;
use glam::Vec3;

mod meshes;


pub enum TrackOuterSurfaceDescription {
    Simple,
    Chunky
}

// TODO: Figure out how to do configuration more flexibly

impl TrackOuterSurfaceDescription {
    pub fn get_mesh(&self, belt_width_mm: f32, segment_length_mm: f32) -> Mesh {
        match self {
            TrackOuterSurfaceDescription::Simple => {
                let mut m = meshes::tread_simple::get_mesh();
                m.scale(Vec3::new(belt_width_mm, segment_length_mm, belt_width_mm));
                m
            },
            TrackOuterSurfaceDescription::Chunky => {
                let mut m = meshes::tread_chunky::get_mesh();
                m.scale(Vec3::new(belt_width_mm, segment_length_mm, belt_width_mm));
                m
            }
        }
    }
}
