//! Provides a GUI interface to the tracktool library. This takes the 
//! form of a fully-rust WASM web-app

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{ArrayBuffer, Function};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{window, Event, FileReader, HtmlElement, KeyEvent, MouseEvent};

pub mod app;
pub mod events;
pub mod layout;
pub mod schematic;
pub mod state;
pub mod visualizer3d;

use console_error_panic_hook;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

/// Start the program running
#[wasm_bindgen]
pub fn start(app_area: HtmlElement) {
    log(&format!(
        "Track Designer WASM Started for area {}",
        app_area.id()
    ));

    app_area.set_class_name("loaded");

    console_error_panic_hook::set_once();

    let app = Rc::new(RefCell::new(app::App::new(app_area)));

    log("App Started");
    let window = window().unwrap();

    {
        // Animation Frame
        let callback = Rc::new(RefCell::new(None));

        let anim_app = app.clone();
        let anim_window = window.clone();
        let anim_callback = callback.clone();

        *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            anim_app
                .borrow_mut()
                .events
                .push_front(events::Event::AnimationFrame);
            anim_app.borrow_mut().tick().expect("Update Failed");

            // Schedule ourself for another requestAnimationFrame callback.

            anim_window
                .request_animation_frame(make_callback(anim_callback.borrow().as_ref().unwrap()))
                .unwrap();
        }) as Box<dyn FnMut()>));
        window
            .request_animation_frame(make_callback(callback.borrow().as_ref().unwrap()))
            .unwrap();
    }

    // TODO: Find a better place/way to do these bindings
    {
        // 3D Visualizer Mouse events
        let canvas = &app.borrow().state.layout.visualizer_canvas;

        let anim_app1 = app.clone();
        let anim_app2 = app.clone();
        let anim_app3 = app.clone();

        let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            anim_app1
                .borrow_mut()
                .events
                .push_front(events::Event::VisualizerMouseMove(event));
        }) as Box<dyn FnMut(_)>);
        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            anim_app2
                .borrow_mut()
                .events
                .push_front(events::Event::VisualizerMouseUp(event));
        }) as Box<dyn FnMut(_)>);
        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            anim_app3
                .borrow_mut()
                .events
                .push_front(events::Event::VisualizerMouseDown(event));
        }) as Box<dyn FnMut(_)>);

        let mouse_move_ref = mouse_move.as_ref().unchecked_ref();
        let mouse_up_ref = mouse_up.as_ref().unchecked_ref();
        let mouse_down_ref = mouse_down.as_ref().unchecked_ref();

        canvas
            .add_event_listener_with_callback("mousedown", mouse_down_ref)
            .unwrap();
        canvas
            .add_event_listener_with_callback("mouseup", mouse_up_ref)
            .unwrap();
        canvas
            .add_event_listener_with_callback("mousemove", mouse_move_ref)
            .unwrap();
        canvas
            .add_event_listener_with_callback("mouseleave", mouse_up_ref)
            .unwrap();

        mouse_move.forget();
        mouse_up.forget();
        mouse_down.forget();
    }
    {
        // Schematic Mouse events
        let schematic_svg = &app.borrow().state.layout.schematic_svg;

        let anim_app1 = app.clone();
        let anim_app2 = app.clone();
        let anim_app3 = app.clone();

        let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
            anim_app1
                .borrow_mut()
                .events
                .push_front(events::Event::SchematicMouseMove(event));
        }) as Box<dyn FnMut(_)>);
        let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
            anim_app2
                .borrow_mut()
                .events
                .push_front(events::Event::SchematicMouseUp(event));
        }) as Box<dyn FnMut(_)>);
        let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
            anim_app3
                .borrow_mut()
                .events
                .push_front(events::Event::SchematicMouseDown(event));
        }) as Box<dyn FnMut(_)>);

        let mouse_move_ref = mouse_move.as_ref().unchecked_ref();
        let mouse_up_ref = mouse_up.as_ref().unchecked_ref();
        let mouse_down_ref = mouse_down.as_ref().unchecked_ref();

        schematic_svg
            .add_event_listener_with_callback("mousedown", mouse_down_ref)
            .unwrap();
        schematic_svg
            .add_event_listener_with_callback("mouseup", mouse_up_ref)
            .unwrap();
        schematic_svg
            .add_event_listener_with_callback("mousemove", mouse_move_ref)
            .unwrap();
        schematic_svg
            .add_event_listener_with_callback("mouseleave", mouse_up_ref)
            .unwrap();

        mouse_move.forget();
        mouse_up.forget();
        mouse_down.forget();
    }

    //~ {
    //~ // Header Buttons
    //~ let anim_app = self.app.clone();

    //~ let open_button_click = Closure::wrap(Box::new(move || {
    //~ log(&format!("Open File Selected"));

    //~ let files = anim_app.borrow().layout.header.open_button.files();
    //~ if let Some(files) = files {
    //~ if let Some(file) = files.get(0) {
    //~ let reader_app = anim_app.clone();
    //~ let reader = FileReader::new().expect("Failed to create file reader");
    //~ let reader_reader = reader.clone();

    //~ let open_file_callback = Closure::wrap(Box::new(move || {
    //~ let data_buffer: ArrayBuffer = reader_reader
    //~ .result()
    //~ .expect("Failed to get file contents")
    //~ .dyn_into()
    //~ .unwrap();
    //~ let data: Vec<u8> = (&data_buffer.try_into()).unwrap();
    //~ reader_app.borrow().load_file(
    //~ &data
    //~ );

    //~ // TODO: extract the data and pipe into app
    //~ //anim_app.load_file(data);
    //~ })
    //~ as Box<dyn FnMut()>);

    //~ reader
    //~ .add_event_listener_with_callback(
    //~ "load",
    //~ open_file_callback.as_ref().unchecked_ref(),
    //~ )
    //~ .unwrap();
    //~ open_file_callback.forget();

    //~ reader.read_as_array_buffer(&file.dyn_into().unwrap());
    //~ }
    //~ }
    //~ }) as Box<dyn FnMut()>);

    //~ app
    //~ .borrow()
    //~ .layout
    //~ .header
    //~ .open_button
    //~ .add_event_listener_with_callback(
    //~ "change",
    //~ open_button_click.as_ref().unchecked_ref(),
    //~ )
    //~ .unwrap();
    //~ open_button_click.forget();

    //~ let download_button_click = Closure::wrap(Box::new(move || {
    //~ log(&format!("Download Button Clicked"));
    //~ }) as Box<dyn FnMut()>);

    //~ app
    //~ .borrow()
    //~ .layout
    //~ .header
    //~ .download_button
    //~ .add_event_listener_with_callback(
    //~ "click",
    //~ download_button_click.as_ref().unchecked_ref(),
    //~ )
    //~ .unwrap();
    //~ download_button_click.forget();
    //~ }
}

fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    closure.as_ref().unchecked_ref()
}
