use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::{window, Element, KeyEvent, MouseEvent, SvgElement};

use super::layout;
use super::visualizer3d;
use super::schematic::Schematic;

use glam::Vec2;
use tracktool::track_path::{TrackPath, WheelDescription};

use meshtools::stl::generate_binary_stl;
use track_outer_surfaces::TrackOuterSurfaceDescription;
use tracktool::{track, track_surface};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    pub visualizer: visualizer3d::Renderer,
    pub schematic: Schematic,
    pub layout: layout::Layout,
    pub path: TrackPath,
}

impl App {
    pub fn new(layout: layout::Layout) -> Self {
        let complete_track = track::generate_track(
            track::TrackDescription {
                outer_surface: TrackOuterSurfaceDescription::Chunky,
                inner_surface: track_surface::TrackInnerSurfaceDescription::Simple,

                belt_thickness_mm: 2.0,
                belt_width_mm: 20.0,

                inner_segment_repeats: 5,
                outer_segment_repeats: 10,
            },
            track::TrackShapeDescription::Loop(100.0),
        );

        let stl_data = generate_binary_stl(&complete_track);
        let mut visualizer = visualizer3d::Renderer::new(layout.visualizer_canvas.clone())
            .expect("Renderer Failed to Init");

        let schematic = Schematic::new(layout.schematic_svg.clone()).expect("Failed to create schematic");
        
        let path = TrackPath::new();

        visualizer
            .add_stl(&stl_data)
            .expect("failed to load stl to visualizer");

        Self {
            visualizer,
            layout,
            schematic,
            path,
        }
    }

    pub fn animation_frame(&mut self) {
        self.visualizer.update();
        self.schematic.update_from_track_path(&self.path);
        
        for (drawing, wheel) in self.schematic.idler_drawings.iter().zip(self.path.path.iter_mut()) {
            let center_position = drawing.get_center_handle_position().expect("arrgh");
            let radius_position = drawing.get_radius_handle_position().expect("arrgh");
            wheel.position = center_position;
            wheel.circumference = (center_position - radius_position).length() * 2.0 * std::f32::consts::PI;
        }
    }

    pub fn load_file(&mut self, file_contents: &[u8]) {
        log(&format!("Loading File"));
    }
}

