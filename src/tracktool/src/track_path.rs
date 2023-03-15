use glam::Vec2;
use super::wheel::WheelDescription;

/// Describes the route that a track takes. Contains no details
/// about the wheels themselves


/// Calculates the path the track will take when joining wheels
pub fn calc_segments(path: &Vec<WheelDescription>) -> Vec<TrackPathSegment> {
    let mut out_vec = vec![];

    if path.len() >= 1 {
        // hardcode dummy data
        out_vec.push(TrackPathSegment {
            line: TrackPathLine {
                start: Vec2::new(-20.0, 4.7),
                end: Vec2::new(20.0, 4.7),
            },
            arc: TrackPathArc {
                center: path[0].position.clone(),
                radius: path[0].radius(),
                start_angle: std::f32::consts::PI / 2.0,
                end_angle: -std::f32::consts::PI / 2.0,
            },
        });
    }
    if path.len() >= 2 {
        out_vec.push(TrackPathSegment {
            line: TrackPathLine {
                start: Vec2::new(-20.0, -4.7),
                end: Vec2::new(20.0, -4.7),
            },
            arc: TrackPathArc {
                center: path[1].position.clone(),
                radius: path[1].radius(),
                start_angle: -std::f32::consts::PI / 2.0,
                end_angle: std::f32::consts::PI / 2.0,
            },
        });
    }

    if path.len() >= 3 {
        unimplemented!();
    }

    out_vec
}

#[derive(Debug)]
pub struct TrackPathSegment {
    pub line: TrackPathLine,
    pub arc: TrackPathArc,
}
impl TrackPathSegment {
    pub fn new() -> Self {
        Self {
            line: TrackPathLine {
                start: Vec2::new(0.0, 0.0),
                end: Vec2::new(0.0, 0.0),
            },
            arc: TrackPathArc {
                center: Vec2::new(0.0, 0.0),
                radius: 1.0,
                start_angle: 0.0,
                end_angle: 1.0,
            },
        }
    }
}

#[derive(Debug)]
pub struct TrackPathLine {
    pub start: Vec2,
    pub end: Vec2,
}

#[derive(Debug)]
pub struct TrackPathArc {
    pub center: Vec2,
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}
