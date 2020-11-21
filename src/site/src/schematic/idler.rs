use super::svg::create_svg_element;
use super::handle::{create_handle, get_position};
use std::rc::Rc;

use web_sys::{Element, SvgElement};
use wasm_bindgen::JsValue;


use tracktool::track_path::{WheelDescription};

pub struct IdlerWheelDrawing {
    main_circle: Element,
    center_handle: Element,
    radius_handle: Element,
}

impl IdlerWheelDrawing {
    pub fn new(svg: &SvgElement) -> Result<Self, JsValue> {
        let main_circle = create_svg_element("circle")?;

        main_circle.set_attribute("fill", "none")?;
        main_circle.set_attribute("stroke", "black")?;
        main_circle.set_attribute("stroke-width", "0.2")?;
        
        let center_handle = create_handle()?;
        let radius_handle = create_handle()?;

        svg.append_child(&main_circle)?;
        svg.append_child(&center_handle)?;
        svg.append_child(&radius_handle)?;
        
        Ok(Self {
            main_circle,
            center_handle,
            radius_handle
        })
    }
    
    pub fn update_from_wheel_description(&mut self, wheel: &WheelDescription) -> Result<(), JsValue> {
        let radius = wheel.radius();
        self.main_circle.set_attribute("cx", &format!("{}", wheel.position[0]))?;
        self.main_circle.set_attribute("cy", &format!("{}", wheel.position[1]))?;
        self.main_circle.set_attribute("r", &format!("{}", radius))?;
        
        if !self.center_handle.has_attribute("cx") {
            self.center_handle.set_attribute("cx", &format!("{}", wheel.position[0]))?;
            self.center_handle.set_attribute("cy", &format!("{}", wheel.position[1]))?;
            
            self.radius_handle.set_attribute("cx", &format!("{}", wheel.position[0]))?;
            self.radius_handle.set_attribute("cy", &format!("{}", wheel.position[1] + radius))?;
        }
        Ok(())
    }
    
    pub fn delete(&self) -> Result<(), JsValue> {
        self.main_circle.parent_node().expect("Not in drawing!!!").remove_child(&self.main_circle)?;
        self.center_handle.parent_node().expect("Not in drawing!!!").remove_child(&self.center_handle)?;
        self.radius_handle.parent_node().expect("Not in drawing!!!").remove_child(&self.radius_handle)?;
        Ok(())
    }
    
    pub fn get_center_handle_position(&self) -> Result<glam::Vec2, JsValue> {
        get_position(&self.center_handle)
    }
    
    pub fn get_radius_handle_position(&self) -> Result<glam::Vec2, JsValue> {
        get_position(&self.radius_handle)
    }
}
