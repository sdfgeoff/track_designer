// Rules: No types in types - only options of types and vecs of types

use tracktool::track_path::{TrackPathSegment, WheelDescription};
use super::schematic::{Schematic, TrackPathSegmentDrawing, IdlerWheelDrawing};



#[derive(Default)] 
pub struct State {
    wheel_descriptions: Vec<WheelDescription>,
    track_path_segments: Vec<TrackPathSegment>,
    
    track_path_segment_drawings: Vec<TrackPathSegmentDrawing>,
    idler_drawings: Vec<IdlerWheelDrawing>,
    
    schematic: Option<Schematic>  
}

