use web_sys::Element;

use glam::Vec2;

pub enum Action {
    ChangeCirclePosition,
    ChangeCircleRadius,
}

impl Action {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "change_circle_radius" => Some(Self::ChangeCircleRadius),
            "change_circle_position" => Some(Self::ChangeCirclePosition),
            _ => None,
        }
    }
    fn to_str(&self) -> &str {
        match self {
            Self::ChangeCircleRadius => "change_circle_radius",
            Self::ChangeCirclePosition => "change_circle_position",
        }
    }
}

pub fn set_drag_action(element: &Element, action: Action) {
    element
        .set_attribute("drag_action", action.to_str())
        .unwrap()
}

pub fn get_drag_action(element: &Element) -> Option<Action> {
    Action::from_str(&element.get_attribute("drag_action")?)
}

pub fn modify_element(elem: &Element, mouse_position: Vec2) {
    if let Some(action) = get_drag_action(elem) {
        match action {
            Action::ChangeCirclePosition => {
                elem.set_attribute("cx", &format!("{}", mouse_position[0]))
                    .expect("Failed to Move");
                elem.set_attribute("cy", &format!("{}", mouse_position[1]))
                    .expect("Failed to Move");
            }
            Action::ChangeCircleRadius => {
                let center = Vec2::new(
                    elem.get_attribute("cx").unwrap().parse::<f32>().unwrap(),
                    elem.get_attribute("cy").unwrap().parse::<f32>().unwrap(),
                );
                let radius = center.distance(mouse_position);
                elem.set_attribute("r", &format!("{}", radius))
                    .expect("Failed to Change Radius");
            }
        }
    }
}
