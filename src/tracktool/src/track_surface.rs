use meshtools::mesh::{Mesh};
use resources::meshes;
use glam::Vec3;




pub enum TrackInnerSurfaceDescription {
    Simple,
}


impl TrackInnerSurfaceDescription {
    pub fn get_mesh(&self, belt_width_mm: f32, segment_length_mm: f32) -> Mesh {
        match self {
            TrackInnerSurfaceDescription::Simple => {
                let mut m = meshes::simple_lugs::get_mesh();
                m.scale(Vec3::new(belt_width_mm/20.0, segment_length_mm/20.0, 1.0));
                m
            }
        }
    }
}
