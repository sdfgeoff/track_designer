pub struct DriveWheelDescription {
    pub outer_surface: DriveWheelSurfaceDescription,
    pub hub: WheelHubDescription,
    
    pub outer_surface_segment_count: u32,
    pub circumference: f32,
}

pub struct IdlerWheelDescription {
    pub outer_surface: IdlerWheelSurfaceDescription,
    pub hub: WheelHubDescription,
    
    pub outer_surface_segment_count: u32,
    pub circumference: f32,
}



pub enum DriveWheelSurfaceDescription {
    Simple,
}


pub enum IdlerWheelSurfaceDescription {
    Simple,
}


impl DriveWheelSurfaceDescription {
    pub fn to_mesh(&self, belt_width_mm: f32, segment_length: f32) -> Mesh {
        unimplemented!();
    }
}


pub enum WheelHubDescription {
    Hole,
    DShaft,
}
