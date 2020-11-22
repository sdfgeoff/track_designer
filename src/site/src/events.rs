//! Events represent operations on state and are generated from multiple
//! sources. Events are emitted to allow communication between separate
//! parts of the program
use super::schematic::{TrackPathSegmentDrawing, WheelDrawing};
use tracktool::track_path::{TrackPathSegment, WheelDescription};
use web_sys::MouseEvent;

#[derive(Debug)]
pub enum Event {
    AnimationFrame,

    WheelInsert(WheelDescription),
    WheelDrawingInsert(WheelDrawing),
    TrackPathSegmentInsert(TrackPathSegment),
    TrackPathSegmentDrawingInsert(TrackPathSegmentDrawing),

    WheelChanged(usize),
    TrackPathSegmentChanged(usize),

    SchematicMouseDown(MouseEvent),
    SchematicMouseUp(MouseEvent),
    SchematicMouseMove(MouseEvent),

    VisualizerMouseDown(MouseEvent),
    VisualizerMouseUp(MouseEvent),
    VisualizerMouseMove(MouseEvent),
}
