//! Some helper functions for working with SVG's
use glam::Vec2;
use wasm_bindgen::JsValue;
use web_sys::{window, Element};

/// Creates an element with the specified type. Avoids having to remember
/// the whole "create_element_ns" nonsense
pub fn create_svg_element(elem: &str) -> Result<Element, JsValue> {
    let document = window().unwrap().document().unwrap();
    let svg_element = document.create_element_ns(Some("http://www.w3.org/2000/svg"), elem);
    svg_element
}

/// Converts Polar to Cartesian coordinates. Yeah, this iis a math library
/// sort of thing, but I couldn't find it in glam.
pub fn polar_to_cartesian(center: &Vec2, radius: f32, angle: f32) -> Vec2 {
    Vec2::new(
        center[0] + f32::cos(angle) * radius,
        center[1] - f32::sin(angle) * radius, // SVG has zero at the top
    )
}
