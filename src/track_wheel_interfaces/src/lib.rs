use glam::Vec3;
use meshtools::mesh::Mesh;

mod meshes;

pub enum TrackWheelInterfaces {
    Flat,
    //Toothed,
    //VBelt,
    //Lugs,
}


pub struct TrackWheelInterface {
    name: String,
    description: String,
    drive_sprocket_meshes: Vec<ConfigurableMesh>,
    idler_meshes: Vec<ConfigurableMesh>,
}

trait ConfigurableMesh {
    fn generate_mesh(&self, Vec<ConfigurationOption>) -> Mesh {
        
    }
    
    fn get_configuration_options(&self) -> Vec<ConfigurationOption> {
        
    }
}




pub enum ConfigurationOption {
    Float(ConfigurableFloat)
    Int(ConfigurableInt)
}

pub struct ConfigurableFloat {
    name: String,
    min: f32,
    max: f32,
    default: f32,
    value: f32,
}
pub struct ConfigurableInt {
    name: String,
    min: i32,
    max: i32,
    default: i32,
    value: i32,
}
