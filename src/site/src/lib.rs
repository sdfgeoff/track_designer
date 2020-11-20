use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{ArrayBuffer, Function};
use wasm_bindgen::prelude::{wasm_bindgen, Closure};
use wasm_bindgen::JsCast;
use web_sys::{
    window, Event, FileReader, HtmlElement,
    KeyEvent, MouseEvent,
};

mod app;
mod css;
mod header_bar;
mod layout;
mod visualizer3d;

use layout::Layout;

// Pull in the console.log function so we can debug things more easily
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// This struct will be accessible from JS as a JS object that can be
// created using `new Core()`
#[wasm_bindgen]
pub struct Core {
    app: Rc<RefCell<app::App>>,
}

#[wasm_bindgen]
impl Core {
    #[wasm_bindgen(constructor)]
    pub fn new(app_area: HtmlElement) -> Self {
        log(&format!(
            "Track Designer WASM Started for area {}",
            app_area.id()
        ));

        css::inject_css(include_str!("packed-style.css"));

        app_area.set_class_name("loaded");

        let layout = Layout::new(&app_area).expect("Failed to create layout");
        let app = Rc::new(RefCell::new(app::App::new(layout)));

        Self { app }
    }

    #[wasm_bindgen]
    pub fn start(&mut self) {
        log("App Started");
        let window = window().unwrap();

        {
            // Animation Frame
            let callback = Rc::new(RefCell::new(None));

            let anim_app = self.app.clone();
            let anim_window = window.clone();
            let anim_callback = callback.clone();

            *callback.borrow_mut() = Some(Closure::wrap(Box::new(move || {
                anim_app.borrow_mut().animation_frame();
                // Schedule ourself for another requestAnimationFrame callback.
                anim_window
                    .request_animation_frame(make_callback(
                        anim_callback.borrow().as_ref().unwrap(),
                    ))
                    .unwrap();
            }) as Box<dyn FnMut()>));
            window
                .request_animation_frame(make_callback(callback.borrow().as_ref().unwrap()))
                .unwrap();
        }

        let canvas = &self.app.borrow().layout.visualizer_canvas;

        {
            // Mouse events
            let anim_app1 = self.app.clone();
            let anim_app2 = self.app.clone();
            let anim_app3 = self.app.clone();

            let mouse_move = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app1.borrow_mut().mouse_move(event);
            }) as Box<dyn FnMut(_)>);
            let mouse_up = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app2.borrow_mut().mouse_up(event);
            }) as Box<dyn FnMut(_)>);
            let mouse_down = Closure::wrap(Box::new(move |event: MouseEvent| {
                anim_app3.borrow_mut().mouse_down(event);
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
            // keyboard events
            canvas.set_tab_index(1); // Canvas elements ignore key events unless they have a tab index
            let anim_app = self.app.clone();

            let callback = Closure::wrap(Box::new(move |event: KeyEvent| {
                let e: Event = event.clone().dyn_into().unwrap();
                e.stop_propagation();
                e.prevent_default();

                anim_app.borrow_mut().key_event(event);
            }) as Box<dyn FnMut(_)>);

            canvas
                .add_event_listener_with_callback("keydown", callback.as_ref().unchecked_ref())
                .unwrap();
            canvas
                .add_event_listener_with_callback("keyup", callback.as_ref().unchecked_ref())
                .unwrap();

            callback.forget();
        }

        {
            // Header Buttons
            let anim_app = self.app.clone();

            let open_button_click = Closure::wrap(Box::new(move || {
                log(&format!("Open File Selected"));

                let files = anim_app.borrow().layout.header.open_button.files();
                if let Some(files) = files {
                    if let Some(file) = files.get(0) {
                        let reader_app = anim_app.clone();
                        let reader = FileReader::new().expect("Failed to create file reader");
                        let reader_reader = reader.clone();

                        let open_file_callback = Closure::wrap(Box::new(move || {
                            let data_buffer: ArrayBuffer = reader_reader
                                .result()
                                .expect("Failed to get file contents")
                                .dyn_into()
                                .unwrap();
                            //~ let data: Vec<u8> = (&data_buffer.try_into()).unwrap();
                            //~ reader_app.borrow().load_file(
                            //~ &data
                            //~ );

                            // TODO: extract the data and pipe into app
                            //anim_app.load_file(data);
                        })
                            as Box<dyn FnMut()>);

                        reader
                            .add_event_listener_with_callback(
                                "load",
                                open_file_callback.as_ref().unchecked_ref(),
                            )
                            .unwrap();
                        open_file_callback.forget();

                        reader.read_as_array_buffer(&file.dyn_into().unwrap());
                    }
                }
            }) as Box<dyn FnMut()>);

            self.app
                .borrow()
                .layout
                .header
                .open_button
                .add_event_listener_with_callback(
                    "change",
                    open_button_click.as_ref().unchecked_ref(),
                )
                .unwrap();
            open_button_click.forget();

            let download_button_click = Closure::wrap(Box::new(move || {
                log(&format!("Download Button Clicked"));
            }) as Box<dyn FnMut()>);
            self.app
                .borrow()
                .layout
                .header
                .download_button
                .add_event_listener_with_callback(
                    "click",
                    download_button_click.as_ref().unchecked_ref(),
                )
                .unwrap();
            download_button_click.forget();
        }
    }
}

fn make_callback(closure: &Closure<dyn FnMut()>) -> &Function {
    closure.as_ref().unchecked_ref()
}
