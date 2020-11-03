use std::fs;

use meshtools::mesh::{Vec3, Mesh, sort_vertex_group_radial};
use meshtools::stl::generate_binary_stl;
use meshtools::tools::{generate_vertex_bridge, make_loop, make_array};

use resources::meshes;


fn main() {
    let complete_track = generate_track(
        TrackDescription {
            tread: TreadDescription::Chunky,
            lug: LugDescription::Simple,
            segment_repeats: 10,
            belt_thickness_mm: 2.0,
            shape: TrackShape::Loop,
        }
    );
    
    fs::write("/tmp/track.stl", generate_binary_stl(&complete_track)).expect("Unable to write file");
}


enum TrackShape {
    Loop,
    Straight,
    Path(TrackPath)
}


struct TrackPath {
    Vec<WheelBasicDescription>
}

struct WheelBasicDescription {
    position: Vec3,
    radius: f32
}

enum TreadDescription {
    Simple,
    Chunky
}

impl TreadDescription {
    fn get_mesh(&self) -> Mesh {
        match self {
            TreadDescription::Simple => meshes::tread_simple::get_mesh(),
            TreadDescription::Chunky => meshes::tread_chunky::get_mesh(),
        }
    }
}

enum LugDescription {
    Simple,
}

impl LugDescription {
    fn get_mesh(&self) -> Mesh {
        match self {
            Simple => meshes::simple_lugs::get_mesh(),
        }
    }
}


struct TrackDescription {
    tread: TreadDescription,
    lug: LugDescription,
    belt_thickness_mm: f32,
    segment_repeats: u32,
    shape: TrackShape
}

fn generate_track(track_description: TrackDescription) -> Mesh {

    let mut track = track_description.tread.get_mesh();
    track.linear_offset(Vec3::new(0.0, 0.0, track_description.belt_thickness_mm/2.0));
    let mut lugs = track_description.lug.get_mesh();
    lugs.linear_offset(Vec3::new(0.0, 0.0, -track_description.belt_thickness_mm/2.0));
    
    match track_description.shape {
        TrackShape::Straight => {
            track = make_array(
                &track,
                track_description.segment_repeats,
                Vec3::new(0.0, track.calc_bounds().2.y, 0.0)
            );
            lugs = make_array(
                &lugs,
                track_description.segment_repeats,
                Vec3::new(0.0, lugs.calc_bounds().2.y, 0.0)
            );
        },
        TrackShape::Loop => {
            track = make_loop(&track, track_description.segment_repeats);
            lugs = make_loop(&lugs, track_description.segment_repeats);
        }
    }
    
    
    sort_vertex_group_radial(&lugs.vertices, lugs.vertex_groups.get_mut("edge_right").unwrap());
    sort_vertex_group_radial(&lugs.vertices, lugs.vertex_groups.get_mut("edge_left").unwrap());
    sort_vertex_group_radial(&track.vertices, track.vertex_groups.get_mut("edge_right").unwrap());
    sort_vertex_group_radial(&track.vertices, track.vertex_groups.get_mut("edge_left").unwrap());
    
    
    let track_sidewall_right = generate_vertex_bridge(
        &track,
        &lugs,
        track.vertex_groups.get("edge_right").as_ref().unwrap(),
        lugs.vertex_groups.get("edge_right").as_ref().unwrap()
    );
    let track_sidewall_left = generate_vertex_bridge(
        &track,
        &lugs,
        track.vertex_groups.get("edge_left").as_ref().unwrap(),
        lugs.vertex_groups.get("edge_left").as_ref().unwrap()
    );
    
    let mut complete_track = Mesh::default();
    complete_track.extend(&track);
    complete_track.extend(&lugs);
    complete_track.extend(&track_sidewall_right);
    complete_track.extend(&track_sidewall_left);
    
    let complete_track = complete_track.merge_by_distance(0.01);
    complete_track
}
