use super::svg::{create_svg_element, polar_to_cartesian};
use wasm_bindgen::JsValue;
use web_sys::{Element, SvgElement};

use tracktool::track_path::TrackPathSegment;

#[derive(Debug)]
pub struct TrackPathSegmentDrawing {
    line: Element,
    arc: Element,
}

impl TrackPathSegmentDrawing {
    pub fn new(svg: &SvgElement) -> Result<Self, JsValue> {
        let line = create_svg_element("line")?;
        line.set_attribute("stroke", "black")?;
        line.set_attribute("stroke-width", "0.5")?;

        let arc = create_svg_element("path")?;
        arc.set_attribute("fill", "none")?;
        arc.set_attribute("stroke", "black")?;
        arc.set_attribute("stroke-width", "0.5")?;

        svg.append_child(&arc)?;
        svg.append_child(&line)?;

        Ok(Self { line, arc })
    }

    pub fn update_from_track_path_segment(
        &mut self,
        segment: &TrackPathSegment,
    ) -> Result<(), JsValue> {
        self.line
            .set_attribute("x1", &format!("{}", segment.line.start[0]))?;
        self.line
            .set_attribute("y1", &format!("{}", segment.line.start[1]))?;
        self.line
            .set_attribute("x2", &format!("{}", segment.line.end[0]))?;
        self.line
            .set_attribute("y2", &format!("{}", segment.line.end[1]))?;

        let radius = segment.arc.radius;
        let center = &segment.arc.center;

        let start_pos = polar_to_cartesian(center, radius, segment.arc.start_angle);
        let end_pos = polar_to_cartesian(center, radius, segment.arc.end_angle);

        let large_arc =
            match (segment.arc.end_angle - segment.arc.start_angle) >= std::f32::consts::PI {
                true => "1",
                false => "0",
            };
        let arc_descriptor = format!(
            "M {} {} A {} {} {} {} {} {} {}",
            start_pos[0], start_pos[1], radius, radius, 0, large_arc, 0, end_pos[0], end_pos[1]
        );

        self.arc.set_attribute("d", &arc_descriptor)?;

        Ok(())
    }

    pub fn delete(&self) -> Result<(), JsValue> {
        self.line
            .parent_node()
            .expect("Not in drawing!!!")
            .remove_child(&self.line)?;
        self.arc
            .parent_node()
            .expect("Not in drawing!!!")
            .remove_child(&self.arc)?;
        Ok(())
    }
}
