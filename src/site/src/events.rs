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
