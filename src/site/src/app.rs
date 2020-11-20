use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::{window, Element, KeyEvent, MouseEvent, SvgElement};

use super::layout;
use super::visualizer3d;

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
    visualizer: visualizer3d::Renderer,
    schematic: Schematic,
    pub layout: layout::Layout,
    click_location: Option<(i32, i32)>,
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

        let schematic = Schematic::new(layout.schematic_svg.clone());

        visualizer
            .add_stl(&stl_data)
            .expect("failed to load stl to visualizer");

        Self {
            visualizer,
            layout,
            schematic,
            click_location: None,
        }
    }

    pub fn animation_frame(&mut self) {
        self.visualizer.update();
        self.schematic.update();
    }

    pub fn mouse_move(&mut self, event: MouseEvent) {
        const DRAG_SENSITIVITY: f32 = 5.0;
        match self.click_location {
            Some(location) => {
                let new = (event.client_x(), event.client_y());
                let delta = (location.0 - new.0, location.1 - new.1);
                self.click_location = Some(new);

                let percentage_x =
                    (delta.0 as f32) / (self.visualizer.resolution.0 as f32) * DRAG_SENSITIVITY;
                let percentage_y =
                    (delta.1 as f32) / (self.visualizer.resolution.0 as f32) * DRAG_SENSITIVITY;

                self.visualizer.drag_view((percentage_x, -percentage_y));
            }
            None => {}
        }
    }
    pub fn mouse_down(&mut self, event: MouseEvent) {
        self.click_location = Some((event.client_x(), event.client_y()));
    }
    pub fn mouse_up(&mut self, _event: MouseEvent) {
        self.click_location = None;
    }

    pub fn key_event(&mut self, _event: KeyEvent) {
        //log(&format!("Key Event {:?}", event));
    }

    pub fn load_file(&mut self, file_contents: &[u8]) {
        log(&format!("Loading File"));
    }
}

struct Schematic {
    svg: SvgElement,
    path: TrackPath,
    dirty: bool,
}
impl Schematic {    
    pub fn new(svg: SvgElement) -> Self {
        Self {
            svg,
            path: TrackPath::new(),
            dirty: true,
        }
    }

    pub fn update(&mut self) {
        if self.dirty {
            self.draw().expect("Drawing Failed");
            self.dirty = false;
        }
    }

    fn clear(&mut self) -> Result<(), JsValue> {
        while let Some(elem) = self.svg.last_element_child() {
            self.svg.remove_child(&elem)?;
        }

        let background = create_svg_element("rect")?;
        background.set_attribute("width", "100%")?;
        background.set_attribute("height", "100%")?;
        background.set_attribute("x", "-50%")?;
        background.set_attribute("y", "-50%")?;
        background.set_attribute("fill", "white")?;
        self.svg.append_child(&background)?;

        Ok(())
    }

    fn draw(&mut self) -> Result<(), JsValue> {
        self.clear()?;

        let mut min_vec = Vec2::new(f32::INFINITY, f32::INFINITY);
        let mut max_vec = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

        for wheel in self.path.path.iter() {
            let radius = wheel.radius();
            let rad_vec = Vec2::new(radius, radius);
            
            Self::draw_wheel(&self.svg, radius, &wheel.position)?;
            
            min_vec = min_vec.min(wheel.position - rad_vec);
            max_vec = max_vec.max(wheel.position + rad_vec);
        }
        
        for segment in self.path.calc_segments() {
            Self::draw_line(&self.svg, &segment.line.start, &segment.line.end)?;
            Self::draw_arc(&self.svg, &segment.arc.center, segment.arc.radius, segment.arc.start_angle, segment.arc.end_angle)?;
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

    fn draw_wheel(svg: &SvgElement, radius: f32, position: &Vec2) -> Result<(), JsValue> {
        let idler_outline = create_svg_element("circle")?;

        idler_outline.set_attribute("cx", &format!("{}", position[0]))?;
        idler_outline.set_attribute("cy", &format!("{}", position[1]))?;
        idler_outline.set_attribute("r", &format!("{}", radius))?;
        idler_outline.set_attribute("fill", "none")?;
        idler_outline.set_attribute("stroke", "black")?;
        idler_outline.set_attribute("stroke-width", "0.2")?;

        svg.append_child(&idler_outline)?;
        Ok(())
    }
    
    fn draw_line(svg: &SvgElement, start_location: &Vec2, end_location: &Vec2) -> Result<(), JsValue> {
        let line = create_svg_element("line")?;
	
        line.set_attribute("x1", &format!("{}", start_location[0]))?;
        line.set_attribute("y1", &format!("{}", start_location[1]))?;
        line.set_attribute("x2", &format!("{}", end_location[0]))?;
        line.set_attribute("y2", &format!("{}", end_location[1]))?;
        
        line.set_attribute("stroke", "black")?;
        line.set_attribute("stroke-width", "0.5")?;
        
        svg.append_child(&line)?;
        Ok(())
    }

    fn draw_arc(svg: &SvgElement, center: &Vec2, radius: f32, start_angle: f32, end_angle: f32) -> Result<(), JsValue> {
        let start_pos = polar_to_cartesian(center, radius, start_angle);
        let end_pos = polar_to_cartesian(center, radius, end_angle);

        let large_arc = match (end_angle - start_angle) >= std::f32::consts::PI {
            true => "1",
            false => "0"
        };
        let arc_descriptor = format!("M {} {} A {} {} {} {} {} {} {}",
            start_pos[0],
            start_pos[1],
            
            radius,
            radius, 
            0, 
            large_arc, 
            0, 
            end_pos[0],
            end_pos[1]
        );
        
        let arc = create_svg_element("path")?;
        arc.set_attribute("d", &arc_descriptor)?;
        
        arc.set_attribute("fill", "none")?;
        arc.set_attribute("stroke", "black")?;
        arc.set_attribute("stroke-width", "0.5")?;
        
        svg.append_child(&arc)?;
        
        Ok(())
    }
}

fn polar_to_cartesian(center: &Vec2, radius: f32, angle: f32) -> Vec2 {
	Vec2::new(
		center[0] + f32::cos(angle) * radius,
		center[1] - f32::sin(angle) * radius, // SVG has zero at the top
	)
}

fn create_svg_element(elem: &str) -> Result<Element, JsValue> {
    let document = window().unwrap().document().unwrap();
    let svg_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), elem);
    svg_element
}
