//! Entire program state
use super::layout::Layout;
use super::schematic::{Schematic, TrackPathSegmentDrawing, WheelDrawing};
use super::visualizer3d::Visualizer3d;
use tracktool::track_path::{TrackPathSegment, WheelDescription};

/// The current state of the program is stored in this struct.
/// This includes fundamental data (data that is needed to represent
/// the track system) as well as derived data (essentially a cache of
/// fundamental data) and runtime data (UI elements.
///
/// When saving/loading, only part of this struct needs to be saved/loaded.
/// This is an area of possible improvement as it would be nice to be able
/// to derive Serialize on this whole struct.
// TODO: Consider splitting fundamental data out? 
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
