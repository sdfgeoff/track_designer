use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;
use web_sys::{window, HtmlElement};

use std::collections::VecDeque;

use super::events::Event;
use super::layout::Layout;
use super::schematic;
use super::state::State;
use super::visualizer3d;

use glam::Vec2;
use tracktool::track_path::{calc_segments, TrackPathSegment, WheelDescription};

use meshtools::stl::generate_binary_stl;
use track_outer_surfaces::TrackOuterSurfaceDescription;
use tracktool::{track, track_surface};

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct App {
    pub state: State,
    pub events: VecDeque<Event>,
}

fn get_time_ms() -> f64 {
    window().unwrap().performance().unwrap().now()
}

const FRAME_TIME_MAX_MS: f64 = 10.0; // 6ms for the browser to paint

impl App {
    pub fn new(app_area: HtmlElement) -> Self {
        let layout = Layout::new(&app_area).expect("Failed to create layout");

        let mut visualizer = visualizer3d::Visualizer3d::new(layout.visualizer_canvas.clone())
            .expect("Renderer Failed to Init");

        let schem = schematic::Schematic::new(layout.schematic_svg.clone())
            .expect("Failed to create schematic");

        {
            //Temporary Stuff
            let complete_track = track::generate_track(
                track::TrackDescription {
                    outer_surface: TrackOuterSurfaceDescription::Chunky,
                    inner_surface: track_surface::TrackInnerSurfaceDescription::Simple,

                    belt_thickness_mm: 2.0,
                    belt_width_mm: 20.0,

                    inner_segment_repeats: 5,
                    outer_segment_repeats: 10,
                },
                track::TrackShapeDescription::Loop(100.0),
            );

            let stl_data = generate_binary_stl(&complete_track);

            visualizer
                .add_stl(&stl_data)
                .expect("failed to load stl to visualizer");
        }

        visualizer.update();

        let mut new = Self {
            state: State {
                wheel_descriptions: vec![],
                track_path_segments: vec![],
                wheel_drawings: vec![],
                track_path_segment_drawings: vec![],
                layout: layout,
                schematic: schem,
                visualizer: visualizer,
            },
            events: VecDeque::new(),
        };

        new.events.push_front(Event::WheelInsert(WheelDescription {
            position: Vec2::new(-20.0, 0.0),
            circumference: 30.0,
        }));
        new.events.push_front(Event::WheelInsert(WheelDescription {
            position: Vec2::new(20.0, 0.0),
            circumference: 30.0,
        }));

        new
    }

    //~ pub fn load_file(&mut self, file_contents: &[u8]) {
    //~ log(&format!("Loading File"));
    //~ }
    pub fn tick(&mut self) -> Result<(), JsValue> {
        // Run through all events

        let start_time = get_time_ms();

        while let Some(event) = self.events.pop_back() {
            log(&format!("handling event {:?}", &event));

            match event {
                Event::AnimationFrame => {
                    //self.state.visualizer.update();
                }

                Event::WheelInsert(wheel_description) => {
                    let mut new_wheel_drawing =
                        schematic::WheelDrawing::new(&self.state.layout.schematic_svg)?;
                    new_wheel_drawing.update_from_wheel_description(&wheel_description)?;

                    self.state.wheel_descriptions.push(wheel_description);

                    self.events
                        .push_front(Event::WheelDrawingInsert(new_wheel_drawing));
                    self.events
                        .push_front(Event::TrackPathSegmentInsert(TrackPathSegment::new()));
                }
                Event::WheelDrawingInsert(wheel_drawing) => {
                    self.state.wheel_drawings.push(wheel_drawing);
                    self.state.schematic.update_bounding_box(
                        &self.state.layout.schematic_svg,
                        &self.state.wheel_descriptions,
                    )?;
                }

                Event::TrackPathSegmentInsert(segment) => {
                    let mut new_segment_drawing =
                        schematic::TrackPathSegmentDrawing::new(&self.state.layout.schematic_svg)?;
                    new_segment_drawing.update_from_track_path_segment(&segment)?;

                    self.events
                        .push_front(Event::TrackPathSegmentDrawingInsert(new_segment_drawing));
                }
                Event::TrackPathSegmentDrawingInsert(segment_drawing) => {
                    self.state.track_path_segment_drawings.push(segment_drawing);
                    self.state.schematic.update_bounding_box(
                        &self.state.layout.schematic_svg,
                        &self.state.wheel_descriptions,
                    )?;
                }

                Event::SchematicMouseDown(event) => {
                    self.state.schematic.mouse_down(event);
                }
                Event::SchematicMouseUp(event) => {
                    self.state.schematic.mouse_up(event);
                    self.state.schematic.update_bounding_box(
                        &self.state.layout.schematic_svg,
                        &self.state.wheel_descriptions,
                    )?;
                }
                Event::SchematicMouseMove(event) => {
                    self.state
                        .schematic
                        .mouse_move(&self.state.layout.schematic_svg, event);

                    if let Some(elem) = &self.state.schematic.selected_element {
                        if let Some(id) =
                            schematic::get_drawing_id_from_element(&self.state.wheel_drawings, elem)
                        {
                            let drawing = self.state.wheel_drawings.get(id).expect("Invalid Id!!");
                            let wheel_desc = self
                                .state
                                .wheel_descriptions
                                .get_mut(id)
                                .expect("Unequal wheels and drawings");

                            wheel_desc.position = drawing.get_center_handle_position()?;
                            wheel_desc.circumference =
                                drawing.get_radius() * 2.0 * std::f32::consts::PI;

                            self.events.push_front(Event::WheelChanged(id));
                        }
                    }
                }

                Event::WheelChanged(id) => {
                    assert!(self.state.wheel_drawings.len() == self.state.wheel_descriptions.len());
                    let drawing = self
                        .state
                        .wheel_drawings
                        .get_mut(id)
                        .expect("Invalid Wheel Changed ID");
                    let description = self
                        .state
                        .wheel_descriptions
                        .get(id)
                        .expect("Invalid Wheel Changed ID");
                    drawing.update_from_wheel_description(&description)?;

                    //self.state.schematic.update_bounding_box(&self.state.layout.schematic_svg, &self.state.wheel_descriptions)?;

                    // TODO: Only update needed segments
                    self.state.track_path_segments = calc_segments(&self.state.wheel_descriptions);
                    for i in 0..self.state.track_path_segments.len() {
                        self.events.push_front(Event::TrackPathSegmentChanged(i));
                    }
                }

                Event::TrackPathSegmentChanged(id) => {
                    assert!(
                        self.state.track_path_segments.len()
                            == self.state.track_path_segment_drawings.len()
                    );
                    let drawing = self
                        .state
                        .track_path_segment_drawings
                        .get_mut(id)
                        .expect("Invalid Wheel Changed ID");
                    let segment = self
                        .state
                        .track_path_segments
                        .get(id)
                        .expect("Invalid Wheel Changed ID");
                    drawing.update_from_track_path_segment(&segment)?;
                }

                Event::VisualizerMouseDown(event) => {
                    self.state.visualizer.mouse_down(event);
                }
                Event::VisualizerMouseUp(event) => {
                    self.state.visualizer.mouse_up(event);
                }
                Event::VisualizerMouseMove(event) => {
                    self.state.visualizer.mouse_move(event);
                }
            }

            if get_time_ms() - start_time > FRAME_TIME_MAX_MS {
                log(&format!(
                    "{} tasks remaining at end of frame",
                    self.events.len()
                ));
                break;
            }
        }
        Ok(())
    }
}
