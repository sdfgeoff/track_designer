use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{HtmlCanvasElement, KeyEvent, MouseEvent};

use super::renderer;

use tracktool::{track, track_surface};
use track_outer_surfaces::TrackOuterSurfaceDescription;
use meshtools::stl::generate_binary_stl;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    renderer: renderer::Renderer,
    
    click_location: Option<(i32, i32)>,
    
}

impl App {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        
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
        let mut renderer = renderer::Renderer::new(canvas).expect("Renderer Failed to Init");

        renderer.add_stl(&stl_data).expect("failed to load stl to renderer");

        Self {
            renderer,
            click_location: None,
        }
    }

    pub fn animation_frame(&mut self) {
        self.renderer.update()
    }
        
    pub fn mouse_move(&mut self, event: MouseEvent) {
        const DRAG_SENSITIVITY: f32 = 5.0;
        match self.click_location {
            Some(location) => {
                
                let new = (event.client_x(), event.client_y());
                let delta = (location.0 - new.0, location.1 - new.1);
                self.click_location = Some(new);
                
                let percentage_x = (delta.0 as f32) / (self.renderer.resolution.0 as f32) * DRAG_SENSITIVITY;
                let percentage_y = (delta.1 as f32) / (self.renderer.resolution.0 as f32) * DRAG_SENSITIVITY;
                
                self.renderer.drag_view((percentage_x, -percentage_y));
            }
            None => {
            }
        }
    }
    pub fn mouse_down(&mut self, event: MouseEvent) {
        self.click_location = Some((event.client_x(), event.client_y()));
    }
    pub fn mouse_up(&mut self, _event: MouseEvent) {
        self.click_location = None;
    }
    
    pub fn key_event(&mut self, event: KeyEvent) {
        log(&format!("Key Event {:?}", event));
    }
}
