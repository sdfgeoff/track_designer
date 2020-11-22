use wasm_bindgen::JsCast;
use web_sys::{window, HtmlButtonElement, HtmlInputElement, HtmlLiElement, HtmlUListElement};

pub struct HeaderBar {
    pub logo: HtmlLiElement,
    pub open_button: HtmlInputElement,
    pub download_button: HtmlButtonElement,
    pub container: HtmlUListElement,
}

impl HeaderBar {
    pub fn new() -> Result<Self, wasm_bindgen::JsValue> {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let header_items: HtmlUListElement = document.create_element("ul")?.dyn_into()?;
        header_items.set_class_name("header_list");

        let logo: HtmlLiElement = document.create_element("li")?.dyn_into()?;
        logo.set_inner_text("Track Designer");
        logo.set_class_name("logo");
        header_items.append_child(&logo)?;

        let open_button: HtmlInputElement = document.create_element("input")?.dyn_into()?;
        open_button.set_inner_text("Open File");
        open_button.set_type("file");
        let item: HtmlLiElement = document.create_element("li")?.dyn_into()?;
        item.append_child(&open_button)?;
        header_items.append_child(&item)?;

        let download_button: HtmlButtonElement = document.create_element("button")?.dyn_into()?;
        download_button.set_inner_text("Download Config");
        let download_item: HtmlLiElement = document.create_element("li")?.dyn_into()?;
        download_item.append_child(&download_button)?;
        header_items.append_child(&download_item)?;

        Ok(Self {
            logo,
            open_button,
            download_button,
            container: header_items,
        })
    }
}
