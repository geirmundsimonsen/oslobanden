use web_sys::{window, Window, HtmlElement, Document, Element};
use wasm_bindgen::JsCast;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

pub fn get_window() -> Window {
    window().expect("no global `window` exists")
}

pub fn get_document() -> Document {
    get_window().document().expect("should have a document on window")
}

pub fn get_body() -> HtmlElement {
    get_document().body().expect("document should have a body")
}

pub fn html_elem(tag: &crate::dom::Tag) -> HtmlElement {
    get_document().create_element(tag.as_html_tag()).unwrap().dyn_into::<HtmlElement>().unwrap()
}

pub fn append(parent: &Element, child: &Element) {
    parent.append_child(child).unwrap();
}