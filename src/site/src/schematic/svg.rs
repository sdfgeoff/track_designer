use web_sys::{window, Element, KeyEvent, MouseEvent, SvgElement};
use glam::Vec2;
use wasm_bindgen::JsValue;

pub fn create_svg_element(elem: &str) -> Result<Element, JsValue> {
    let document = window().unwrap().document().unwrap();
    let svg_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), elem);
    svg_element
}

pub fn polar_to_cartesian(center: &Vec2, radius: f32, angle: f32) -> Vec2 {
	Vec2::new(
		center[0] + f32::cos(angle) * radius,
		center[1] - f32::sin(angle) * radius, // SVG has zero at the top
	)
}
