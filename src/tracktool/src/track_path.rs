use glam::Vec2;

/// Describes the route that a track takes. Contains no details
/// about the wheels themselves

pub struct TrackPath {
    pub segment_length: f32,
    pub path: Vec<WheelBaseDescription>,
}

pub struct WheelBaseDescription {
    position: Vec2,
    radius: f32,
}

impl TrackPath {
    pub fn length(&self) -> f32 {
        unimplemented!();
    }
}
