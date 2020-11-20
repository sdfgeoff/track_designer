use glam::Vec2;

/// Describes the route that a track takes. Contains no details
/// about the wheels themselves

pub struct TrackPath {
    pub path: Vec<WheelDescription>,
}

pub struct WheelDescription {
    pub position: Vec2,
    pub circumference: f32,
}

impl WheelDescription {
    pub fn radius(&self) -> f32 {
        self.circumference / (2.0 * std::f32::consts::PI)
    }
}

impl TrackPath {
    pub fn length(&self) -> f32 {
        unimplemented!();
    }

    pub fn new() -> Self {
        Self {
            path: vec![
                WheelDescription {
                    position: Vec2::new(-20.0, 0.0),
                    circumference: 30.0,
                },
                WheelDescription {
                    position: Vec2::new(20.0, 0.0),
                    circumference: 30.0,
                },
            ],
        }
    }

    /// Calculates the path the track will take when joining wheels
    pub fn calc_segments(&self) -> Vec<TrackPathSegment> {
        let mut out_vec = vec![];

        // hardcode dummy data
        out_vec.push(TrackPathSegment {
            line: TrackPathLine {
                start: Vec2::new(-20.0, 4.7),
                end: Vec2::new(20.0, 4.7),
            },
            arc: TrackPathArc {
                center:Vec2::new(-20.0, 0.0),
                radius: 4.7,
                start_angle: std::f32::consts::PI/2.0,
                end_angle: -std::f32::consts::PI/2.0,
            }
        });
        
        out_vec.push(TrackPathSegment {
            line: TrackPathLine {
                start: Vec2::new(-20.0, -4.7),
                end: Vec2::new(20.0, -4.7),
            },
            arc: TrackPathArc {
                center:Vec2::new(20.0, 0.0),
                radius: 4.7,
                start_angle: -std::f32::consts::PI/2.0,
                end_angle: std::f32::consts::PI/2.0,
            }
        });

        out_vec
    }
}

pub struct TrackPathSegment {
    pub line: TrackPathLine,
    pub arc: TrackPathArc,
}

pub struct TrackPathLine {
    pub start: Vec2,
    pub end: Vec2,
}

pub struct TrackPathArc {
    pub center: Vec2,
    pub radius: f32,
    pub start_angle: f32,
    pub end_angle: f32,
}
