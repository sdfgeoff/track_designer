
struct TrackSystemDescription {
    drive_wheel: DriveWheel
    idlers: Vec<Idler>
    track: TrackDescription,
}


struct DriveWheel {
    position: Vec2,
    hub: HubDesign,
    number_segments: u16
}

struct Idler {
    position: Vec2,
    hub: HubDesign,
    radius: u16
}

struct TrackDescription {
    number_segments: u16,
    
    track_inner_surface: ConfiguredMesh,
    track_outer_surface: ConfiguredMesh,
    
    idler_wheel_outer_surface: ConfiguredMesh,
    drive_wheel_outer_surface: ConfiguredMesh,
}


struct HubDesign {
    mesh: ConfiguredMesh
}

struct ConfiguredMesh {
    mesh: Mesh,
    configuration: HashMap
}
