//! Handles are SVG elements that are used to manipulate the state.
//! The user clicks on them, and when he drags, something happens.
//! To do this, you use the function `set_drag_action`
//! Then, when the mouse is dragged, `modify_element` can be called with
//! an element and a mouse position.
//! Internally, this works by adding the attribute `drag_action` to the
//! svg element.

use glam::Vec2;
use web_sys::Element;

/// The different actions that clicking/dragging on a SVG element can have.
/// Primarily this prevents typos
pub enum Action {
    /// Move a circle so the center is at the mosue position.
    /// Circles use cx and cy, so it has to differ from
    /// rectangles or lines.
    ChangeCirclePosition,

    /// Changes circle radius to reach to the cursor
    ChangeCircleRadius,
}

impl Action {
    /// Convert from a str so that we can figure out what an element does
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "change_circle_radius" => Some(Self::ChangeCircleRadius),
            "change_circle_position" => Some(Self::ChangeCirclePosition),
            _ => None,
        }
    }

    /// Convert to a str which can be stored in an HTML element
    fn to_str(&self) -> &str {
        match self {
            Self::ChangeCircleRadius => "change_circle_radius",
            Self::ChangeCirclePosition => "change_circle_position",
        }
    }
}

/// Give an element a drag aaction.
pub fn set_drag_action(element: &Element, action: Action) {
    element
        .set_attribute("drag_action", action.to_str())
        .unwrap()
}

/// Retrieve a drag action from an element
pub fn get_drag_action(element: &Element) -> Option<Action> {
    Action::from_str(&element.get_attribute("drag_action")?)
}

/// Perform the action associated the specified element.
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
