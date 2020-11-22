use super::handle::{set_drag_action, Action};
use super::svg::create_svg_element;

use wasm_bindgen::JsValue;
use web_sys::{Element, SvgElement};

use glam::Vec2;

const RADIUS_HANDLE_SIZE: f32 = 0.25;
const RADIUS_HANDLE_SCALE_FACTOR: f32 = 1.0 - RADIUS_HANDLE_SIZE / 2.0;

use tracktool::track_path::WheelDescription;

#[derive(Debug)]
pub struct WheelDrawing {
    main_circle: Element,
    center_handle: Element,
    radius_handle: Element,
}

impl WheelDrawing {
    pub fn new(svg: &SvgElement) -> Result<Self, JsValue> {
        let main_circle = create_svg_element("circle")?;

        main_circle.set_attribute("fill", "none")?;
        main_circle.set_attribute("stroke", "black")?;
        main_circle.set_attribute("stroke-width", "2%")?;

        let radius_handle = create_svg_element("circle")?;
        radius_handle.set_attribute("fill", "none")?;
        radius_handle.set_attribute("stroke", "blue")?;
        radius_handle.set_attribute("stroke-width", "1.0")?;
        radius_handle.set_attribute("drag_action", "change_circle_radius")?;

        let center_handle = create_svg_element("circle")?;
        center_handle.set_attribute("fill", "blue")?;
        center_handle.set_attribute("stroke", "none")?;
        center_handle.set_attribute("r", "3%")?;

        set_drag_action(&center_handle, Action::ChangeCirclePosition);
        set_drag_action(&radius_handle, Action::ChangeCircleRadius);

        svg.append_child(&main_circle)?;
        svg.append_child(&center_handle)?;
        svg.append_child(&radius_handle)?;

        Ok(Self {
            main_circle,
            center_handle,
            radius_handle,
        })
    }

    pub fn contains_element(&self, elem: &Element) -> bool {
        elem == &self.main_circle || elem == &self.center_handle || elem == &self.radius_handle
    }

    pub fn update_from_wheel_description(
        &mut self,
        wheel: &WheelDescription,
    ) -> Result<(), JsValue> {
        let radius = wheel.radius();
        self.main_circle
            .set_attribute("cx", &format!("{}", wheel.position[0]))?;
        self.main_circle
            .set_attribute("cy", &format!("{}", wheel.position[1]))?;
        self.main_circle
            .set_attribute("r", &format!("{}", radius))?;

        self.center_handle
            .set_attribute("cx", &format!("{}", wheel.position[0]))?;
        self.center_handle
            .set_attribute("cy", &format!("{}", wheel.position[1]))?;

        self.radius_handle
            .set_attribute("cx", &format!("{}", wheel.position[0]))?;
        self.radius_handle
            .set_attribute("cy", &format!("{}", wheel.position[1]))?;
        self.radius_handle
            .set_attribute("r", &format!("{}", radius * RADIUS_HANDLE_SCALE_FACTOR))?;
        self.radius_handle
            .set_attribute("stroke-width", &format!("{}", radius * RADIUS_HANDLE_SIZE))?;

        Ok(())
    }

    pub fn delete(&self) -> Result<(), JsValue> {
        self.main_circle
            .parent_node()
            .expect("Not in drawing!!!")
            .remove_child(&self.main_circle)?;
        self.center_handle
            .parent_node()
            .expect("Not in drawing!!!")
            .remove_child(&self.center_handle)?;
        Ok(())
    }

    pub fn get_center_handle_position(&self) -> Result<glam::Vec2, JsValue> {
        Ok(Vec2::new(
            self.center_handle
                .get_attribute("cx")
                .unwrap()
                .parse::<f32>()
                .unwrap(),
            self.center_handle
                .get_attribute("cy")
                .unwrap()
                .parse::<f32>()
                .unwrap(),
        ))
    }

    pub fn get_radius(&self) -> f32 {
        self.radius_handle
            .get_attribute("r")
            .unwrap()
            .parse::<f32>()
            .unwrap()
            / RADIUS_HANDLE_SCALE_FACTOR
    }
}
