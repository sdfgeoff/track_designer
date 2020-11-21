use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsValue, JsCast};
use web_sys::{window, Element, KeyEvent, MouseEvent, SvgElement, SvgGraphicsElement};

use glam::Vec2;

use tracktool::track_path::{TrackPath, WheelDescription, TrackPathSegment};

mod svg;
mod idler;
mod track_path_segment;
mod handle;

pub use idler::IdlerWheelDrawing;
pub use track_path_segment::TrackPathSegmentDrawing;


pub struct Schematic {
    pub svg: SvgGraphicsElement,
    pub idler_drawings: Vec<IdlerWheelDrawing>,
    pub track_path_segments: Vec<TrackPathSegmentDrawing>,
    pub background: Element,
    
    selected_element: Option<Element>,
}


// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Schematic {    
    pub fn new(svg: SvgElement) -> Result<Self, JsValue> {
        
        let background = svg::create_svg_element("rect")?;
        background.set_attribute("width", "100%")?;
        background.set_attribute("height", "100%")?;
        background.set_attribute("x", "-50%")?;
        background.set_attribute("y", "-50%")?;
        background.set_attribute("fill", "white")?;
        svg.append_child(&background)?;
        
        Ok(Self {
            svg: svg.dyn_into()?,
            background,
            idler_drawings: vec!(),
            track_path_segments: vec!(),
            selected_element: None,
        })
    }
    
    pub fn update_from_track_path(&mut self, track_path: &TrackPath) -> Result<(), JsValue> {
        self.update_wheels(&track_path.path)?;
        self.update_path_segments(&track_path.calc_segments())?;
        self.update_bounding_box(&track_path.path)?;
            
        Ok(())
        
    }
    
    pub fn update_wheels(&mut self, wheels: &Vec<WheelDescription>) -> Result<(), JsValue> {
        // Ensure there are enough objects to display the wheels
        while wheels.len() > self.idler_drawings.len() {
            self.idler_drawings.push(IdlerWheelDrawing::new(&self.svg)?);
        }
        while wheels.len() < self.idler_drawings.len() {
            self.idler_drawings.pop().unwrap().delete()?; // Unwrap is safe due to loop conditional
        }
        
        // Update each wheel entity
        for (wheel, drawing) in wheels.iter().zip(self.idler_drawings.iter_mut()) {
            drawing.update_from_wheel_description(wheel)?;
        }
        
        Ok(())
    }
    
    pub fn update_path_segments(&mut self, segments: &Vec<TrackPathSegment>) -> Result<(), JsValue> {
        while segments.len() > self.track_path_segments.len() {
            self.track_path_segments.push(TrackPathSegmentDrawing::new(&self.svg)?);
        }
        while segments.len() < self.track_path_segments.len() {
            self.track_path_segments.pop().unwrap().delete()?; // Unwrap is safe due to loop conditional
        }
        
        // Update each entity
        for (segment, drawing) in segments.iter().zip(self.track_path_segments.iter_mut()) {
            drawing.update_from_track_path_segment(segment)?;
        }
        Ok(())
    }
    
    pub fn update_bounding_box(&mut self, wheels: &Vec<WheelDescription>) -> Result<(), JsValue> {
        // Ensure bounding box of image
        let mut min_vec = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max_vec = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);
    
        for wheel in wheels.iter() {
            let radius = wheel.radius();
            let rad_vec = Vec2::new(radius, radius);
            
            min_vec = min_vec.min(wheel.position - rad_vec);
            max_vec = max_vec.max(wheel.position + rad_vec);
        }
        
        let top_left = min_vec;
        let dimensions = max_vec - min_vec;
        let padding = dimensions[0] * 0.1;

        self.svg.set_attribute(
            "viewBox",
            &format!(
                "{} {} {} {}",
                top_left[0] - padding,
                top_left[1] - padding,
                dimensions[0] + padding,
                dimensions[1] + padding,
            ),
        )?;
        Ok(())
    }
    
    
    pub fn mouse_down(&mut self, evt: MouseEvent) {
        if let Some(target) = evt.target() {
            self.selected_element = Some(target.dyn_into().expect("Not Right Element"));
        }
    }
    pub fn mouse_up(&mut self, evt: MouseEvent) {
        self.selected_element = None;
    }
    pub fn mouse_move(&mut self, evt: MouseEvent) {
        if let Some(elem) = &self.selected_element {
            //let graphics_elem: SvgGraphicsElement = self.svg.dyn_into().expect("Err");
            let ctm = self.svg.get_screen_ctm().unwrap();
            
            let x = (evt.client_x() as f32 - ctm.e()) / ctm.a();
            let y = (evt.client_y() as f32 - ctm.f()) / ctm.d();
            
            elem.set_attribute("cx", &format!("{}", x)).expect("Failed to Move");
            elem.set_attribute("cy", &format!("{}", y)).expect("Failed to Move");
            log(&format!("{} {} {:?}", x, y, elem));
        }
        
    }
}

