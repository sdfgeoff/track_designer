// Rules: No types in types - only options of types and vecs of types

use super::layout::Layout;
use super::schematic::{Schematic, TrackPathSegmentDrawing, WheelDrawing};
use super::visualizer3d::Visualizer3d;
use tracktool::track_path::{TrackPathSegment, WheelDescription};

pub struct State {
    // Fundamental Data
    // This data is needed to recreate the setup, and should be saved
    // and loaded with the file.
    pub wheel_descriptions: Vec<WheelDescription>,

    // Derived Data
    // This data is derived from the fundamental data and is a cache of
    // computation. It could be saved but it can also just be regenerated
    // when needed.
    pub track_path_segments: Vec<TrackPathSegment>,

    // Runtime Data
    // This data contains UI elements etc. It cannot be saved as it
    // contains pointers to shared memory, GPU objects etc.
    pub wheel_drawings: Vec<WheelDrawing>,
    pub track_path_segment_drawings: Vec<TrackPathSegmentDrawing>,
    pub layout: Layout,
    pub schematic: Schematic,
    pub visualizer: Visualizer3d,
}
