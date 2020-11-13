use meshtools::mesh::{Mesh, sort_vertex_group_radial};
use meshtools::tools::{generate_vertex_bridge, make_array};
use glam::Vec3;


use super::track_path::TrackPath;
use super::track_surface;
use track_outer_surfaces::TrackOuterSurfaceDescription;


/// All the information needed to generate a track mesh. This is a
/// complete, self-contained description of a track.
pub struct TrackDescription {
    /// A pattern with which to decorate the outside of the track.
    /// This is the tread which is in contact with the ground.
    pub outer_surface: TrackOuterSurfaceDescription,
    
    /// The pattern that connects the track with the wheels and
    /// engages with the drive wheel
    pub inner_surface: track_surface::TrackInnerSurfaceDescription,
    
    /// The thickness of the track ignoring the inner and outer surface patterns, 
    pub belt_thickness_mm: f32,
    
    /// The belt width
    pub belt_width_mm: f32,
    
    /// How many times the inner surface repeats itself in the length of
    /// the tracks
    pub inner_segment_repeats: u32,
    
    /// The tread around the outside doesn't need to have the same
    /// length constraints as the inner. It's suggested that this is an
    /// integer multiple or integer fractional of the inner_segment_repeats
    /// variable.
    pub outer_segment_repeats: u32,
}




/// What path/shape the track should follow
pub enum TrackShapeDescription {
    /// Track is in a single band. Parameter is the circumference
    Loop(f32),
    
    /// Track is cut/unrolled into a line. Parameter is the length
    Straight(f32),
    
    /// Track follows a system of wheels.
    Path(TrackPath)
}



impl TrackShapeDescription {
    pub fn length_mm(&self) -> f32 {
        match self {
            TrackShapeDescription::Loop(length) => *length,
            TrackShapeDescription::Straight(length) => *length,
            TrackShapeDescription::Path(path) => path.length(),
        }
    }
}





pub fn generate_track(track_description: TrackDescription, shape: TrackShapeDescription) -> Mesh {

    let length_mm = shape.length_mm();
    let outer_segment_length_mm = length_mm / (track_description.outer_segment_repeats as f32);
    let inner_segment_length_mm = length_mm / (track_description.inner_segment_repeats as f32);

    let mut outer_surface = track_description.outer_surface.get_mesh(track_description.belt_width_mm, outer_segment_length_mm);
    outer_surface.linear_offset(Vec3::new(0.0, 0.0, track_description.belt_thickness_mm/2.0));
    let mut inner_surface = track_description.inner_surface.get_mesh(track_description.belt_width_mm, inner_segment_length_mm);
    inner_surface.linear_offset(Vec3::new(0.0, 0.0, -track_description.belt_thickness_mm/2.0));
    
    
    outer_surface = make_array(
        &outer_surface,
        track_description.outer_segment_repeats,
        Vec3::new(0.0, outer_segment_length_mm, 0.0)
    );
    inner_surface = make_array(
        &inner_surface,
        track_description.inner_segment_repeats,
        Vec3::new(0.0, inner_segment_length_mm, 0.0)
    );
    
    match shape {
        TrackShapeDescription::Straight(_) => {
            // It's already straight
        },
        TrackShapeDescription::Loop(_) => {
            let circumference = length_mm;
            let radius_mm = circumference / (2.0 * std::f32::consts::PI);
            outer_surface.linear_offset(Vec3::new(0.0, 0.0, radius_mm));
            inner_surface.linear_offset(Vec3::new(0.0, 0.0, radius_mm));
            
            inner_surface.bend(2.0 * std::f32::consts::PI / circumference);
            outer_surface.bend(2.0 * std::f32::consts::PI / circumference);
            
        }
        TrackShapeDescription::Path(path) => {
            unimplemented!()
        }
    }
    
    
    sort_vertex_group_radial(&outer_surface.vertices, outer_surface.vertex_groups.get_mut("edge_right").unwrap());
    sort_vertex_group_radial(&outer_surface.vertices, outer_surface.vertex_groups.get_mut("edge_left").unwrap());
    sort_vertex_group_radial(&inner_surface.vertices, inner_surface.vertex_groups.get_mut("edge_right").unwrap());
    sort_vertex_group_radial(&inner_surface.vertices, inner_surface.vertex_groups.get_mut("edge_left").unwrap());
    
    
    let track_sidewall_right = generate_vertex_bridge(
        &outer_surface,
        &inner_surface,
        outer_surface.vertex_groups.get("edge_right").as_ref().expect("Track outer Surface missing edge_right vertex group"),
        inner_surface.vertex_groups.get("edge_right").as_ref().expect("Track inner Surface missing edge_left vertex group")
    );
    let track_sidewall_left = generate_vertex_bridge(
        &inner_surface,
        &outer_surface,
        inner_surface.vertex_groups.get("edge_left").as_ref().expect("Track inner Surface missing edge_left vertex group"),
        outer_surface.vertex_groups.get("edge_left").as_ref().expect("Track outer Surface missing edge_left vertex group")
    );
    
    let mut complete_track = Mesh::default();
    complete_track.extend(&outer_surface);
    complete_track.extend(&inner_surface);
    complete_track.extend(&track_sidewall_right);
    complete_track.extend(&track_sidewall_left);
    
    let complete_track = complete_track.merge_by_distance(0.01);
    complete_track
}
