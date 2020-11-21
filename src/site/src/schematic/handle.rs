use super::svg::create_svg_element;
use wasm_bindgen::JsValue;
use web_sys::Element;


use glam::Vec2;

pub fn create_handle() -> Result<Element, JsValue> {
    let elem = create_svg_element("circle")?;
    
    elem.set_attribute("fill", "blue")?;
    elem.set_attribute("stroke", "none")?;
    elem.set_attribute("r", "3%")?;
    
    Ok(elem)
}


pub fn get_position(elem: &Element) -> Result<glam::Vec2, JsValue> {
    Ok(Vec2::new(
        elem.get_attribute("cx").unwrap().parse::<f32>().unwrap(),
        elem.get_attribute("cy").unwrap().parse::<f32>().unwrap()
    ))
}
