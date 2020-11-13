use meshtools::stl::generate_binary_stl;
use std::fs;

use track_outer_surfaces::TrackOuterSurfaceDescription;

mod descriptors;
mod track;
mod track_path;
mod track_surface;

fn main() {
    let complete_track = track::generate_track(
        track::TrackDescription {
            outer_surface: TrackOuterSurfaceDescription::Chunky,
            inner_surface: track_surface::TrackInnerSurfaceDescription::Simple,

            belt_thickness_mm: 5.0,
            belt_width_mm: 20.0,

            inner_segment_repeats: 5,
            outer_segment_repeats: 10,
        },
        track::TrackShapeDescription::Loop(100.0),
    );

    fs::write("/tmp/track.stl", generate_binary_stl(&complete_track))
        .expect("Unable to write file");
}
