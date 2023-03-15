use glam::Vec2;

#[derive(Debug)]
pub struct WheelDescription {
    pub position: Vec2,
    pub circumference: f32,
}

impl WheelDescription {
    pub fn radius(&self) -> f32 {
        self.circumference / (2.0 * std::f32::consts::PI)
    }
}
