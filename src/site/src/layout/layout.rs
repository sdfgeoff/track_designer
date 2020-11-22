/// Governs the location of all the major parts of the application.
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{window, HtmlCanvasElement, HtmlDivElement, HtmlElement, SvgGraphicsElement};

use super::css;
use super::header_bar;

pub enum ViewMode {
    SchematicMain,
    VisualizerMain,
}

pub struct Layout {
    pub pane_visualizer: HtmlDivElement,
    pub pane_schematic: HtmlDivElement,
    pub pane_parameters: HtmlDivElement,
    pub pane_header_bar: HtmlDivElement,

    pub visualizer_canvas: HtmlCanvasElement,
    pub schematic_svg: SvgGraphicsElement,
    pub header: header_bar::HeaderBar,
}

impl Layout {
    pub fn new(parent_area: &HtmlElement) -> Result<Self, JsValue> {
        css::inject_css(include_str!("packed-style.css"));

        let pane_visualizer = append_div(parent_area);
        let pane_schematic = append_div(parent_area);
        let pane_parameters = append_div(parent_area);
        let pane_header_bar = append_div(parent_area);

        pane_header_bar.set_class_name("header_bar");

        let window = window().unwrap();
        let document = window.document().unwrap();

        let visualizer_canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        pane_visualizer.append_child(&visualizer_canvas)?;
        visualizer_canvas.set_class_name("fill");

        let header = header_bar::HeaderBar::new().expect("Failed to make header bar");
        pane_header_bar.append_child(&header.container)?;

        let schematic_svg: SvgGraphicsElement = document
            .create_element_ns(Some("http://www.w3.org/2000/svg"), "svg")?
            .dyn_into()?;
        schematic_svg.class_list().add_1("fill")?;
        pane_schematic.append_child(&schematic_svg)?;

        let mut layout = Self {
            pane_visualizer,
            pane_schematic,
            pane_parameters,
            pane_header_bar,

            header,
            schematic_svg,
            visualizer_canvas,
        };
        layout.set_view(ViewMode::VisualizerMain);

        Ok(layout)
    }

    pub fn set_view(&mut self, view: ViewMode) {
        match view {
            ViewMode::SchematicMain => {
                self.set_primary(&self.pane_schematic);
                self.set_secondary(&self.pane_visualizer);
                self.set_tertiary(&self.pane_parameters);
            }
            ViewMode::VisualizerMain => {
                self.set_primary(&self.pane_visualizer);
                self.set_secondary(&self.pane_schematic);
                self.set_tertiary(&self.pane_parameters);
            }
        }
    }

    fn set_primary(&self, element: &HtmlDivElement) {
        element.set_class_name("pane_primary")
    }
    fn set_secondary(&self, element: &HtmlDivElement) {
        element.set_class_name("pane_secondary")
    }
    fn set_tertiary(&self, element: &HtmlDivElement) {
        element.set_class_name("pane_tertiary")
    }
}

fn append_div(parent_area: &HtmlElement) -> HtmlDivElement {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let new_div: HtmlDivElement = document
        .create_element("div")
        .expect("failed to create div")
        .dyn_into()
        .unwrap();
    parent_area
        .append_child(&new_div)
        .expect("Failed to append div");
    new_div
}
