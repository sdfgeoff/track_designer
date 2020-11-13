use web_sys::window;

pub fn inject_css(css: &str) {
    let window = window().unwrap();
    let document = window.document().unwrap();

    let head = document.head().unwrap();

    let style = document.create_element("style").unwrap();
    style.set_inner_html(css);
    head.append_child(&style);
}
