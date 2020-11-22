use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{Element, MouseEvent, SvgElement, SvgGraphicsElement};

use glam::Vec2;

use tracktool::track_path::{WheelDescription};

mod handle;
mod svg;
mod track_path_segment;
mod wheel;

pub use track_path_segment::TrackPathSegmentDrawing;
pub use wheel::WheelDrawing;

pub struct Schematic {
    //~ pub svg: SvgGraphicsElement,
    pub background: Element,
    pub selected_element: Option<Element>,
}

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Schematic {
    pub fn new(svg: SvgGraphicsElement) -> Result<Self, JsValue> {
        let background = svg::create_svg_element("rect")?;
        background.set_attribute("width", "100%")?;
        background.set_attribute("height", "100%")?;
        background.set_attribute("x", "-50%")?;
        background.set_attribute("y", "-50%")?;
        background.set_attribute("fill", "white")?;
        svg.append_child(&background)?;

        Ok(Self {
            background,
            //~ idler_drawings: vec!(),
            //~ track_path_segments: vec!(),
            selected_element: None,
        })
    }

    pub fn update_bounding_box(
        &mut self,
        svg: &SvgElement,
        wheels: &Vec<WheelDescription>,
    ) -> Result<(), JsValue> {
        let mut top_left = Vec2::new(0.0, 0.0);
        let mut dimensions = Vec2::new(100.0, 100.0);

        if !wheels.is_empty() {
            // Ensure bounding box of image
            let mut min_vec = Vec2::new(f32::INFINITY, f32::INFINITY);
            let mut max_vec = Vec2::new(f32::NEG_INFINITY, f32::NEG_INFINITY);

            for wheel in wheels.iter() {
                let radius = wheel.radius();
                let rad_vec = Vec2::new(radius, radius);

                min_vec = min_vec.min(wheel.position - rad_vec);
                max_vec = max_vec.max(wheel.position + rad_vec);
            }

            dimensions = max_vec - min_vec;
            top_left = min_vec;
        }
        svg.set_attribute(
            "viewBox",
            &format!(
                "{} {} {} {}",
                top_left[0], top_left[1], dimensions[0], dimensions[1],
            ),
        )?;

        self.background
            .set_attribute("width", &format!("{}", dimensions[0]))?;
        self.background
            .set_attribute("height", &format!("{}", dimensions[1]))?;
        self.background
            .set_attribute("x", &format!("{}", top_left[0]))?;
        self.background
            .set_attribute("y", &format!("{}", top_left[1]))?;

        Ok(())
    }

    pub fn mouse_down(&mut self, evt: MouseEvent) {
        if let Some(target) = evt.target() {
            self.selected_element = Some(target.dyn_into().expect("Not Right Element"));
        }
    }
    pub fn mouse_up(&mut self, _evt: MouseEvent) {
        self.selected_element = None;
    }
    pub fn mouse_move(&mut self, svg: &SvgGraphicsElement, evt: MouseEvent) {
        if let Some(elem) = &self.selected_element {
            //let graphics_elem: SvgGraphicsElement = self.svg.dyn_into().expect("Err");
            let ctm = svg.get_screen_ctm().unwrap();

            let x = (evt.client_x() as f32 - ctm.e()) / ctm.a();
            let y = (evt.client_y() as f32 - ctm.f()) / ctm.d();
            let mouse_position = Vec2::new(x, y);

            handle::modify_element(elem, mouse_position);

            //~ log(&format!("{} {} {:?}", x, y, elem));
        }
    }
}

pub fn get_drawing_id_from_element(
    drawings: &Vec<WheelDrawing>,
    elem: &Element,
) -> Option<usize> {
    for (id, drawing) in drawings.iter().enumerate() {
        if drawing.contains_element(elem) {
            return Some(id);
        }
    }
    return None;
}
